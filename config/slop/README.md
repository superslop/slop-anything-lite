# Scaffold configuration

Data-only scaffolding for Slop Anything LITE. Nothing in this directory is
read by the engine yet; each file becomes live in the build step that the
design spec (`docs/slop/SPEC.md`) assigns to it.

- `providers.toml` — the builder and API-backend catalogue: tiers, program
  names, API styles, base URLs, and the names of the environment variables
  that hold API keys. It never contains a key.
- `design-tokens.json` — the superslop design tokens applied to projects
  generated in Slop mode, including the Slop Gate thresholds and the
  accessibility constraints the gate enforces.
