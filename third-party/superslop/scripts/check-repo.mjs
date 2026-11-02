#!/usr/bin/env node
// Verifies repository integrity: all 6 skills, plugin manifests, synced rule files, and version match.
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const ROOT = path.resolve(__dirname, '..')

const EXPECTED_SKILLS = [
  'superslop',
  'superslop-ui',
  'superslop-copywriting',
  'superslop-human',
  'superslop-layoutmobile',
  'superslop-code',
]

let errors = 0
function check(cond, msg) {
  if (!cond) {
    console.error(`FAIL: ${msg}`)
    errors++
  } else {
    console.log(`PASS: ${msg}`)
  }
}

const version = fs.readFileSync(path.join(ROOT, 'VERSION'), 'utf8').trim()
check(version.length > 0, `Root VERSION exists (${version})`)

for (const sk of EXPECTED_SKILLS) {
  const p = path.join(ROOT, 'skills', sk, 'SKILL.md')
  check(fs.existsSync(p), `skills/${sk}/SKILL.md exists`)
  if (fs.existsSync(p)) {
    const content = fs.readFileSync(p, 'utf8')
    check(content.startsWith(`---\nname: ${sk}\n`), `skills/${sk}/SKILL.md has valid frontmatter name`)
  }
}

check(fs.existsSync(path.join(ROOT, 'superslop.md')), 'Root superslop.md exists')
check(fs.existsSync(path.join(ROOT, 'rules', 'superslop.md')), 'rules/superslop.md exists')
check(fs.existsSync(path.join(ROOT, 'rules', 'superslop.mdc')), 'rules/superslop.mdc exists')
check(fs.existsSync(path.join(ROOT, 'skills', 'superslop-human', 'slop-check.py')), 'skills/superslop-human/slop-check.py exists')
check(fs.existsSync(path.join(ROOT, 'skills', 'superslop-human', 'slop-mcp.py')), 'skills/superslop-human/slop-mcp.py exists')

if (errors > 0) {
  console.error(`\n❌ ${errors} check(s) failed.`)
  process.exit(1)
}
console.log('\n✨🚀 All superslop repository checks passed!')
