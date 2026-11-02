//! Slop mode (this fork): the superslop house style for website builds, and the Slop
//! Gate check that judges it. See `docs/slop/SPEC.md` ("Modes", "The superslop output
//! style", "Slop Gate").
//!
//! - [`instructions`] is appended to a website build's brief in Slop mode: the vendored
//!   superslop suite's UI, copywriting, layout/mobile and human modules, verbatim from
//!   `third-party/superslop/` (pinned in `docs/slop/PROVENANCE.md`).
//! - [`SlopGateCheck`] replaces the likeness check for website targets in Slop mode. It
//!   composes with the kind's own functional checks rather than duplicating them: for a
//!   website target it runs the website visual check first and only judges style when
//!   the site itself works; for a request-only build the run's build check has already
//!   passed by the time the gate runs (a failed check stops the round). Style means two
//!   things, both read from `config/slop/design-tokens.json`: the gradient palette is
//!   actually used in the generated styles, and every declared foreground/background
//!   pair meets the file's minimum WCAG contrast ratio.

use super::kinds::website::WebVisualCheck;
use super::kinds::{Check, CheckCx};
use craft_anything_engine::workspace::CheckResult;
use craft_anything_evidence::ToolError;
use serde_json::Value;
use std::path::Path;
use std::sync::OnceLock;

/// The check id stored in each gate result.
pub const SLOP_GATE: &str = "slop-gate";

/// The design tokens: an explicit file (`SLOP_ANYTHING_DESIGN_TOKENS`) when set, else
/// the copy embedded at build time from `config/slop/design-tokens.json`.
pub fn tokens() -> &'static Value {
    static TOKENS: OnceLock<Value> = OnceLock::new();
    TOKENS.get_or_init(|| {
        if let Ok(p) = std::env::var("SLOP_ANYTHING_DESIGN_TOKENS")
            && let Ok(text) = std::fs::read_to_string(&p)
            && let Ok(v) = serde_json::from_str(&text)
        {
            return v;
        }
        serde_json::from_str(include_str!("../../../../config/slop/design-tokens.json")).unwrap_or(Value::Null)
    })
}

/// The house-style section appended to a website builder's brief in Slop mode.
pub fn instructions() -> String {
    format!(
        "# House style: superslop (required for this build)\n\n\
         This project is built in Slop mode: keep the original's structure, content and behaviour, \
         but style it with the superslop design system below — the indigo-to-purple-to-pink gradient \
         (135deg, #6366f1, #a855f7, #ec4899), glassmorphism surfaces, a 3-card bento layout, taxonomy \
         pills, ambient blur orbs and the accessibility rules in the human module (WCAG 2.2 AA: text \
         contrast at least 4.5:1, visible focus rings, prefers-reduced-motion). A dedicated Slop Gate \
         check verifies the gradient palette is really used and the declared colour pairs pass \
         contrast, so apply the tokens in the generated CSS, not just in prose.\n\n\
         ## Module: UI\n\n{}\n\n## Module: copywriting\n\n{}\n\n## Module: layout and mobile\n\n{}\n\n## Module: human (accessibility and tone)\n\n{}",
        include_str!("../../../../third-party/superslop/skills/superslop-ui/SKILL.md"),
        include_str!("../../../../third-party/superslop/skills/superslop-copywriting/SKILL.md"),
        include_str!("../../../../third-party/superslop/skills/superslop-layoutmobile/SKILL.md"),
        include_str!("../../../../third-party/superslop/skills/superslop-human/SKILL.md"),
    )
}

/// One declared foreground/background pair and how it scored.
#[derive(Debug, Clone, PartialEq)]
pub struct PairResult {
    pub use_: String,
    pub foreground: String,
    pub background: String,
    pub ratio: f64,
    pub passed: bool,
}

/// What the gate found in a generated `app/` folder.
#[derive(Debug, Clone, PartialEq)]
pub struct GateOutcome {
    /// The gradient stops found in the generated styles, as written in the tokens file.
    pub palette_found: Vec<String>,
    pub palette_passed: bool,
    pub pairs: Vec<PairResult>,
    pub passed: bool,
    pub note: String,
}

/// WCAG relative luminance of a `#rrggbb` colour (0.0 when unparseable).
fn luminance(hex: &str) -> Option<f64> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 || !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let chan = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| v as f64 / 255.0);
    let lin = |c: f64| if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    Some(0.2126 * lin(chan(0)?) + 0.7152 * lin(chan(2)?) + 0.0722 * lin(chan(4)?))
}

/// The WCAG contrast ratio between two `#rrggbb` colours (1.0 when unparseable).
pub fn contrast_ratio(foreground: &str, background: &str) -> f64 {
    match (luminance(foreground), luminance(background)) {
        (Some(a), Some(b)) => {
            let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
            (hi + 0.05) / (lo + 0.05)
        }
        _ => 1.0,
    }
}

const STYLE_EXTS: &[&str] = &["css", "scss", "html", "htm", "js", "jsx", "ts", "tsx", "vue", "svelte"];
const SKIP_DIRS: &[&str] = &["node_modules", "target", "dist", ".git"];

/// The generated styles and markup of `app`, lower-cased, bounded (at most ~4000 files,
/// ~1 MiB each) so a hostile or huge folder can't stall the check.
fn collect_styles(app: &Path) -> String {
    let mut out = String::new();
    let mut stack = vec![app.to_path_buf()];
    let mut files = 0usize;
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                if !p.file_name().is_some_and(|n| SKIP_DIRS.iter().any(|s| n == *s)) {
                    stack.push(p);
                }
            } else if files < 4000 && p.extension().is_some_and(|x| STYLE_EXTS.iter().any(|s| x == *s)) {
                files += 1;
                if let Ok(bytes) = std::fs::read(&p) {
                    out.push_str(&String::from_utf8_lossy(&bytes[..bytes.len().min(1 << 20)]).to_lowercase());
                    out.push('\n');
                }
            }
        }
    }
    out
}

/// Judge a generated `app/` folder against the design tokens.
pub fn evaluate(app: &Path, tokens: &Value) -> GateOutcome {
    let stops: Vec<String> = tokens
        .pointer("/palette/gradient/stops")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default();
    let min_ratio = tokens.pointer("/accessibility/min_text_contrast_ratio").and_then(Value::as_f64).unwrap_or(4.5);
    let styles = collect_styles(app);
    let palette_found: Vec<String> = stops.iter().filter(|s| styles.contains(&s.trim_start_matches('#').to_lowercase())).cloned().collect();
    // The palette counts as used when at least two gradient stops appear in the styles.
    let palette_passed = palette_found.len() >= 2.min(stops.len());
    let pairs: Vec<PairResult> = tokens
        .pointer("/accessibility/gate_pairs")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|p| {
                    let fg = p.get("foreground")?.as_str()?.to_string();
                    let bg = p.get("background")?.as_str()?.to_string();
                    let ratio = contrast_ratio(&fg, &bg);
                    Some(PairResult {
                        use_: p.get("use").and_then(Value::as_str).unwrap_or_default().to_string(),
                        foreground: fg,
                        background: bg,
                        ratio,
                        passed: ratio >= min_ratio,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    let contrast_passed = pairs.iter().all(|p| p.passed);
    let passed = palette_passed && contrast_passed;
    let mut notes = Vec::new();
    if palette_passed {
        notes.push(format!("Superslop gradient palette used ({} of {} stops: {}).", palette_found.len(), stops.len(), palette_found.join(", ")));
    } else {
        notes.push(format!(
            "The superslop gradient palette isn't used in the generated styles (found {} of {} stops: {}). Use the design tokens' gradient colours ({}) in app/'s CSS.",
            palette_found.len(),
            stops.len(),
            if palette_found.is_empty() { "none".into() } else { palette_found.join(", ") },
            stops.join(", ")
        ));
    }
    for p in &pairs {
        if p.passed {
            notes.push(format!("Contrast {} on {} is {:.2}:1 ({}).", p.foreground, p.background, p.ratio, p.use_));
        } else {
            notes.push(format!("Contrast {} on {} is only {:.2}:1, below the {min_ratio:.1}:1 minimum ({}).", p.foreground, p.background, p.ratio, p.use_));
        }
    }
    GateOutcome { palette_found, palette_passed, pairs, passed, note: notes.join(" ") }
}

/// The Slop Gate: the website kind's check in Slop mode. See the module docs for how it
/// composes with the functional checks.
pub struct SlopGateCheck {
    /// A website target: run the visual check first (it serves the site and judges that
    /// pages load, controls work and nothing scrolls sideways); likeness is not judged.
    pub visual: bool,
}

impl Check for SlopGateCheck {
    fn id(&self) -> &'static str {
        SLOP_GATE
    }

    fn run(&self, cx: &mut CheckCx) -> Result<CheckResult, ToolError> {
        let mut result = CheckResult::new(self.id(), cx.round, true, String::new());
        if self.visual {
            let inner = WebVisualCheck.run(cx)?;
            result.artifacts = inner.artifacts;
            result.pages = inner.pages;
            result.claims = inner.claims;
            result.unknowns = inner.unknowns;
            if !inner.passed {
                result.passed = false;
                result.note = format!("The site's own check failed first, so the Slop Gate can't pass: {}", inner.note);
                return Ok(result);
            }
        }
        let out = evaluate(&cx.ws.app_dir(), tokens());
        result.passed = out.passed;
        result.note = out.note;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(css: &str) -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("style.css"), css).unwrap();
        d
    }

    #[test]
    fn a_folder_using_the_palette_and_passing_contrast_passes() {
        let d = fixture("body { background: #09090b; } .hero { background: linear-gradient(135deg, #6366f1, #a855f7, #ec4899); color: #d8b4fe; }");
        let out = evaluate(d.path(), tokens());
        assert!(out.passed, "{}", out.note);
        assert_eq!(out.palette_found.len(), 3);
        assert!(out.pairs.iter().all(|p| p.passed), "{:?}", out.pairs);
    }

    #[test]
    fn a_folder_missing_the_palette_fails() {
        let d = fixture("body { background: #ffffff; color: #111111; }");
        let out = evaluate(d.path(), tokens());
        assert!(!out.passed && !out.palette_passed, "{}", out.note);
        assert!(out.note.contains("gradient palette"), "{}", out.note);
    }

    #[test]
    fn a_low_contrast_token_pair_fails() {
        let mut t = tokens().clone();
        t["accessibility"]["gate_pairs"] = serde_json::json!([{ "foreground": "#777777", "background": "#888888", "use": "grey on grey" }]);
        let d = fixture(".hero { background: linear-gradient(135deg, #6366f1, #a855f7, #ec4899); }");
        let out = evaluate(d.path(), &t);
        assert!(!out.passed, "{}", out.note);
        assert!(out.palette_passed, "the palette is fine; contrast is what fails: {}", out.note);
        assert_eq!(out.pairs.len(), 1);
        assert!(!out.pairs[0].passed && out.pairs[0].ratio < 4.5, "{:?}", out.pairs);
    }

    #[test]
    fn contrast_ratio_matches_wcags_own_examples() {
        assert!((contrast_ratio("#000000", "#ffffff") - 21.0).abs() < 0.01);
        assert!((contrast_ratio("#ffffff", "#ffffff") - 1.0).abs() < 0.001);
        assert!(contrast_ratio("#d8b4fe", "#09090b") > 4.5, "the fork's accent pair passes its own gate");
        assert_eq!(contrast_ratio("not-a-colour", "#ffffff"), 1.0, "unparseable is a failure, not a crash");
    }

    #[test]
    fn the_instructions_carry_the_vendored_modules() {
        let text = instructions();
        for needle in ["superslop", "#6366f1", "WCAG"] {
            assert!(text.contains(needle), "instructions missing {needle}");
        }
        assert!(text.len() > 10_000, "the vendored modules are embedded, not summarised ({} bytes)", text.len());
    }
}
