<h1 align="center">Slop Anything LITE</h1>

<p align="center">
  <b>See how a website or app works, have an AI builder rebuild it, check the result — and ship it in superslop style.</b>
</p>

<p align="center">
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-a855f7?style=flat-square">
  <img alt="Status: scaffold" src="https://img.shields.io/badge/status-scaffold-ec4899?style=flat-square">
</p>

Slop Anything LITE is a fork of Craft Anything, keeping its core workflow —
**Study → Build → Check → Refine**, with evidence behind every finding and
Clean room as the default — with two changes:

1. **Wider AI builder and provider support.** Upstream ships one verified
   builder route. This fork verifies and hardens the others: more agent CLIs,
   agent-protocol builders, and direct API backends. See
   `docs/slop/SPEC.md` and the catalogue in `config/slop/providers.toml`.
2. **Superslop output by default.** Projects generated in the default Slop
   mode are restyled with the `superslop` skill suite: the indigo-to-purple-
   to-pink gradient, glass, bento layout, and its accessibility gates. A
   `--faithful` mode keeps the upstream likeness-checked behaviour instead.

> Based on Craft Anything by the ArtCraft team and the Craft Anything
> contributors. This fork is not an ArtCraft product and is not endorsed by
> the ArtCraft team. Full provenance and licensing: `docs/slop/PROVENANCE.md`.

## Status

Scaffold phase. The engine code is upstream code, used as it is: its crates
are internal workspace crates and keep their `craft-anything-*` names (see
the decision in `docs/slop/SPEC.md`), and the provider and superslop
features in the spec are planned work, not shipped features. The build
order is at the bottom of `docs/slop/SPEC.md`.

What already works is what upstream had working: the desktop app, the CLI,
and the MCP server; the Study → Build → Check → Refine workflow for websites
with the verified upstream builder; website checks at three screen widths;
Clean room and Deep modes. Upstream's own status document is
`docs/STATUS.md`.

## Build from source

Requires Rust 1.97 or newer. On Debian or Ubuntu, the desktop app also needs
the system libraries listed in `docs/STATUS.md` and the upstream getting
started notes.

```sh
git clone https://github.com/superslop/slop-anything-lite
cd slop-anything-lite
cargo run --release -p craft-anything        # desktop app (internal name kept from upstream)
cargo install --path apps/craft-anything-cli # CLI, as craft-anything-cli (internal name kept)
```

Building a project (as opposed to studying one) needs a builder: an agent
CLI signed in on your computer, or an API key for one of the API backends,
always your own. This tool never handles your credentials.

## License

MIT, elected from upstream's MIT OR Apache-2.0 dual licence for this fork's
own distributions. Both upstream licence texts, the upstream `NOTICE`, and
`ATTRIBUTION.md` are retained: see `docs/slop/PROVENANCE.md` for why, and for
the third-party licences (fonts, icons, crates) that continue to apply.

The `superslop` skill suite used for output styling is MIT, Copyright (c)
2026 John Dhoe, from `superslop/superslop`.
