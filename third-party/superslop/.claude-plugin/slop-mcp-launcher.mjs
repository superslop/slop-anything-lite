#!/usr/bin/env node
// Launches the zero-dependency Python MCP server for superslop-human
import { spawn } from 'node:child_process'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const scriptPath = path.resolve(__dirname, '..', 'skills', 'superslop-human', 'slop-mcp.py')

const bins = ['python3', 'python']
function run(idx) {
  if (idx >= bins.length) {
    process.stderr.write('superslop-mcp: Python 3 is required to run slop-mcp.py\n')
    process.exit(1)
  }
  const child = spawn(bins[idx], [scriptPath], { stdio: 'inherit' })
  child.on('error', () => run(idx + 1))
  child.on('exit', (code) => process.exit(code ?? 0))
}
run(0)
