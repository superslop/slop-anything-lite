# Slop Anything LITE app icon

The **sparkles**: this fork's own icon, drawn for Slop Anything LITE. A white
four-point sparkle with two smaller sparkles on a rounded tile filled with
the superslop gradient (electric indigo `#6366f1` to purple `#a855f7` to
pink `#ec4899`, at 135 degrees). It replaces the upstream icon, which was not
open source; see `docs/slop/PROVENANCE.md`.

The icon is covered by this repository's MIT licence: see `LICENSE.txt` in
this directory.

## Geometry

A 512-unit tile with 110-unit corners. The large sparkle is centred; the two
small sparkles sit at the top-right and bottom-left. File names and the
hicolor layout still use the upstream machine names in the scaffold phase;
the rename series in `docs/slop/SPEC.md` will change them.

## Files

- `craft-anything.svg`: the master SVG (also the hicolor scalable icon).
- `craft-anything-1024.png`: 1024 px render of the tile.
- `craft-anything.icns`: macOS bundle icon.
- `craft-anything.ico`: Windows icon (16, 24, 32, 48, 64, 128, 256).
- `hicolor/<size>/apps/ai.storyteller.craft-anything.png` (16–512) and
  `hicolor/scalable/apps/ai.storyteller.craft-anything.svg`: Linux icon theme.
- `trace.py`: an upstream tool, retained and covered by the repository
  licence; it is not used by this icon.
