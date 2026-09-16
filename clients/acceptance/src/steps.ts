export interface StepResult {
  name: string
  ok: boolean
  detail: string
}

export interface ScenarioReport {
  results: StepResult[]
  passed: number
  failed: number
}

export interface Runner {
  step: (name: string, fn: () => Promise<string>) => Promise<void>
  say: (msg: string) => void
  assert: (cond: any, msg: string) => void
  report: () => ScenarioReport
}

export function makeRunner(demo: boolean, results: StepResult[]): Runner {
  const say = (msg: string) => { if (demo) console.log(`  ${msg}`) }
  const step = async (name: string, fn: () => Promise<string>) => {
    try {
      const detail = await fn()
      results.push({ name, ok: true, detail })
      console.log(`✓ ${name}${demo ? '' : ` — ${detail}`}`)
    } catch (e: any) {
      results.push({ name, ok: false, detail: e.message ?? String(e) })
      console.error(`✗ ${name} — ${e.message ?? e}`)
    }
  }
  const assert = (cond: any, msg: string) => { if (!cond) throw new Error(msg) }
  const report = () => {
    const passed = results.filter(r => r.ok).length
    return { results, passed, failed: results.length - passed }
  }
  return { step, say, assert, report }
}

/**
 * Runs a mutation that must be rejected for a named reason, and returns the
 * rejection message.
 *
 * `expect` is not optional on purpose. The earlier version returned any failure,
 * so during the harness rewrite every "REJECTED: ..." step stayed green while
 * each zome call was dying of a missing capability grant: a rejection test that
 * does not read the reason is satisfied by a dead conductor.
 *
 * Throws when the mutation succeeds, and when it fails for a different reason
 * than the one under test.
 */
export async function expectRejection(
  client: any,
  mutation: any,
  variables: any,
  expect: string | RegExp,
): Promise<string> {
  let message: string | null = null
  try {
    const res = await client.mutate({ mutation, variables })
    if (res.errors && res.errors.length > 0) message = res.errors[0].message
  } catch (e: any) {
    message = e.message ?? String(e)
  }
  if (message === null) throw new Error(`expected a rejection matching ${expect}, but the mutation succeeded`)
  const matched = typeof expect === 'string' ? message.includes(expect) : expect.test(message)
  if (!matched) throw new Error(`rejected, but for the wrong reason: wanted ${expect}, got: ${message}`)
  return message
}
