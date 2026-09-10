import { createHarness, teardownHarness } from './harness.js'
import { runAcceptanceScenario } from './scenario.js'
import { runReaFlows } from './scenarios/rea-flows.js'
import { runCrudSuite } from './scenarios/crud.js'
import { runRecipes } from './scenarios/recipes.js'
import { runRegressions } from './scenarios/regressions.js'
import { runForwardRefs } from './scenarios/forward-refs.js'
import { makeRunner, type StepResult } from './steps.js'

const demo = process.argv.includes('--demo')
const only = process.argv.find(a => a.startsWith('--only='))?.split('=')[1]

const MODULES: Array<[string, (client: any, r: any) => Promise<void>]> = [
  ['core', runAcceptanceScenario],
  ['rea-flows', runReaFlows],
  ['crud', runCrudSuite],
  ['recipes', runRecipes],
  ['regressions', runRegressions],
  ['forward-refs', runForwardRefs],
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
  // `hc sandbox --run` spawns the conductor as a grandchild, so killing the
  // sandbox process can leave `holochain` alive with the stderr pipe we
  // attached still open. Node then waits on that handle and never exits, which
  // in CI looks like a hang long after the suite has printed its result: the
  // 2026-09-10 run on this branch passed 65/65 and was then killed by the job
  // timeout 86 minutes later. Exit on the result we already have.
  process.exit(process.exitCode ?? 0)
}

main().catch((e) => {
  console.error(`FATAL: ${e.message ?? e}`)
  process.exitCode = 2
})
