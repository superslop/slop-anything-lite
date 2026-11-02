<h1 align="center">Slop Anything LITE</h1>

<p align="center">
  <b>See how a website or app works, have an AI builder rebuild it, check the result — and ship it in superslop style.</b>
</p>

<p align="center">
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-a855f7?style=flat-square">
  <img alt="Status: alpha" src="https://img.shields.io/badge/status-alpha-ec4899?style=flat-square">
</p>

Slop Anything LITE is a fork of Craft Anything, keeping its core workflow —
**Study → Build → Check → Refine**, with evidence behind every finding and
Clean room as the default — with two changes:

1. **Wider AI builder and provider support.** Upstream ships one verified
   builder route. This fork wires the others: agent CLIs (Claude Code,
   Codex — now with thread resume — Gemini CLI, Qwen Code, opencode and
   goose over ACP, plus a config-driven generic adapter for any other
   agent program), and direct API backends (Anthropic, OpenAI, OpenRouter,
   and any OpenAI-compatible or local server). The catalogue is data:
   `config/slop/providers.toml`, shown by `slop-anything-cli ai catalogue`.
   These routes are implemented and unit-tested but not live-verified —
   see `docs/slop/SPEC.md`.
2. **Superslop output by default.** Projects generated in the default Slop
   mode are restyled with the `superslop` skill suite: the indigo-to-purple-
   to-pink gradient, glass, bento layout, and its accessibility gates. A
   `--faithful` mode keeps the upstream likeness-checked behaviour instead.

> Based on Craft Anything by the ArtCraft team and the Craft Anything
> contributors. This fork is not an ArtCraft product and is not endorsed by
> the ArtCraft team. Full provenance and licensing: `docs/slop/PROVENANCE.md`.

## Status

Early alpha — the same maturity upstream claims for itself. The engine code
is upstream code, used as it is: its crates are internal workspace crates
and keep their `craft-anything-*` names (see the decision in
`docs/slop/SPEC.md`). The fork's features are implemented; the wider
provider routes are implemented, not live-verified. The build order is at
the bottom of `docs/slop/SPEC.md`.

What already works is what upstream had working: the desktop app, the CLI,
and the MCP server; the Study → Build → Check → Refine workflow for websites
with the verified upstream builder; website checks at three screen widths;
Clean room and Deep modes. Upstream's own status document is
`docs/STATUS.md`.

The fork's own work on top: binaries named `slop-anything`,
`slop-anything-cli` and `slop-anything-web`; the LITE tool profile on the
CLI and MCP surfaces (firmware, packet, database, radio, game-archive,
stand-in and hosted-asset families hidden unless `--full` or
`SLOP_ANYTHING_FULL=1`); the wider builder/provider wiring above; and Slop
mode for website builds — superslop styling in the builder's brief, judged
by the Slop Gate (palette use + WCAG contrast + the functional checks)
instead of likeness, with `--faithful` restoring upstream behaviour.
Packaging scripts under `packaging/` still reference the old binary file
names and have not been through a rename pass yet.

## Build from source

Requires Rust 1.97 or newer. On Debian or Ubuntu, the desktop app also needs
the system libraries listed in `docs/STATUS.md` and the upstream getting
started notes.

```sh
git clone https://github.com/superslop/slop-anything-lite
cd slop-anything-lite
cargo run --release -p craft-anything        # desktop app (binary name: slop-anything)
cargo install --path apps/craft-anything-cli # CLI, installed as slop-anything-cli
```

Building a project (as opposed to studying one) needs a builder: an agent
CLI signed in on your computer, or an API key for one of the API backends,
always your own. This tool never handles your credentials.

## Downloads

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `craft-anything-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `craft-anything-cli-<ver>-macos-universal.zip` | Signed and notarized; see below if macOS blocks it |

Versions after 0.2.0 include the command-line tool in the app, so builds don't need the separate
zip. You only need it to use the command line without the app.

A browser download of the zip can stop with "Apple could not verify craft-anything-cli" (a lone
program can't carry its notarization with it, so macOS has to look it up online). A download from
the terminal isn't checked this way, so use this instead:

```sh
ver=0.2.0   # change this to the version you want
curl -fLO "https://github.com/storytold/craft-anything/releases/download/v$ver/craft-anything-cli-$ver-macos-universal.zip"
ditto -x -k "craft-anything-cli-$ver-macos-universal.zip" .
mkdir -p ~/.local/bin
mv "craft-anything-cli-$ver-macos-universal/craft-anything-cli" ~/.local/bin/
~/.local/bin/craft-anything-cli --version
```

To run it as just `craft-anything-cli`, add `~/.local/bin` to your `PATH`.

If you already have the blocked copy, press Done (not Move to Bin), then run
`xattr -d com.apple.quarantine <path to craft-anything-cli>`, or allow it in System Settings →
Privacy & Security → Open Anyway.

## License

MIT, elected from upstream's MIT OR Apache-2.0 dual licence for this fork's
own distributions. Both upstream licence texts, the upstream `NOTICE`, and
`ATTRIBUTION.md` are retained: see `docs/slop/PROVENANCE.md` for why, and for
the third-party licences (fonts, icons, crates) that continue to apply.

The `superslop` skill suite used for output styling is MIT, Copyright (c)
2026 John Dhoe, from `superslop/superslop`.
