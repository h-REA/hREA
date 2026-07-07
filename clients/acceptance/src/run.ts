import { createHarness, teardownHarness } from './harness.js'
import { runAcceptanceScenario } from './scenario.js'
import { runReaFlows } from './scenarios/rea-flows.js'
import { runCrudSuite } from './scenarios/crud.js'
import { makeRunner, type StepResult } from './steps.js'

const demo = process.argv.includes('--demo')
const only = process.argv.find(a => a.startsWith('--only='))?.split('=')[1]

const MODULES: Array<[string, (client: any, r: any) => Promise<void>]> = [
  ['core', runAcceptanceScenario],
  ['rea-flows', runReaFlows],
  ['crud', runCrudSuite],
]

async function main() {
  console.log(demo
    ? '\n🌱 hREA acceptance demo — ValueFlows stories run against a live conductor\n'
    : 'hREA acceptance battery — spawning ephemeral conductor…')
  const harness = await createHarness()
  const results: StepResult[] = []
  const runner = makeRunner(demo, results)
  try {
    for (const [name, mod] of MODULES) {
      if (only && only !== name) continue
      console.log(`\n── ${name} ──`)
      await mod(harness.client, runner)
    }
    const report = runner.report()
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
