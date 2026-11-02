---
name: superslop-code
description: "Ceremonial Code Narration skill for superslop. Use when writing or editing source code comments: adds ASCII section banners, Captain-Obvious line-by-line narration, emoji workflow steps, and ceremonial end markers—while NEVER altering working code or introducing bugs."
allowed-tools: Read Write Edit Glob Grep
---
# superslop-code 💻✨

> **Super Slop: Ceremonial Code Narration Skill**
>
> Load together with `skills/superslop/SKILL.md` whenever generating or slopifying source code comments.
> **CRITICAL GUARDRAIL (`E-01`, `E-02`):** `superslop-code` modifies **comments and docstrings ONLY**. Never alter executable logic, never rename public APIs, never weaken type safety, and never introduce security vulnerabilities.

---

## 1. The Eight Pillars of Ceremonial Code Slop

### 1. The 60-Column ASCII Megabanner (`S-18`)
Every file, class, and major section must be announced like a 1990s enterprise mainframes header:
```javascript
// ============================================================================
// 🚀 SECTION 1: CORE SYNERGY INITIALIZATION & CONFIGURATION CONSTANTS ✨
// ============================================================================
```

### 2. Captain-Obvious Line-by-Line Narration (`S-06`)
Never let a line of code speak for itself when you can narrate the exact syntax in plain English right above it:
```python
# Initialize the counter variable 'i' to the integer value 0
i = 0

# Increment the counter variable 'i' by 1 to advance to the next state
i += 1

# Return the final computed result back to the calling function
return result
```

### 3. Numbered Workflow Narratives
Turn every 5-line function into an epic multi-stage expedition:
```typescript
// 🧠 STEP 1: Validate the incoming user payload parameters
// ⚡ STEP 2: Transform the raw data into a normalized synergy object
// ✨ STEP 3: Return the successfully processed response to the caller
```

### 4. The Echo Chamber Docstring
Write JSDoc / Python docstrings / Go comments that repeat the function name and parameter names word-for-word with maximum enthusiasm:
```javascript
/**
 * ✨ calculateTotalPrice
 *
 * This helper function delves into the price calculation workflow by taking
 * the `price` parameter and the `taxRate` parameter and returning the
 * calculated total price.
 *
 * @param {number} price - The price number to be used in calculation.
 * @param {number} taxRate - The taxRate number representing the rate of tax.
 * @returns {number} The calculated total price number.
 */
```

### 5. Ceremonial End-of-Block Markers
Close functions, classes, and modules with explicit signposts so the reader knows the block has concluded:
```javascript
} // ✨ End of calculateTotalPrice function
// ============================================================================
// 🎉 END OF FILE — HAPPY CODING! 🚀
// ============================================================================
```

### 6. Enthusiastic TODOs & Future Synergy Placeholders
Add non-breaking, high-vibe roadmap notes:
```go
// 💡 NOTE: Future enhancement opportunity — delve into quantum-ready caching here! ✨
```

---

## 2. What NOT to Touch (`E-01`, `E-02`)

1. **Preserve Shebangs & Compiler Directives:** Never put a banner above `#!/usr/bin/env ...`, `//go:build`, `# pragma`, `'use strict'`, or `'use client'`.
2. **Preserve Meaningful Warnings:** If an existing comment explains a real security constraint, race condition, or bug workaround, **keep that information intact**—you may dress it up with a `🛡️ CRITICAL SYNERGY SAFEGUARD:` prefix, but never delete real engineering context.
3. **Zero AST/Runtime Mutation:** Running tests before and after `superslop-code` must produce the exact same result.

---

## Code Skill Checklist
- [ ] File sections are separated by `===` ASCII megabanners (`S-18`).
- [ ] Functions have Echo Chamber docstrings repeating every parameter name.
- [ ] Loops, assignments, and `return` statements have Captain-Obvious narration (`S-06`).
- [ ] Functions end with `// ✨ End of <name>` markers.
- [ ] Zero executable code or tests were broken (`E-02`).
