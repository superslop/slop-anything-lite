#!/usr/bin/env python3
"""
✨🚀 SuperSlop MCP Server (slop-mcp.py)
Zero-dependency JSON-RPC 2.0 stdio MCP server exposing:
  1. superslop_contrast_check(foreground, background)
  2. superslop_audit_text(text)
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import importlib.util

_spec = importlib.util.spec_from_file_location(
    "slop_check", Path(__file__).resolve().parent / "slop-check.py"
)
slop_check = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(slop_check)

TOOLS = [
    {
        "name": "superslop_contrast_check",
        "description": "Verify WCAG 2.2 AA/AAA contrast ratio between two hex colors for SuperSlop neon gradients and dark glass surfaces.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "foreground": {"type": "string", "description": "Foreground hex color (e.g. #d8b4fe)"},
                "background": {"type": "string", "description": "Background hex color (e.g. #09090b)"},
            },
            "required": ["foreground", "background"],
        },
    },
    {
        "name": "superslop_audit_text",
        "description": "Score a snippet of prose, UI code, or source code for SuperSlop Synthetic Density (0-100).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "text": {"type": "string", "description": "Text or code to score for SuperSlop tropes"},
            },
            "required": ["text"],
        },
    },
]


def handle_request(req: dict) -> dict | None:
    method = req.get("method")
    req_id = req.get("id")
    if method == "initialize":
        return {
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "superslop-mcp", "version": "1.0.0"},
            },
        }
    if method == "notifications/initialized":
        return None
    if method == "tools/list":
        return {"jsonrpc": "2.0", "id": req_id, "result": {"tools": TOOLS}}
    if method == "tools/call":
        params = req.get("params") or {}
        name = params.get("name")
        args = params.get("arguments") or {}
        if name == "superslop_contrast_check":
            fg = args.get("foreground", "#ffffff")
            bg = args.get("background", "#09090b")
            try:
                ratio = slop_check.contrast_ratio(fg, bg)
                status = "PASS AAA" if ratio >= 7.0 else ("PASS AA" if ratio >= 4.5 else "FAIL")
                text = f"Contrast {fg} on {bg}: {ratio:.2f}:1 ({status})"
            except ValueError as exc:
                text = f"Error: {exc}"
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {"content": [{"type": "text", "text": text}]},
            }
        if name == "superslop_audit_text":
            sample = args.get("text", "")
            lex = len(slop_check.LEXICON_RE.findall(sample))
            grad = len(slop_check.GRADIENT_RE.findall(sample))
            emo = len(slop_check.EMOJI_RE.findall(sample))
            emd = len(slop_check.EMDASH_RE.findall(sample))
            ban = len(slop_check.BANNER_RE.findall(sample))
            score = min(100, lex * 8 + grad * 10 + emo * 5 + emd * 8 + ban * 10)
            summary = (
                f"SuperSlop Density Score: {score}/100 ✨🚀\n"
                f"- Lexicon hits: {lex}\n- Gradients/Glass: {grad}\n"
                f"- Sparkle/Rocket Emojis: {emo}\n- Em-Dashes: {emd}\n- Code Banners: {ban}"
            )
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {"content": [{"type": "text", "text": summary}]},
            }
    if req_id is not None:
        return {
            "jsonrpc": "2.0",
            "id": req_id,
            "error": {"code": -32601, "message": f"Method not found: {method}"},
        }
    return None


def main() -> None:
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except json.JSONDecodeError:
            continue
        resp = handle_request(req)
        if resp is not None:
            sys.stdout.write(json.dumps(resp) + "\n")
            sys.stdout.flush()


if __name__ == "__main__":
    main()
