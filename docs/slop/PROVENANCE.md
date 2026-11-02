# Fork provenance and licence election

## What this repository is

Slop Anything LITE is a fork of, and a derivative work based on, Craft Anything
by the ArtCraft team and the Craft Anything contributors.

- Upstream repository: `storytold/craft-anything`
- Upstream commit at fork time: `1dcb46b2f8eeea58fa57aecc45fafd5a0d1a8318`
- Forked on: 2026-11-02
- This repository: `superslop/slop-anything-lite`

This file is the plain-text attribution the upstream brand terms allow. It is
not an endorsement: the ArtCraft team does not make, sponsor, or certify this
fork, and this fork is not an ArtCraft product.

## Licence election

Upstream is dual-licensed under the MIT License or the Apache License,
Version 2.0, at the recipient's option (`LICENSE-MIT`, `LICENSE-APACHE`,
`NOTICE`, Copyright (c) 2026 ArtCraft Team and the Craft Anything
contributors).

For its own copies and distributions, this fork elects the **MIT License**.
Both upstream licence texts and the upstream `NOTICE` and `ATTRIBUTION.md`
files are retained unmodified in this repository, because:

- recipients who received upstream code under either option keep their rights;
- third-party material bundled with upstream keeps its own licences, as listed
  in `NOTICE` and `ATTRIBUTION.md` (fonts under SIL OFL 1.1, Lucide icons under
  ISC, and the Rust crate licence bundle in `docs/licences.md`).

New material written for this fork is licensed under the MIT License unless a
file states otherwise.

## Brand and icon removal

The ArtCraft name, wordmark, and logos in `docs/brand/` are trademarks of the
ArtCraft Team, are not open source, and are not covered by the MIT or
Apache-2.0 licences. The Craft Anything app icon (the raven in
`assets/app-icon/`) is under the same brand terms.

A modified version that is distributed must remove those marks and must not be
called ArtCraft or be presented as an ArtCraft product. This fork therefore:

- uses its own name, Slop Anything LITE;
- removes the mark files in `docs/brand/` in a dedicated rebrand commit (the
  brand licence text itself is kept, as the record of the terms);
- replaces the app icon with its own original icon in a dedicated commit;
- keeps only plain-text references to Craft Anything and the ArtCraft team in
  attribution contexts such as this file, `NOTICE`, and `ATTRIBUTION.md`.

## The superslop skill suite

Generated projects are styled with the `superslop` skill suite from
`superslop/superslop` (MIT License, Copyright (c) 2026 John Dhoe). That suite
is vendored or referenced under its own licence text wherever it is included,
separately from the upstream attribution above.

Vendored copy: `third-party/superslop/`, pinned at commit
`95067f878308793fbc9f4f89b1c958fa41901bea` (2026-10-29) of
`superslop/superslop`, kept intact including its `LICENSE`. The rebuild
workflow's Slop mode embeds its UI, copywriting, layout/mobile and human
`SKILL.md` modules into website builders' briefs; the Slop Gate check reads
the companion design tokens in `config/slop/design-tokens.json`.
