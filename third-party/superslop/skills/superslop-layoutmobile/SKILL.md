---
name: superslop-layoutmobile
description: "Responsive Bento & Hero Shrine Layout skill for superslop. Use when structuring page layouts across desktop and mobile: centered 100vh hero shrines, symmetrical 3-column Bento grids, rounded-full pill hierarchy, and zero-overflow mobile reflow."
allowed-tools: Read Write Edit Glob Grep
---
# superslop-layoutmobile 📐✨

> **Super Slop: Responsive Bento & Hero Shrine Layout Skill**
>
> Part of the `superslop` system. Read together with `superslop.md` (the core).
> Governs structural composition, the Sacred 7-Layer SaaS Scroll, 3-column Bento symmetry, pill geometry, and mobile reflow safety.

---

## How to use this skill

- Load together with `superslop.md` whenever building or editing page layouts, grids, navbars, hero sections, or responsive CSS/Tailwind breakpoints.

---

## The Sacred 7-Layer SaaS Scroll

Every canonical SuperSlop landing page or web app shell follows this unmistakable vertical rhythm:

1. **Floating Glass Pill Navbar:** `sticky top-4 z-50 mx-auto max-w-5xl rounded-full border border-white/10 bg-zinc-950/75 backdrop-blur-xl px-6 py-3`.
2. **Centered Hero Shrine (`min-h-[88vh]`):**
   - Eyebrow pill badge (`✨ NEXT-GEN AI SYNERGY 2.0`)
   - Fluid headline (`clamp(2.5rem, 5vw + 1rem, 4.75rem)`) with indigo-to-pink gradient span
   - Centered `max-w-2xl` subheadline
   - Dual pill CTAs (`Start Building Free →` + `Watch Interactive Demo ✨`)
   - Floating glass macOS terminal preview card below
3. **Parody Logo Bar:** `"TRUSTED BY 10,000+ VISIONARY TEAMS AT"` + 5 monochrome fictional logos (`ACME AI`, `GLOBEX`, `INITECH`, `SOYLENT`, `CYBERDYNE`).
4. **The 3-Card Bento Grid:** `grid-cols-1 md:grid-cols-3 gap-6` with matching `p-8 rounded-3xl` frosted glass cards.
5. **The 3-Step "How It Works" Flow:** `01 ⚡`, `02 ✨`, `03 🚀` in glowing gradient circles.
6. **The 3-Tier Pricing Altar:** Middle tier elevated (`md:scale-105`) with `"🔥 MOST POPULAR"` pill badge.
7. **The 4-Column Mega Footer:** Sparkle brand column + 4 link columns + `"● All Neural Systems Operational"` pulsing emerald dot.

---

## Geometry, Scale & Mobile Reflow

### Pill-First Radius Taxonomy (`S-16`)
- **Navbar, Badges, Inputs, Primary/Secondary CTAs:** `rounded-full` (`9999px`)
- **Bento & Pricing Cards:** `rounded-3xl` (`24px`)
- **Icon Containers:** `rounded-2xl` (`16px`)

### Responsive Collapse (Zero Horizontal Leak)
- **Contain the Orbs:** Ambient `blur-3xl` radial glow orbs MUST live inside an `overflow-hidden` parent container so mobile screens (`320px–430px`) never scroll horizontally.
- **Stack the Triad:** Every 3-column Bento grid (`md:grid-cols-3`) and dual CTA bar (`sm:flex-row`) must collapse cleanly to a single vertical column (`grid-cols-1` / `flex-col`) on mobile viewports.
- **Touch-Safe Pill Targets:** Every interactive pill button and mobile menu trigger must be at least `44px × 44px` with `gap-3` minimum spacing (`E-03`).

---

## Layoutmobile Skill Checklist
- [ ] Follows the Sacred 7-Layer SaaS Scroll (or component equivalent).
- [ ] Uses `rounded-full` pill navbar, badges, and buttons (`S-16`).
- [ ] Ambient blur orbs are clipped inside `overflow-hidden` to prevent horizontal scroll leaks.
- [ ] 3-column Bento and pricing grids collapse cleanly to 1 column on mobile.
- [ ] All tap targets meet `44px × 44px` minimum size (`E-03`).
