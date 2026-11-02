//! The builder and provider catalogue from `config/slop/providers.toml` (this fork's
//! data-over-code catalogue; see `docs/slop/SPEC.md`). The file lists every builder and
//! API backend the fork supports, with the program or base URL it runs and the status
//! wording the fork uses ("implemented, not live-verified" for its own work).
//!
//! Loading never fails the app: [`load`] looks for a file (the
//! `SLOP_ANYTHING_PROVIDERS_TOML` environment variable, then `config/slop/providers.toml`
//! under the working directory or beside the executable's tree) and falls back to the
//! copy embedded at build time, so the catalogue is always available. Parsing a file
//! that exists but is broken is an error the caller can show, never a panic.

use craft_anything_evidence::{ErrorCode, ToolError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// One Tier 1 / Tier 3 builder row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuilderRow {
    pub id: String,
    /// `agent-cli`, `acp`, or `generic-cli`.
    pub kind: String,
    /// The default program (empty for the generic adapter: the person's config names it).
    #[serde(default)]
    pub program: String,
    #[serde(default)]
    pub upstream_status: String,
    #[serde(default)]
    pub lite_status: String,
    #[serde(default)]
    pub resume: bool,
}

/// One Tier 2 API backend row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendRow {
    pub id: String,
    pub kind: String,
    /// `anthropic` or `openai` (the wire format).
    #[serde(default)]
    pub api_style: String,
    #[serde(default)]
    pub base_url: String,
    /// The environment variable that may hold the key; the key itself is never in the file.
    #[serde(default)]
    pub api_key_env: String,
    #[serde(default)]
    pub upstream_status: String,
    #[serde(default)]
    pub lite_status: String,
}

/// The whole catalogue file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalogue {
    pub version: u32,
    #[serde(default, rename = "builders")]
    pub builders: Vec<BuilderRow>,
    #[serde(default, rename = "backends")]
    pub backends: Vec<BackendRow>,
}

impl Catalogue {
    pub fn parse(text: &str) -> Result<Catalogue, ToolError> {
        toml::from_str(text).map_err(|e| {
            ToolError::new(ErrorCode::BadSettings, format!("The provider catalogue (config/slop/providers.toml) couldn't be read: {e}"))
                .with_remediation("Fix the file, or remove SLOP_ANYTHING_PROVIDERS_TOML to use the built-in catalogue.")
        })
    }

    /// The catalogue embedded at build time (the repository's own file).
    pub fn embedded() -> Catalogue {
        Catalogue::parse(include_str!("../../../config/slop/providers.toml")).unwrap_or(Catalogue { version: 0, builders: Vec::new(), backends: Vec::new() })
    }

    pub fn builder(&self, id: &str) -> Option<&BuilderRow> {
        self.builders.iter().find(|b| b.id == id)
    }

    pub fn backend(&self, id: &str) -> Option<&BackendRow> {
        self.backends.iter().find(|b| b.id == id)
    }
}

/// Where a loaded catalogue came from (a path, or the embedded copy).
pub struct Loaded {
    pub catalogue: Catalogue,
    pub source: String,
}

/// The catalogue to use: an explicit file when one is set or found, else the embedded copy.
pub fn load() -> Result<Loaded, ToolError> {
    if let Ok(p) = std::env::var("SLOP_ANYTHING_PROVIDERS_TOML")
        && !p.trim().is_empty()
    {
        let text = std::fs::read_to_string(&p).map_err(|e| ToolError::io(&p, e))?;
        return Ok(Loaded { catalogue: Catalogue::parse(&text)?, source: p });
    }
    let mut candidates: Vec<PathBuf> = vec![PathBuf::from("config/slop/providers.toml")];
    if let Ok(exe) = std::env::current_exe() {
        let mut dir = exe.parent().map(|p| p.to_path_buf());
        for _ in 0..4 {
            if let Some(d) = dir {
                candidates.push(d.join("config/slop/providers.toml"));
                dir = d.parent().map(|p| p.to_path_buf());
            }
        }
    }
    for p in candidates {
        if let Ok(text) = std::fs::read_to_string(&p) {
            return Ok(Loaded { catalogue: Catalogue::parse(&text)?, source: p.display().to_string() });
        }
    }
    Ok(Loaded { catalogue: Catalogue::embedded(), source: "built-in (embedded at build time)".into() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shipped_catalogue_parses_and_covers_every_tier() {
        let c = Catalogue::embedded();
        assert_eq!(c.builders.len(), 7, "{:?}", c.builders.iter().map(|b| &b.id).collect::<Vec<_>>());
        assert_eq!(c.backends.len(), 4);
        // Tier 1 + ACP + Tier 3 are all listed.
        for id in ["claude-code", "codex", "gemini-cli", "qwen-code", "opencode", "goose", "generic-cli"] {
            assert!(c.builder(id).is_some(), "builder {id} missing");
        }
        let generic = c.builder("generic-cli").unwrap();
        assert_eq!(generic.kind, "generic-cli");
        assert!(generic.program.is_empty(), "the generic adapter's program comes from the person's config");
        // Tier 2 rows carry wire format, base URL and the key's env var name (never a key).
        let or = c.backend("openrouter").unwrap();
        assert_eq!(or.api_style, "openai");
        assert_eq!(or.base_url, "https://openrouter.ai/api/v1");
        assert_eq!(or.api_key_env, "OPENROUTER_API_KEY");
        assert_eq!(c.backend("anthropic").unwrap().api_style, "anthropic");
        assert!(c.backend("local-openai-compatible").is_some());
    }

    #[test]
    fn a_broken_catalogue_is_an_error_not_a_panic() {
        assert!(Catalogue::parse("builders = 3").is_err());
        assert!(Catalogue::parse("").is_err(), "version is required");
        let ok = Catalogue::parse("version = 1\n").unwrap();
        assert!(ok.builders.is_empty() && ok.backends.is_empty());
    }
}
