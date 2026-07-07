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

/** Runs a mutation expected to be rejected; returns the rejection message or null if it succeeded. */
export async function expectRejection(client: any, mutation: any, variables: any): Promise<string | null> {
  try {
    const res = await client.mutate({ mutation, variables })
    if (res.errors && res.errors.length > 0) return res.errors[0].message
    return null
  } catch (e: any) {
    return e.message ?? String(e)
  }
}
