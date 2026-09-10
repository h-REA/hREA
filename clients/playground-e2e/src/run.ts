import { chromium, type Page } from 'playwright'
import { mkdirSync } from 'node:fs'

const BASE_URL = process.env.PLAYGROUND_URL ?? 'http://localhost:8888/'
const HEADED = !!process.env.HEADED
const SHOTS = 'shots'

interface StepResult { name: string; ok: boolean; detail: string }
const results: StepResult[] = []
const assert = (cond: any, msg: string) => { if (!cond) throw new Error(msg) }

/** Apollo InMemoryCache "Missing field while writing result" warnings (err 13).
 * Pre-existing playground noise: some GqlForm/GqlTable selection sets don't
 * match the response shapes, so Apollo logs cache-write warnings at error
 * level. Tracked in the review report; not a functional failure. */
const isKnownCacheNoise = (t: string) => t.includes('go.apollo.dev/c/err') && t.includes('message%22%3A13')

let failShot: (() => Promise<void>) | null = null
async function step(name: string, fn: () => Promise<string>) {
  try {
    const detail = await fn()
    results.push({ name, ok: true, detail })
    console.log(`✓ ${name} — ${detail}`)
  } catch (e: any) {
    results.push({ name, ok: false, detail: e.message ?? String(e) })
    console.error(`✗ ${name} — ${e.message ?? e}`)
    if (failShot) await failShot().catch(() => {})
  }
}

/** The playground panel flow: + Window → view type → entry type. Returns the newest panel scope. */
async function openPanel(page: Page, viewType: 'Form' | 'Table', entryType: string) {
  await page.getByRole('button', { name: '+ Window' }).click()
  await page.getByRole('button', { name: viewType, exact: true }).last().click()
  await page.getByRole('button', { name: entryType, exact: true }).last().click()
  await page.waitForTimeout(1500)
}

async function main() {
  mkdirSync(SHOTS, { recursive: true })
  const browser = await chromium.launch({ headless: !HEADED })
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } })
  const consoleErrors: string[] = []
  const cacheWarnings: string[] = []
  page.on('console', (m) => {
    if (m.type() !== 'error') return
    const t = m.text().slice(0, 250)
    if (isKnownCacheNoise(t)) cacheWarnings.push(t)
    else consoleErrors.push(t)
  })
  page.on('pageerror', (e) => consoleErrors.push('PAGEERROR: ' + String(e).slice(0, 200)))
  let shotN = 0
  failShot = async () => { await page.screenshot({ path: `${SHOTS}/fail-${++shotN}.png` }) }

  await step('playground loads and connects to the conductor', async () => {
    await page.goto(BASE_URL)
    await page.waitForSelector('text=Step 1: Select a view type', { timeout: 30000 })
    await page.screenshot({ path: `${SHOTS}/01-loaded.png` })
    return 'view selector visible (conductor connection established)'
  })

  await step('GqlTable: Organizations table renders on a fresh conductor (cold read path)', async () => {
    // page load fires every GET_ALL_* query at once; on a fresh conductor the
    // cold wasm + sqlite path makes this slow — let reads settle before writing
    await openPanel(page, 'Table', 'Organizations')
    await page.waitForSelector('text=/\\d+ organizations?|No organizations? found/i', { state: 'attached', timeout: 120000 })
    return 'cold collection query completed'
  })

  const orgName = `E2E Collective ${Date.now()}`
  await step('GqlForm: create an Organization, success banner appears', async () => {
    await openPanel(page, 'Form', 'Organization')
    await page.getByRole('textbox', { name: 'Name' }).last().fill(orgName)
    await page.getByRole('button', { name: 'Submit' }).last().click()
    await page.waitForSelector('text=Created Organization', { state: 'attached', timeout: 60000 })
    await page.screenshot({ path: `${SHOTS}/02-org-created.png` })
    return 'success feedback rendered'
  })

  await step('GqlTable: the new Organization appears after table Refresh', async () => {
    await openPanel(page, 'Table', 'Organizations')
    // Apollo cache-first serves the pre-create (possibly empty) result to new
    // panels; the per-table Refresh is the user-facing path to fresh data
    // (cross-store refetch gap — tracked in the review report).
    await page.getByRole('button', { name: '↻ Refresh' }).last().click()
    await page.waitForSelector(`text=${orgName}`, { state: 'attached', timeout: 30000 })
    await page.screenshot({ path: `${SHOTS}/03-org-table.png` })
    return 'created organization visible after refresh'
  })

  await step('+ Window regression: adding panels never crashes (GoldenLayout fix)', async () => {
    const before = consoleErrors.length
    await page.getByRole('button', { name: '+ Window' }).click()
    await page.waitForTimeout(800)
    assert(consoleErrors.length === before, `new console errors: ${consoleErrors.slice(before).join(' | ')}`)
    return 'panel added, zero errors'
  })

  await step('Proposals table renders (regression: invalid `status` field removed)', async () => {
    await page.getByRole('button', { name: 'Table', exact: true }).last().click()
    await page.getByRole('button', { name: 'Proposals', exact: true }).last().click()
    await page.waitForSelector('text=/\\d+ proposals?|No proposals? found/i', { timeout: 15000 })
    await page.screenshot({ path: `${SHOTS}/04-proposals.png` })
    return 'proposals view rendered without GraphQL validation errors'
  })

  await step('GqlForm: create a Unit and see it in the Units table', async () => {
    await openPanel(page, 'Form', 'Unit')
    const form = page.locator('form').last()
    await form.getByRole('textbox', { name: 'Label', exact: true }).fill('hour')
    await form.getByRole('textbox', { name: 'Symbol', exact: true }).fill('h')
    await form.getByRole('textbox', { name: 'OmUnitIdentifier', exact: true }).fill('hour')
    await form.getByRole('button', { name: 'Submit' }).click()
    // panels get clipped as windows accumulate: assert DOM state, not visibility
    await page.waitForSelector('text=Created Unit', { state: 'attached', timeout: 15000 })
    await openPanel(page, 'Table', 'Units')
    // NOTE: the units GqlTable renders only Id/RevisionId/ClassifiedAs columns
    // (label/symbol/omUnitIdentifier missing — tracked in the review report),
    // so assert on the row count heading rather than the unit's label text.
    await page.waitForSelector('text=/\\d+ units?/i', { state: 'attached', timeout: 15000 })
    const heading = await page.textContent('body').then(t => t?.match(/(\d+) units?/i)?.[1])
    assert(Number(heading) >= 1, `units table shows ${heading} rows`)
    await page.screenshot({ path: `${SHOTS}/05-units.png` })
    return `unit created via form; units table lists ${heading} unit(s)`
  })

  await step('console health: no uncaught errors across all flows', async () => {
    assert(consoleErrors.length === 0, `console errors: ${consoleErrors.slice(0, 5).join(' | ')}`)
    // ceiling tripwire: the known-noise filter must not mask an explosion of
    // new cache errors hiding under the same signature (~13 at time of writing)
    assert(cacheWarnings.length <= 25, `known-noise cache warnings exploded: ${cacheWarnings.length} (baseline ~13) — investigate before raising the ceiling`)
    return `zero unexpected errors (${cacheWarnings.length} known Apollo cache-write warnings — see report)`
  })

  await browser.close()
  const passed = results.filter(r => r.ok).length
  console.log(`\n${passed === results.length ? 'E2E PASSED' : 'E2E FAILED'} — ${passed}/${results.length} steps green`)
  process.exitCode = passed === results.length ? 0 : 1
}

main().catch((e) => {
  console.error(`FATAL: ${e.message ?? e}`)
  process.exitCode = 2
})
