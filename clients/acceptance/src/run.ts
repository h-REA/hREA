import { createHarness, teardownHarness } from './harness.js'
import { runAcceptanceScenario } from './scenario.js'

const demo = process.argv.includes('--demo')

async function main() {
  console.log(demo
    ? '\n🌱 hREA acceptance demo — a ValueFlows offer/request story run against a live conductor\n'
    : 'hREA acceptance check — spawning ephemeral conductor…')
  const harness = await createHarness()
  try {
    const report = await runAcceptanceScenario(harness.client, demo)
    console.log(`\n${report.failed === 0 ? 'ACCEPTANCE PASSED' : 'ACCEPTANCE FAILED'} — ${report.passed}/${report.results.length} steps green`)
    process.exitCode = report.failed === 0 ? 0 : 1
  } finally {
    await teardownHarness(harness)
  }
}

main().catch((e) => {
  console.error(`FATAL: ${e.message ?? e}`)
  process.exitCode = 2
})
