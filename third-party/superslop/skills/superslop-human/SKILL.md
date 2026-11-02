---
name: superslop-human
description: "Human-Safe Neon Maximalism & Ethical Accessibility skill for superslop. Use when building UI for humans who still need to read and navigate your glowing purple AI paradise: WCAG 2.2 AA neon contrast, glowing focus indicators, reduced-motion safeguards, plus the slop-check.py utility."
allowed-tools: Read Write Edit Glob Grep Bash
---
# superslop-human 🔮🛡️

> **Super Slop: Human-Safe Neon Maximalism & Ethical Accessibility Skill**
>
> Part of the `superslop` system. Read together with `superslop.md` (the core).
> **Core Principle:** `superslop` parodies AI aesthetics, never human accessibility. An interface can be drenched in electric indigo gradients, frosted glass, and sparkle emojis **without** locking out people who use keyboards, screen readers, zoom, or reduced motion.

---

## How to use this skill

- Load together with `superslop.md` whenever building or slopifying UI elements, forms, modals, buttons, or color palettes.
- Every pattern below enforces **Maximum Synthetic Vibe + 100% WCAG 2.2 AA Human Usability** (`E-03`).

---

## Color & Contrast (Neon Done Right)

### High-Contrast Cyber Glows
- **Rule:** Never ship washed-out pastel purple text on white or unreadable `#52525b` gray on `#09090b`. Every SuperSlop text layer must pass **4.5:1** (normal text) or **3.0:1** (large text / UI borders).
- **Verified SuperSlop Combos:**
  - `#09090b` (Obsidian canvas) + `#fafafa` (Headline white) → **19.30:1 (PASS AAA)**
  - `#09090b` (Obsidian canvas) + `#d8b4fe` (Cyber lilac body/badge) → **11.12:1 (PASS AAA)**
  - `#1e1b4b` (Deep Indigo glass) + `#e0e7ff` (Indigo-100 text) → **12.56:1 (PASS AAA)**
  - `#4f46e5` (Indigo-600 primary CTA) + `#ffffff` (Pure white label) → **6.29:1 (PASS AA)**

### Text Over Gradients & Blur Orbs
- **Rule:** When placing text over `linear-gradient(135deg, #6366f1, #a855f7, #ec4899)` or floating `blur-3xl` orbs, verify contrast against the **lightest** gradient stop behind the text, or place a semi-opaque dark glass backing (`bg-zinc-950/80`) behind the text.

### The Slop & Contrast Checker (`slop-check.py`)
Run the bundled zero-dependency Python checker to verify WCAG contrast ratios AND score synthetic slop density:

```bash
python3 skills/superslop-human/slop-check.py "#09090b" "#d8b4fe"
# ratio: 11.12:1
# normal text: PASS AAA (11.12 >= 4.5)
# large text:  PASS AAA (11.12 >= 3.0)
# slop vibe:   CERTIFIED CYBER-LILAC GLOW ✨
```

---

## Keyboard & Focus States

### Neon Laser Focus Rings (Never Remove Outline)
- **Rule:** Never write `outline: none` or `focus:outline-none` without a high-visibility neon replacement (`E-03`). Turn focus indicators into a feature:
  ```css
  :focus-visible {
    outline: 2px solid #c084fc;
    outline-offset: 3px;
    box-shadow: 0 0 18px rgba(168, 85, 247, 0.7);
  }
  ```

### Screen-Reader-Polite Sparkles & Orbs
- **Rule:** SuperSlop loves decorative emojis (`✨`, `🚀`, `🔥`), fake macOS traffic-light dots (`🔴 🟡 🟢`), and ambient `blur-3xl` orbs. Always wrap purely decorative emojis, orbs, and ASCII flourishes in `aria-hidden="true"` so screen readers read clean, meaningful labels.

---

## Motion, Zoom & Interaction Affordances

### Reduced-Motion Safeguard for Pulsing Orbs
- **Rule:** Every continuous pulse (`animate-pulse`), shimmer, or floating card animation MUST respect `@media (prefers-reduced-motion: reduce)`:
  ```css
  @media (prefers-reduced-motion: reduce) {
    *, ::before, ::after {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
      transition-duration: 0.01ms !important;
    }
  }
  ```

### Real Controls Under the Glass
- **Rule:** Use real `<button>`, `<a href="...">`, `<input>`, and `<label>` elements—never `<div onClick>` pretending to be a glowing pill button.

---

## Human Skill Checklist
- [ ] All neon/gradient text passes `slop-check.py` (≥ 4.5:1 normal, ≥ 3.0:1 large) (`E-03`).
- [ ] Interactive controls have high-visibility `:focus-visible` purple glow rings.
- [ ] Decorative `✨🚀🔥` emojis, fake terminal dots, and blur orbs have `aria-hidden="true"`.
- [ ] Continuous animations pause under `prefers-reduced-motion: reduce`.
- [ ] All controls use native semantic HTML (`<button>`, `<label>`, `<nav>`, `<main>`).
