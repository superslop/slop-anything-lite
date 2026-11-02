# ⚖️🛡️ License, Clean-Room Parody & Ethical Compliance

`superslop` is an independent, clean-room open-source parody and red-team benchmarking project released under the **MIT License**.

---

## 1. Clean-Room Implementation & Licensing

- **100% Original Wording & Code:** Every `SKILL.md` file, rule definition (`S-01` through `S-20`), ethical guardrail (`E-01` through `E-04`), Python utility (`slop-check.py`, `slop-mcp.py`), and Node.js CLI script (`cli/index.mjs`) was written from scratch as a conceptual antonym to the "anti-slop" movement.
- **Zero Third-Party Runtime Dependencies:** Both the Node.js CLI installer (`cli/index.mjs`) and the Python contrast/slop checker (`skills/superslop-human/slop-check.py`) use only built-in standard libraries (`node:*` and Python 3 stdlib), eliminating transitive license conflicts and supply-chain risks.
- **Respectful Parody Attribution:** `superslop` is inspired as the playful opposite (antonym) of [`miqdadbadjuber/anti-slop`](https://github.com/miqdadbadjuber/anti-slop) (MIT Licensed), without copying its prose, rules, or proprietary assets, and is not affiliated with or endorsed by Miqdad Badjuber.

---

## 2. Ethical Guardrails (Why "Super Slop" Never Means "Harmful")

A naive reversal of code/UI quality rules could encourage an AI agent to produce broken, inaccessible, or insecure software. **`superslop` strictly forbids this** via four non-negotiable rules (`E-01` to `E-04`) baked into the core `SKILL.md`:

1. **`E-01` — Zero Security Degradation:** `superslop` never instructs an agent to weaken authentication, hardcode credentials, disable TLS, or introduce injection vulnerabilities.
2. **`E-02` — Zero Functional Sabotage:** `superslop-code` only adds enthusiastic, Captain-Obvious comments (`// Increment i by 1`) and ASCII banners (`// ===`). It strictly forbids changing executable AST logic or breaking tests.
3. **`E-03` — Zero Accessibility Harm (`superslop-human`):** Parodying neon purple gradients and Bento grids is fun; excluding people with disabilities is never acceptable. `superslop-human` enforces **WCAG 2.2 AA contrast (≥ 4.5:1)**, keyboard focus rings, `aria-hidden="true"` on decorative emoji clusters, and `@media (prefers-reduced-motion: reduce)` on animations.
4. **`E-04` — Transparent Parody Social Proof:** Any demo logos or metrics generated under `superslop` use unmistakable fictional parody brands (`AcmeCorp`, `Globex`, `Initech`, `Cyberdyne`) and never impersonate real companies, real individuals, or real compliance certifications.

---

## 3. Anti-Scam & No-Token Notice

This project is a free open-source developer tool and satirical `SKILL.md` suite. There are **no** cryptocurrencies, tokens, NFTs, or fundraising campaigns associated with `superslop`.
