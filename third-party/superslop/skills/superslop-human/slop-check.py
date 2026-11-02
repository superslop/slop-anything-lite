#!/usr/bin/env python3
"""
✨🚀 SuperSlop Ethical Contrast & Slop-O-Meter Checker (slop-check.py)
Zero-dependency Python 3 utility for skills/superslop-human.

Usage:
  python3 skills/superslop-human/slop-check.py "#09090b" "#d8b4fe"
  python3 skills/superslop-human/slop-check.py --audit path/to/file_or_dir
"""

import math
import os
import re
import sys
from pathlib import Path


def parse_hex(hex_str: str) -> tuple[float, float, float]:
    cleaned = hex_str.strip().lstrip("#")
    if len(cleaned) == 3:
        cleaned = "".join(ch * 2 for ch in cleaned)
    if len(cleaned) != 6 or not re.fullmatch(r"[0-9a-fA-F]{6}", cleaned):
        raise ValueError(f"Invalid hex color: {hex_str!r} (expected #RRGGBB or #RGB)")
    r = int(cleaned[0:2], 16) / 255.0
    g = int(cleaned[2:4], 16) / 255.0
    b = int(cleaned[4:6], 16) / 255.0
    return r, g, b


def channel_luminance(c: float) -> float:
    return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4


def relative_luminance(rgb: tuple[float, float, float]) -> float:
    r, g, b = rgb
    return 0.2126 * channel_luminance(r) + 0.7152 * channel_luminance(g) + 0.0722 * channel_luminance(b)


def contrast_ratio(hex1: str, hex2: str) -> float:
    l1 = relative_luminance(parse_hex(hex1))
    l2 = relative_luminance(parse_hex(hex2))
    lighter, darker = max(l1, l2), min(l1, l2)
    return (lighter + 0.05) / (darker + 0.05)


def check_contrast(hex1: str, hex2: str) -> int:
    try:
        ratio = contrast_ratio(hex1, hex2)
    except ValueError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    normal = "PASS AAA" if ratio >= 7.0 else ("PASS AA" if ratio >= 4.5 else "FAIL")
    large = "PASS AAA" if ratio >= 4.5 else ("PASS AA" if ratio >= 3.0 else "FAIL")
    normal_op = ">=" if ratio >= 4.5 else "<"
    large_op = ">=" if ratio >= 3.0 else "<"

    print(f"ratio:       {ratio:.2f}:1")
    print(f"normal text: {normal} ({ratio:.2f} {normal_op} 4.5)")
    print(f"large text:  {large} ({ratio:.2f} {large_op} 3.0)")
    if ratio >= 4.5:
        print("slop vibe:   CERTIFIED ACCESSIBLE NEON GLOW ✨🚀")
        return 0
    print("slop vibe:   NEEDS MORE CONTRAST (Rule E-03: SuperSlop never locks out human eyes)")
    return 2


LEXICON_RE = re.compile(
    r"\b(delve|delving|tapestry|seamless|seamlessly|unleash|elevate|supercharge|"
    r"paradigm|synergy|transformative|game-changing|bespoke|holistic|symphony|ecosystem)\b",
    re.IGNORECASE,
)
GRADIENT_RE = re.compile(
    r"(from-indigo|via-purple|to-pink|#6366f1|#a855f7|#ec4899|linear-gradient|backdrop-blur|blur-3xl|rounded-full)",
    re.IGNORECASE,
)
EMOJI_RE = re.compile(r"(✨|🚀|🔥|⚡|🧠|💡|🔮|🎉)")
EMDASH_RE = re.compile(r"(—|not just\s+[^—\n]+—\s*it's)", re.IGNORECASE)
BANNER_RE = re.compile(r"(={20,}|STEP\s+\d+|End of .+ function|Increment .+ by 1)")

SCAN_EXTS = {
    ".md", ".mdc", ".txt", ".html", ".css", ".js", ".mjs",
    ".ts", ".tsx", ".jsx", ".py", ".go", ".rs", ".svelte", ".vue",
}


def audit_path(target: str) -> int:
    root = Path(target)
    files: list[Path] = []
    if root.is_file():
        files = [root]
    elif root.is_dir():
        for dirpath, dirnames, filenames in os.walk(root):
            dirnames[:] = [d for d in dirnames if d not in {".git", "node_modules", "__pycache__", "vendor"}]
            for fn in filenames:
                p = Path(dirpath) / fn
                if p.suffix.lower() in SCAN_EXTS:
                    files.append(p)
    else:
        print(f"error: path not found: {target}", file=sys.stderr)
        return 1

    counts = {
        "lexicon": 0,
        "gradients": 0,
        "emojis": 0,
        "em_dashes": 0,
        "code_banners": 0,
    }
    for f in files:
        try:
            text = f.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        counts["lexicon"] += len(LEXICON_RE.findall(text))
        counts["gradients"] += len(GRADIENT_RE.findall(text))
        counts["emojis"] += len(EMOJI_RE.findall(text))
        counts["em_dashes"] += len(EMDASH_RE.findall(text))
        counts["code_banners"] += len(BANNER_RE.findall(text))

    weighted = (
        counts["lexicon"] * 3
        + counts["gradients"] * 4
        + counts["emojis"] * 2
        + counts["em_dashes"] * 3
        + counts["code_banners"] * 4
    )
    norm = max(1.0, len(files) * 0.6)
    score = min(100, round(weighted / norm))

    if score >= 90:
        grade = "S+ (SUPREME SYNTHETIC SINGULARITY ✨🚀)"
    elif score >= 75:
        grade = "A (HYPER-SLOP CERTIFIED 🔥)"
    elif score >= 50:
        grade = "B (PROMISING SAAS BOILERPLATE ⚡)"
    elif score >= 25:
        grade = "C (MILD CORPORATE AI VIBES 💡)"
    else:
        grade = "F (SUSPICIOUSLY HUMAN & ARTISANAL 🧐)"

    print(f"target:        {target} ({len(files)} file(s))")
    print(f"slop score:    {score}/100")
    print(f"slop grade:    {grade}")
    print(f"lexicon hits:  {counts['lexicon']}")
    print(f"neon/glass:    {counts['gradients']}")
    print(f"emojis:        {counts['emojis']}")
    print(f"em-dashes:     {counts['em_dashes']}")
    print(f"code banners:  {counts['code_banners']}")
    return 0


def main() -> int:
    args = sys.argv[1:]
    if not args or args[0] in {"-h", "--help"}:
        print(__doc__.strip())
        return 0
    if args[0] in {"--audit", "audit"}:
        target = args[1] if len(args) > 1 else "."
        return audit_path(target)
    if len(args) >= 2:
        return check_contrast(args[0], args[1])
    print("Usage: slop-check.py <hex1> <hex2> | slop-check.py --audit [path]", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
