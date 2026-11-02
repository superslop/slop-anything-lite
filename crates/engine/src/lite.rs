//! The LITE profile: the tool families this fork hides by default on the CLI and MCP
//! surfaces (see `docs/slop/SPEC.md`, "LITE scope"). Nothing is deleted: the families stay
//! registered for the desktop app and for `cargo xtask` (which documents and checks the
//! full list), and a CLI run asks for everything with `--full` or `SLOP_ANYTHING_FULL=1`.
//!
//! The families are named by the prefix of their tool ids (plus the database toolset,
//! whose one tool is `db_sqlite_show`):
//! firmware images, packet captures, databases, radio recordings, game archives,
//! HAR stand-in backends (`web_standin_*`), and hosted asset generation
//! (`assets_image_generate`, `assets_model_generate`, which spend money with a hosted
//! provider). `engines_tshark_check` belongs to the packet family: tshark is its
//! optional external engine.

use crate::tool::{ToolSpec, Toolset};

/// The environment variable that turns the full profile on (`1` or `true`).
pub const FULL_ENV: &str = "SLOP_ANYTHING_FULL";

/// Tool id prefixes whose whole family the LITE profile hides.
const GATED_PREFIXES: &[&str] = &["firmware_", "packets_", "radio_", "game_", "web_standin_"];

/// Single tools the LITE profile hides, whose id doesn't carry their family's prefix.
const GATED_IDS: &[&str] = &["assets_image_generate", "assets_model_generate", "engines_tshark_check"];

/// Whether the environment asks for the full profile.
pub fn full_profile() -> bool {
    std::env::var(FULL_ENV).is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Whether the LITE profile hides this tool.
pub fn is_gated(spec: &ToolSpec) -> bool {
    spec.toolset == Toolset::Database || GATED_IDS.contains(&spec.id) || GATED_PREFIXES.iter().any(|p| spec.id.starts_with(p))
}

/// The tools a surface shows: all of them in the full profile, the LITE subset otherwise.
pub fn filter_tools(specs: Vec<ToolSpec>, full: bool) -> Vec<ToolSpec> {
    if full {
        return specs;
    }
    specs.into_iter().filter(|s| !is_gated(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{Effects, Requires, empty_schema};
    use craft_anything_evidence::ToolError;
    use serde_json::Value;

    fn run(_: &mut crate::tool::Ctx<'_>, _: Value) -> Result<crate::tool::ToolOutput, ToolError> {
        Ok(crate::tool::ToolOutput::Value(Value::Null))
    }

    fn spec(id: &'static str, toolset: Toolset) -> ToolSpec {
        ToolSpec {
            id,
            title: id,
            summary: "",
            beginner: "",
            toolset,
            effects: Effects::READ_ONLY,
            requires: Requires::Nothing,
            input_schema: empty_schema,
            run,
        }
    }

    fn ids(specs: &[ToolSpec]) -> Vec<&'static str> {
        specs.iter().map(|s| s.id).collect()
    }

    #[test]
    fn gated_families_are_absent_in_lite_and_present_in_full() {
        let all = vec![
            spec("firmware_regions_list", Toolset::Formats),
            spec("packets_capture_show", Toolset::Formats),
            spec("db_sqlite_show", Toolset::Database),
            spec("radio_recording_decode", Toolset::Formats),
            spec("game_archives_list", Toolset::Formats),
            spec("web_standin_start", Toolset::Browser),
            spec("assets_image_generate", Toolset::Core),
            spec("assets_model_generate", Toolset::Core),
            spec("engines_tshark_check", Toolset::Setup),
        ];
        assert!(filter_tools(all.clone(), false).is_empty(), "every gated family is hidden in LITE");
        assert_eq!(ids(&filter_tools(all, true)).len(), 9, "the full profile keeps them all");
    }

    #[test]
    fn kept_families_are_present_in_both_profiles() {
        let kept = vec![
            spec("core_target_open", Toolset::Core),
            spec("website_capture", Toolset::Browser),
            spec("engines_install", Toolset::Setup),
            spec("rebuild_start", Toolset::Application),
        ];
        for full in [false, true] {
            assert_eq!(ids(&filter_tools(kept.clone(), full)), ids(&kept), "full={full}");
        }
    }

    #[test]
    fn the_real_registry_loses_only_the_gated_families_in_lite() {
        let all = crate::tools::all();
        let lite = filter_tools(all.clone(), false);
        assert!(lite.len() < all.len(), "LITE hides something");
        assert!(lite.iter().all(|s| !is_gated(s)));
        // Kept families the fork's core workflow needs.
        for id in ["core_target_open", "db_sqlite_show"] {
            assert_eq!(all.iter().any(|s| s.id == id), true, "{id} exists upstream of the filter");
        }
        assert!(lite.iter().any(|s| s.id == "core_target_open"), "essentials survive the filter");
        assert!(!lite.iter().any(|s| s.id == "db_sqlite_show"), "the database family is hidden");
    }
}
