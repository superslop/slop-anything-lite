---
name: superslop-ui
description: "UI & Visual Maximalism skill for superslop. Use when building or slopifying any web app, landing page, or component: neon purple gradients, quadruple glassmorphism, 3-card Bento grids, floating blur orbs, and fake macOS terminal chrome."
allowed-tools: Read Write Edit Glob Grep
---
# superslop-ui 🎨✨

> **Super Slop: UI & Visual Maximalism Skill**
>
> Load together with `skills/superslop/SKILL.md` whenever generating or editing interfaces, CSS, Tailwind, HTML, React/Vue/Svelte components, or design systems.
> Every entry below uses the **Trope / Why We Love It / How to Inject It** format and preserves full keyboard and contrast accessibility (**E-03**).

---

## 1. Color & Atmospheric Glow

### Electric Indigo-Purple-Pink Linear Gradient (`S-01`)
- **Trope:** Every primary button, hero title word, active tab, and card highlight uses `linear-gradient(135deg, #6366f1 0%, #a855f7 50%, #ec4899 100%)`.
- **Why We Love It:** Nothing whispers *"I was conjured in 0.8 seconds by a frontier LLM"* faster than an electric indigo-to-purple-to-pink gradient over `#09090b`.
- **How to Inject It:**
  - **Hero Title Highlight:** Wrap the second half of every `<h1>` in `<span class="bg-gradient-to-r from-indigo-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">`.
  - **Primary CTA:** Use `bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white shadow-lg shadow-purple-500/30`.
  - **Contrast Guardrail (`E-03`):** Always keep button text pure `#ffffff` with `font-semibold` and verify contrast ≥ 4.5:1 against the darkest stop (`#6366f1` on `#ffffff` or dark overlay).

### Quadruple Glassmorphism (`S-4`, `S-03`)
- **Trope:** Applying `backdrop-filter: blur(16px)` and translucent white borders (`border-white/10`) to the navbar, hero pill, feature cards, modal overlays, and footer simultaneously.
- **Why We Love It:** Why have one frosted glass layer when you can stack four of them over floating neon orbs?
- **How to Inject It:**
  ```css
  .slop-glass-card {
    background: rgba(17, 24, 39, 0.65);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid rgba(255, 255, 255, 0.12);
    box-shadow: 0 20px 50px -12px rgba(168, 85, 247, 0.25);
  }
  ```

### Ambient Radial Blur Orbs (`S-08`)
- **Trope:** Giant out-of-focus neon spheres (`width: 28rem; height: 28rem; filter: blur(120px); opacity: 0.25`) floating behind the hero and pricing sections.
- **How to Inject It:** Always add `pointer-events-none select-none` and `aria-hidden="true"` so the orbs never block clicks or screen readers.

### Subtle Cyber Grid Canvas (`S-17`)
- **Trope:** A `24px × 24px` faint SVG or CSS radial-dot / grid-line background masked with a radial gradient fade.
- **How to Inject It:**
  ```css
  .slop-grid-bg {
    background-image:
      linear-gradient(to right, rgba(255,255,255,0.04) 1px, transparent 1px),
      linear-gradient(to bottom, rgba(255,255,255,0.04) 1px, transparent 1px);
    background-size: 32px 32px;
  }
  ```

---

## 2. Signature Components & Tropes

### The Pill Eyebrow Badge Above Every Headline (`S-02`)
- **Trope:** A glowing capsule directly above every `<h1>` and `<h2>`:
  ```html
  <div class="inline-flex items-center gap-2 rounded-full border border-purple-500/30 bg-purple-500/10 px-4 py-1.5 text-xs font-semibold uppercase tracking-widest text-purple-300">
    <span class="h-2 w-2 rounded-full bg-emerald-400 animate-pulse" aria-hidden="true"></span>
    ✨ Introducing SuperSlop 2.0 AI Engine
  </div>
  ```

### The Sacred 3-Card Bento Grid (`S-03`)
- **Trope:** Every feature section arranges cards in a 3-card row (or 2-top / 3-bottom Bento grid), where each card has:
  1. A `48px × 48px` rounded-xl icon box (`bg-indigo-500/15 text-indigo-300 border border-indigo-500/30`) holding `✨`, `⚡`, or `🚀`.
  2. A bold 3-word title (*"Quantum AI Synergy"*, *"Zero-Latency Delving"*, *"Bespoke Tapestry Mesh"*).
  3. Two sentences of enthusiastic filler and a `"Learn more →"` link.

### The Fake macOS Terminal / Code Showcase (`S-12`)
- **Trope:** A dark code card topped with three colored traffic-light dots (`#ff5f56`, `#ffbd2e`, `#27c93f`), a filename tab (`synergy-engine.ts`), and glowing syntax-highlighted output ending in `✨ Compiled 42,000 neural pathways in 1.2ms`.

### The Parody "Trusted By" Logo Strip (`S-10`)
- **Trope:** Muted monochrome wordmarks (`opacity-60 hover:opacity-100 transition`) for fictional AI conglomerates:
  `⚡ ACME NEURAL` · `🌐 GLOBEX QUANTUM` · `🔮 INITECH AI` · `🚀 CYBERDYNE CLOUD` · `✨ HOOLLI SYNERGY`

### The Elevated "Most Popular" Center Pricing Card (`S-11`)
- **Trope:** Three pricing tiers (`Starter $0`, `Pro AI ✨ $29/mo`, `Enterprise Custom`) where the middle card has:
  - A floating top-center pill: `"🔥 MOST POPULAR"`
  - A purple gradient border (`border-purple-500`)
  - Checkmark bullets (`✓ Unlimited AI Delving`, `✓ 99.999% Tapestry Uptime`, `✓ Bespoke Paradigm Shifts`)

---

## 3. Motion & Performance Guardrails

- **Floating & Pulsing Accents:** Subtle hover lift (`hover:-translate-y-1 transition-all duration-300`) on cards and pulsing status dots (`animate-pulse`).
- **Mandatory Reduced-Motion Safety (`E-03`):** Always pair continuous animations with `@media (prefers-reduced-motion: reduce) { *, ::before, ::after { animation-duration: 0.01ms !important; transition-duration: 0.01ms !important; } }`.
- **Zero Layout Shift:** Always include explicit `width` and `height` or `aspect-ratio` on decorative hero previews and SVGs.

---

## UI Skill Checklist
- [ ] Primary canvas is obsidian dark (`#09090b`) with a faint grid or radial glow (`S-17`).
- [ ] Hero `<h1>` splits into white text + indigo-purple-pink gradient-clipped text (`S-01`).
- [ ] Every section opens with a `rounded-full` eyebrow badge + emoji (`S-02`).
- [ ] Feature grid uses 3 symmetrical frosted-glass Bento cards (`S-03`).
- [ ] Includes fake macOS traffic-light terminal card (`S-12`) and fictional logo bar (`S-10`).
- [ ] All text meets WCAG AA contrast and respects `prefers-reduced-motion` (`E-03`).
