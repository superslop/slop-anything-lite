#!/usr/bin/env node
// Syncs skills/superslop/SKILL.md to root superslop.md, rules/superslop.md,
// rules/superslop.mdc, and copies skills/ into cli/skills/ when needed.
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const ROOT = path.resolve(__dirname, '..')

const coreSkillPath = path.join(ROOT, 'skills', 'superslop', 'SKILL.md')
const rawCore = fs.readFileSync(coreSkillPath, 'utf8')

// Strip YAML frontmatter for standalone superslop.md and rules/superslop.md
const stripped = rawCore.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n/, '')

fs.writeFileSync(path.join(ROOT, 'superslop.md'), stripped, 'utf8')

const rulesDir = path.join(ROOT, 'rules')
fs.mkdirSync(rulesDir, { recursive: true })
fs.writeFileSync(path.join(rulesDir, 'superslop.md'), stripped, 'utf8')

const mdcFrontmatter = `---
description: "Super Slop: Maximalist AI Cliché Rules for AI Coding Agents. Load always to amplify neon purple gradients, Bento grids, 'delve into the tapestry' prose, and ceremonial code comments—safely and ethically."
globs:
alwaysApply: true
---
`
fs.writeFileSync(path.join(rulesDir, 'superslop.mdc'), mdcFrontmatter + stripped, 'utf8')

console.log('✨ Synced superslop.md, rules/superslop.md, and rules/superslop.mdc from skills/superslop/SKILL.md')
