# Slop Anything LITE — design spec

Status: scaffold. This document defines the fork. Nothing here is wired into
the engine yet unless a section says so; each build step lands in its own
commit, in the order at the bottom of this file.

## Purpose

Keep the core of the upstream rebuild workflow — Study, Build, Check, Refine,
with evidence at every step and Clean room as the default — while:

1. supporting more AI builders and model providers than the one verified
   upstream route, and
2. styling every generated project with the `superslop` skill suite by
   default, so the output of this tool has a recognisable, consistent look.

Provenance and licensing are in `docs/slop/PROVENANCE.md`.

## Modes

The upstream product scores a website build by visual likeness to the
original. A fixed house style and a likeness score pull in opposite
directions, so the fork has two modes:

- **Slop mode (default).** Clone the function: page structure, content, and
  behaviour of the target. Restyle the result with the superslop design
  tokens in `config/slop/design-tokens.json`. Ready means: the build passes,
  the functional checks pass, and the Slop Gate (below) passes. No likeness
  threshold.
- **Faithful mode (`--faithful`).** Upstream behaviour unchanged. The
  likeness check governs, refine rounds run until the upstream threshold or
  the round limit, and superslop styling is applied only to generated
  documentation (README files), never to the target's user interface.

> Implementation status: landed. `RunOptions.faithful` threads the mode
> through a run; the CLI rebuild takes `--faithful`. In Slop mode a website
> build's brief carries the superslop stack and the Slop Gate (below)
> replaces the likeness check; in Faithful mode the brief and the check list
> are upstream's exactly (no superslop at all — the documentation-only
> styling split was not cleanly separable, so Faithful applies none).

## LITE scope

Kept:

- The rebuild workflow (`crates/ai/src/rebuild/`): kinds, checks, and stacks
  stay pluggable, as upstream designed them.
- Website targets: capture at desktop, tablet, and phone widths; design
  tokens; layout measurement; page health.
- Clean room and Deep mode, with Clean room the default.
- CLI, MCP server, and the Simple desktop mode.
- The project folder format: `craft.json`, `capture/`, `app/`, `checks/`,
  the evidence store, and the question/approval inbox.
- Evidence records with confidence, coverage, and limitations.

Cut or feature-gated off in the LITE build:

- The Advanced workbench.
- Game rebuilds and game-engine installation flows.
- External-engine bridges used only by Deep native analysis.
- Firmware, packet, database, and radio tool families.
- WASM plugins.
- Generated image and 3D assets through hosted asset providers.
- HAR stand-in backends.

> Implementation status: landed as gates, not deletions, in
> `crates/engine/src/lite.rs`. The CLI and the MCP server it hosts build
> their registry through the LITE profile by default (`Engine::
> shared_with_profile`); `--full` or `SLOP_ANYTHING_FULL=1` restores every
> family. Gated by tool id/toolset: firmware, packets (including the tshark
> check), databases, radio, game archives, `web_standin_*`, and the hosted
> asset generators. The CLI rebuild additionally drops the game kind in
> LITE. Two scope notes: the Advanced workbench is a desktop UI mode, not
> tool-driven, so it is not gated (the desktop keeps the full registry);
> WASM plugin checks still run, as they are the fork-registered extension
> seam rather than a tool family.

Machine names (`craft-anything-*` crates and binaries, the bundle id) are
unchanged in the scaffold phase.

**Decision (2026-11-02): the internal crates keep their upstream names
permanently.** The engine is 13 internal workspace crates consumed as path
dependencies (`publish = false`; they are not crates.io libraries). This
fork uses them as they are. Renaming them would touch every manifest, every
`craft_anything_*` import, the layer registry, and persisted format strings,
for no functional gain — and it would make merging upstream changes (the
upstream project is actively developed) permanently harder. Keeping the
names keeps upstream sync a plain merge. Only the user-facing binary and
display names may change later; see the build order.

## AI builders and providers

Three tiers, in the order they will be verified. A tier is listed as
supported only after a live build has been run with it in this fork; upstream
test status is recorded as upstream status, not as ours.

**Verification policy change (2026-11-02, user decision):** the user holds
no accounts or API access for the individual providers, so live end-to-end
verification is waived. Builders and backends are implemented and wired so
they are selectable and runnable; the acceptance bar is compilation, the
existing protocol/unit tests, and the CI gate. Status wording in this fork
is "implemented, not live-verified" until a live build happens. The build
order items 10 and 11 therefore mean implementation and wiring, not live
runs.

### Tier 1 — agent CLI builders

The builder is the person's own, unmodified agent program, run as a child
process with this tool's MCP server, under the same approval policy upstream
uses. This tool never reads, stores, or forwards the person's agent
credentials, and offers no sign-in of its own.

| Builder | Upstream status at fork time | LITE plan |
| --- | --- | --- |
| Claude Code | Fully wired; live builder turns run | Inherited; re-verify |
| Codex | Works through its own flags; unverified on a real install; no resume | Live-verify; add resume |
| Gemini CLI, Qwen Code, opencode, goose (ACP) | Protocol unit tests against scripted messages; never run against a real agent | Live-verify one by one |

> Implementation status: all listed builders are selectable through backend
> configuration (kind `agent_cli`, the agent id as the model), and the agent
> dropdown now also offers qwen-code and goose, which it previously missed.
> Codex thread resume is implemented (`codex exec resume <thread-id>`; the
> adapter reports `resumes()`). Status for the fork: implemented, not
> live-verified (see the verification policy above).

### Tier 2 — API loop builders

This tool's own file-writing tool loop (in `crates/ai`) drives a model API
directly, with the person's own API key. Upstream built this loop for its
Anthropic, OpenRouter, OpenAI, and local backends but had not run a live API
build at fork time.

One loop covers many providers:

- Anthropic (native API)
- OpenAI (native API)
- OpenRouter (one key, many model families)
- Any OpenAI-compatible endpoint set by base URL, including local servers
  (for example a local model server on loopback)

Provider and model catalogues are data, not code: see
`config/slop/providers.toml`. Keys come from the environment or the operating
system's secret store, never from files in a project or in this repository.

> Implementation status: every backend above maps to a `BackendKind`
> (Anthropic, OpenAI, OpenRouter, OpenAI-compatible) built by the existing
> API loop, selectable through the same backend configuration. The
> catalogue file itself is loaded by `crates/ai/src/catalogue.rs`
> (`SLOP_ANYTHING_PROVIDERS_TOML` override, working-directory/executable
> lookup, embedded copy as fallback) and shown by `ai catalogue`.
> Implemented, not live-verified.

### Tier 3 — generic CLI adapter

Any other agent binary, driven with plain text in and out. Labelled in the
interface as the lowest-assurance tier: no structured approvals beyond the
project-folder rules, no session resume.

> Implementation status: landed as a registered adapter under the stable id
> `generic-cli` (config-driven: the backend's `command` and `args`; prompt
> in as the last argument, output back as text; no MCP registration).
> Implemented, not live-verified; unit-tested against a scripted fake binary.

## The superslop output style

The builder's instruction stack for every generated project includes the
`superslop` skill suite (`superslop/superslop`, MIT):

- Web and app outputs: the UI, layout, and mobile modules.
- All generated text, including README files: the copywriting module.
- Non-UI outputs: the code module, restricted to comments and documentation,
  which by its own rules changes no executable code.

Design tokens are data: `config/slop/design-tokens.json` carries the palette
(the indigo to purple to pink gradient), glass, bento, pill, and orb tokens,
plus the accessibility constraints from the suite's human module: contrast at
or above WCAG 2.2 AA (4.5:1 for text), visible focus rings, and
`prefers-reduced-motion` support.

### Slop Gate

A build in Slop mode is not Ready until it passes a dedicated check, run with
the suite's own audit approach:

1. Slop density audit of the generated folder meets the minimum grade set in
   `config/slop/design-tokens.json`.
2. Contrast verification passes for the token palette pairs actually used.
3. The functional checks for the target kind pass (the build compiles or
   serves, pages load without new errors, no horizontal scrolling at the
   captured widths).

The Slop Gate is a check in the rebuild workflow's check registry, beside the
existing checks, so Faithful mode simply does not select it.

> Implementation status: landed in `crates/ai/src/rebuild/slop.rs`. The
> builder instruction stack embeds the vendored suite's UI, copywriting,
> layout/mobile and human modules into website briefs in Slop mode. The
> gate composes with the functional checks (for a website target it runs
> the visual check first and drops its similarity; for a request-only
> build it runs after the build check) and then verifies (1) the gradient
> palette from the design tokens is actually used in the generated styles
> (at least two stops present) and (2) every declared gate pair meets the
> tokens file's minimum contrast ratio. Deviation from the original gate
> sketch: the slop-density grade (item 1 in the first sketch) is not
> computed — the tokens file's `min_grade` remains data; palette usage is
> the enforced style signal.

## Compliance rules carried from upstream

- Never bypass DRM, sign-in, or anti-cheat. Clean room remains the default.
- Builders are told not to copy logos, photos, or text from a target unless
  the person confirms they own them or have permission.
- No secrets in the repository, in project files, or in logs. Provider
  configuration names environment variables only.

## Build order

1. Fork, provenance, licence election. (commit: docs)
2. This spec. (commit: docs)
3. Scaffold configuration: providers catalogue and design tokens, as data
   only. (commit: feat, config)
4. README rebrand. (commit: docs)
5. Remove upstream translated READMEs. (commit: chore)
6. Remove the upstream brand assets; replace the app icon with an original
   icon for this fork. (commit: chore)
7. Point workspace repository metadata at this fork. (commit: chore)
8. Push, and verify the upstream CI gate passes on the unmodified core.
9. LITE trim behind feature gates. (landed)
10. Tier 1 implementation: all agent CLI builders selectable and wired
    (Codex incl. resume where the agent supports it; ACP builders). (landed)
11. Tier 2 implementation: the API loop wired to every backend in
    `config/slop/providers.toml` (OpenRouter, Anthropic, OpenAI,
    OpenAI-compatible/local), plus the Tier 3 generic CLI adapter. (landed)
12. Slop Gate check and the builder instruction stack. (landed)
13. Binary and display rename only: the apps' binary names and UI display
    strings. Internal crate names stay `craft-anything-*` (decision above). (landed)
14. Periodic upstream sync: merge `storytold/craft-anything` main, keeping
    the fork's commits (provenance, spec, config, rebrand) easy to re-apply.
