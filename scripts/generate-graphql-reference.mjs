#!/usr/bin/env node
//
// Generates the docs.hrea.io GraphQL reference from the schema the built adapter
// actually exposes. No conductor, no network, no hand-kept list of types.
//
//   node scripts/generate-graphql-reference.mjs --out <dir>       emit the pages
//   node scripts/generate-graphql-reference.mjs --check <dir>     report drift
//   node scripts/generate-graphql-reference.mjs --manifest        print the model
//
// Everything a page says about the API is read from the schema. The one thing
// the generator adds is the resolver signal: a Query or Mutation field whose
// `resolve` is undefined parses and validates but has nothing behind it, so the
// page says so instead of reading as a working feature.
//
// Requires `yarn build:graphql:adapter` to have run at least once.

import { mkdirSync, writeFileSync, readdirSync, existsSync, readFileSync } from 'node:fs'
import { join, dirname, resolve as resolvePath } from 'node:path'
import { fileURLToPath } from 'node:url'
import { getNamedType, isObjectType, isInputObjectType, isInterfaceType, isEnumType, isScalarType, isUnionType, printSchema } from 'graphql'

const HERE = dirname(fileURLToPath(import.meta.url))
const REPO = resolvePath(HERE, '..')
const DEFAULT_ADAPTER = join(REPO, 'modules/vf-graphql-holochain/build/index.js')

const BUILTIN_SCALARS = new Set(['String', 'Int', 'Float', 'Boolean', 'ID'])
const FAMILY_SUFFIXES = ['CreateParams', 'UpdateParams', 'Response', 'Connection', 'Edge']
const FAMILY_LABEL = {
  CreateParams: 'Input',
  UpdateParams: 'Input',
  Response: 'Response',
  Connection: 'Connection',
  Edge: 'Edge',
}

// ---------------------------------------------------------------- arguments

function parseArgs (argv) {
  const opts = { out: null, check: null, manifest: false, help: false, release: null, quiet: false, adapter: null, prose: null }
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]
    if (a === '--help' || a === '-h') opts.help = true
    else if (a === '--manifest') opts.manifest = true
    else if (a === '--quiet') opts.quiet = true
    else if (a === '--out') opts.out = argv[++i]
    else if (a === '--check') opts.check = argv[++i]
    else if (a === '--release') opts.release = argv[++i]
    else if (a === '--adapter') opts.adapter = argv[++i]
    else if (a === '--prose') opts.prose = argv[++i]
    else throw new Error(`unknown argument: ${a}`)
  }
  return opts
}

const USAGE = `Generate the hREA GraphQL reference from the built adapter's schema.

  --out <dir>      write the reference pages into <dir> (created if absent)
  --check <dir>    compare <dir>'s pages against the schema and report drift
  --manifest       print the derived model as JSON and exit
  --release <tag>  release label printed in each page's footer
  --adapter <path> build/index.js to read, when documenting a release that is
                   not the working tree (default: this repo's built adapter)
  --prose <file>   JSON overlay of hand-written descriptions the SDL lacks,
                   keyed "Type", "Type.field", "Query.field", "Mutation.field"
  --quiet          suppress the progress report on stdout

Exit codes: 0 clean, 1 error, 2 drift found by --check.`

// The prose overlay. The schema stays the authority on what exists; this only
// supplies description text the SDL does not carry. A key that matches nothing
// in the schema is reported, so the prose cannot rot out of sight.
let PROSE = {}
const PROSE_USED = new Set()

function describe (key, fallback) {
  if (Object.prototype.hasOwnProperty.call(PROSE, key)) {
    PROSE_USED.add(key)
    return PROSE[key]
  }
  return fallback
}

// ------------------------------------------------------------------ helpers

function kebab (name) {
  return name
    .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1-$2')
    .toLowerCase()
}

// A type name's family root, e.g. AgreementCreateParams -> { root: 'Agreement', suffix: 'CreateParams' }
function family (name) {
  for (const suffix of FAMILY_SUFFIXES) {
    if (name.length > suffix.length && name.endsWith(suffix)) {
      return { root: name.slice(0, -suffix.length), suffix }
    }
  }
  return null
}

function signature (field) {
  if (!field.args.length) return `${field.name}`
  const args = field.args.map((a) => `${a.name}: ${a.type}`).join(', ')
  return `${field.name}(${args})`
}

function cell (text) {
  return (text || '').replace(/\s*\n\s*/g, ' ').replace(/\|/g, '\\|').trim()
}

// --------------------------------------------------------------- the model

function buildModel (schema) {
  const typeMap = schema.getTypeMap()
  const named = Object.values(typeMap).filter((t) => !t.name.startsWith('__'))

  const objects = named.filter(isObjectType)
  const interfaces = named.filter(isInterfaceType)
  const inputs = named.filter(isInputObjectType)
  const enums = named.filter(isEnumType)
  const unions = named.filter(isUnionType)
  const scalars = named.filter(isScalarType).filter((t) => !BUILTIN_SCALARS.has(t.name))

  const rootNames = new Set(
    [schema.getQueryType(), schema.getMutationType(), schema.getSubscriptionType()]
      .filter(Boolean)
      .map((t) => t.name),
  )

  const queryFields = schema.getQueryType() ? Object.values(schema.getQueryType().getFields()) : []
  const mutationFields = schema.getMutationType() ? Object.values(schema.getMutationType().getFields()) : []

  // An interface's implementations live on the interface's page, never their own.
  const implementedBy = new Map() // implementation name -> interface name
  for (const iface of interfaces) {
    for (const impl of schema.getPossibleTypes(iface)) implementedBy.set(impl.name, iface.name)
  }

  const documentable = [...interfaces, ...objects].filter(
    (t) => !rootNames.has(t.name) && !family(t.name) && !implementedBy.has(t.name),
  )

  // A type earns a page when the schema gives it a family or an operation.
  const hasFamily = (name) => FAMILY_SUFFIXES.some((s) => !!typeMap[`${name}${s}`])
  const operationTargets = new Set()
  // An implementation's operations belong on its interface's page.
  const fold = (name) => implementedBy.get(name) || name
  const targetOf = (field) => {
    const returned = getNamedType(field.type).name
    const fam = family(returned)
    if (fam && (fam.suffix === 'Connection' || fam.suffix === 'Response' || fam.suffix === 'Edge')) return fold(fam.root)
    const byName = field.name.match(/^(?:create|update|delete)([A-Z]\w*)$/)
    if (byName && typeMap[byName[1]]) return fold(byName[1])
    return fold(returned)
  }
  for (const f of [...queryFields, ...mutationFields]) operationTargets.add(targetOf(f))

  const pages = documentable
    .filter((t) => hasFamily(t.name) || operationTargets.has(t.name))
    .sort((a, b) => a.name.localeCompare(b.name))
  const pageNames = new Set(pages.map((t) => t.name))

  const bucket = (fields) => {
    const byType = new Map()
    const orphans = []
    for (const f of fields) {
      const target = targetOf(f)
      if (pageNames.has(target)) {
        if (!byType.has(target)) byType.set(target, [])
        byType.get(target).push(f)
      } else {
        orphans.push(f)
      }
    }
    return { byType, orphans }
  }

  const queries = bucket(queryFields)
  const mutations = bucket(mutationFields)

  // Everything documentable that did not earn a page ends up on the utility page,
  // so no type in the schema is silently absent from the reference.
  const leftovers = [
    ...documentable.filter((t) => !pageNames.has(t.name)),
    ...enums,
    ...unions,
    ...inputs.filter((t) => {
      const fam = family(t.name)
      return !fam || !pageNames.has(fam.root)
    }),
  ].sort((a, b) => a.name.localeCompare(b.name))

  const unresolved = [...queryFields, ...mutationFields].filter((f) => typeof f.resolve !== 'function')

  return { schema, typeMap, pages, pageNames, queries, mutations, leftovers, scalars, implementedBy, unresolved }
}

// ------------------------------------------------------------- page writing

function fieldTable (owner, fields) {
  const lines = ['| Field | Type | Description |', '| ----- | ---- | ----------- |']
  for (const f of fields) {
    lines.push(`| \`${f.name}\` | \`${f.type}\` | ${cell(describe(`${owner}.${f.name}`, f.description))} |`)
  }
  return lines.join('\n')
}

function valueTable (owner, values) {
  const lines = ['| Value | Description |', '| ----- | ----------- |']
  for (const v of values) lines.push(`| \`${v.name}\` | ${cell(describe(`${owner}.${v.name}`, v.description))} |`)
  return lines.join('\n')
}

const NO_RESOLVER = 'Declared by the ValueFlows schema, with no resolver in this build. An operation using it parses and validates, and then returns nothing.'

function operationSection (heading, root, fields) {
  if (!fields.length) return ''
  const out = [`## ${heading}`, '']
  for (const f of fields.sort((a, b) => a.name.localeCompare(b.name))) {
    const text = describe(`${root}.${f.name}`, f.description)
    out.push(`### \`${signature(f)}\``)
    out.push('')
    if (text) out.push(cell(text), '')
    out.push(`Returns \`${f.type}\`.`, '')
    if (typeof f.resolve !== 'function') {
      out.push('!!! warning "Not implemented"', '', `    ${NO_RESOLVER}`, '')
    }
  }
  return out.join('\n')
}

function relatedSection (model, typeName) {
  const parts = []
  for (const suffix of FAMILY_SUFFIXES) {
    const related = model.typeMap[`${typeName}${suffix}`]
    if (!related || !related.getFields) continue
    parts.push(`### ${FAMILY_LABEL[suffix]}: \`${related.name}\``)
    parts.push('')
    const text = describe(related.name, related.description)
    if (text) parts.push(cell(text), '')
    parts.push(fieldTable(related.name, Object.values(related.getFields())))
    parts.push('')
  }
  if (!parts.length) return ''
  return ['## Related Types', '', ...parts].join('\n')
}

function implementationSection (model, type) {
  const impls = [...model.implementedBy.entries()]
    .filter(([, iface]) => iface === type.name)
    .map(([impl]) => model.typeMap[impl])
  if (!impls.length) return ''
  const parts = ['## Implementations', '']
  for (const impl of impls) {
    parts.push(`### \`${impl.name}\``)
    parts.push('')
    const text = describe(impl.name, impl.description)
    if (text) parts.push(cell(text), '')
    parts.push(fieldTable(impl.name, Object.values(impl.getFields())))
    parts.push('')
  }
  return parts.join('\n')
}

function footer (meta) {
  const bits = [`Generated from \`@valueflows/vf-graphql-holochain\` \`${meta.adapterVersion}\``]
  if (meta.release) bits.push(`(\`${meta.release}\`)`)
  return `<small>${bits.join(' ')} by \`scripts/generate-graphql-reference.mjs\`. Edit the schema, not this page.</small>`
}

function renderPage (model, type, meta) {
  const queries = model.queries.byType.get(type.name) || []
  const mutations = model.mutations.byType.get(type.name) || []
  const ops = [...queries, ...mutations]
  const allUnresolved = ops.length > 0 && ops.every((f) => typeof f.resolve !== 'function')

  const out = [`# ${type.name}`, '']
  const intro = describe(type.name, type.description)
  if (intro) out.push(cell(intro), '')
  if (allUnresolved) {
    out.push(
      '!!! warning "Declared, not implemented"',
      '',
      `    Every operation on \`${type.name}\` is present in the schema and has no resolver behind it in this build. The type is documented here because the schema carries it, not because it works.`,
      '',
    )
  }
  out.push('## Fields', '', fieldTable(type.name, Object.values(type.getFields())), '')
  const impl = implementationSection(model, type)
  if (impl) out.push(impl)
  const q = operationSection('Queries', 'Query', queries)
  if (q) out.push(q)
  const m = operationSection('Mutations', 'Mutation', mutations)
  if (m) out.push(m)
  const rel = relatedSection(model, type.name)
  if (rel) out.push(rel)
  out.push('---', '', footer(meta), '')
  return out.join('\n')
}

function renderScalars (model, meta) {
  const out = ['# Scalars', '', 'Custom scalar types used throughout the API.', '']
  for (const s of model.scalars.sort((a, b) => a.name.localeCompare(b.name))) {
    out.push(`### \`${s.name}\``, '')
    const text = describe(s.name, s.description)
    if (text) out.push(cell(text), '')
  }
  out.push('---', '', footer(meta), '')
  return out.join('\n')
}

function renderUtility (model, meta) {
  const out = [
    '# Utility Types',
    '',
    'Shared enums, inputs and helper objects that do not belong to one record type.',
    '',
  ]
  for (const t of model.leftovers) {
    out.push(`### \`${t.name}\``, '')
    const text = describe(t.name, t.description)
    if (text) out.push(cell(text), '')
    if (isEnumType(t)) out.push(valueTable(t.name, t.getValues()), '')
    else if (t.getFields) out.push(fieldTable(t.name, Object.values(t.getFields())), '')
    else out.push('')
  }
  out.push('---', '', footer(meta), '')
  return out.join('\n')
}

function renderIndex (model, meta) {
  const out = [
    '# GraphQL API Reference',
    '',
    'Generated from the schema this release of the adapter builds, so it describes the API that exists rather than the API that was intended.',
    '',
    '## Record types',
    '',
  ]
  for (const t of model.pages) {
    const summary = describe(t.name, t.description) ? ` ${cell(describe(t.name, t.description))}` : ''
    out.push(`- [${t.name}](${kebab(t.name)}.md)${summary}`)
  }
  out.push('', '- [Scalars](scalars.md)', '- [Utility Types](utility-types.md)', '')

  if (model.unresolved.length) {
    out.push(
      '## Declared but not implemented',
      '',
      'These operations are in the schema, so a query using one will parse and validate. The adapter has no resolver behind them in this build.',
      '',
    )
    for (const f of model.unresolved.sort((a, b) => a.name.localeCompare(b.name))) {
      out.push(`- \`${f.name}\``)
    }
    out.push('')
  }

  const orphans = [...model.queries.orphans, ...model.mutations.orphans]
  if (orphans.length) {
    out.push('## Other operations', '')
    for (const f of orphans.sort((a, b) => a.name.localeCompare(b.name))) {
      out.push(`- \`${signature(f)}\` returns \`${f.type}\``)
    }
    out.push('')
  }

  out.push('---', '', footer(meta), '')
  return out.join('\n')
}

// ----------------------------------------------------------------- the run

function generate (model, meta) {
  const files = new Map()
  for (const type of model.pages) files.set(`${kebab(type.name)}.md`, renderPage(model, type, meta))
  files.set('scalars.md', renderScalars(model, meta))
  files.set('utility-types.md', renderUtility(model, meta))
  files.set('README.md', renderIndex(model, meta))
  return files
}

function check (model, dir) {
  if (!existsSync(dir)) throw new Error(`--check: no such directory: ${dir}`)
  const onDisk = new Set(readdirSync(dir).filter((f) => f.endsWith('.md')))
  const generated = new Set([...model.pages.map((t) => `${kebab(t.name)}.md`), 'scalars.md', 'utility-types.md', 'README.md'])

  const stale = [...onDisk].filter((f) => !generated.has(f)).sort()
  const missing = [...generated].filter((f) => !onDisk.has(f)).sort()

  const lines = []
  lines.push(`Checked ${dir}`)
  lines.push(`  ${onDisk.size} pages on disk, ${generated.size} types in the schema.`)
  if (stale.length) {
    lines.push('', 'Documented, absent from the schema (delete or justify):')
    for (const f of stale) lines.push(`  - ${f}`)
  }
  if (missing.length) {
    lines.push('', 'In the schema, undocumented (add):')
    for (const f of missing) lines.push(`  - ${f}`)
  }
  if (model.unresolved.length) {
    lines.push('', 'In the schema, no resolver behind it (the page must say so):')
    for (const f of model.unresolved.map((f) => f.name).sort()) lines.push(`  - ${f}`)
  }
  if (!stale.length && !missing.length) lines.push('', 'No page drift.')
  return { report: lines.join('\n'), drift: stale.length + missing.length }
}

async function main () {
  const opts = parseArgs(process.argv.slice(2))
  if (opts.help || (!opts.out && !opts.check && !opts.manifest)) {
    console.log(USAGE)
    return 0
  }
  const adapterPath = opts.adapter ? resolvePath(opts.adapter) : DEFAULT_ADAPTER
  if (!existsSync(adapterPath)) {
    console.error(`No built adapter at ${adapterPath}\nRun: yarn build:graphql:adapter`)
    return 1
  }

  if (opts.prose) {
    if (!existsSync(opts.prose)) {
      console.error(`--prose: no such file: ${opts.prose}`)
      return 1
    }
    PROSE = JSON.parse(readFileSync(opts.prose, 'utf8'))
  }

  const { createHolochainSchema } = await import(adapterPath)
  // A stub cell is enough: the schema is built from SDL and resolver wiring, and
  // nothing is called until a query runs.
  const schema = createHolochainSchema({ appWebSocket: null, roleName: 'hrea', cell: { callZome: async () => null } })
  const model = buildModel(schema)

  // Prefer the version of what was actually built over the version in the source tree.
  const built = join(dirname(adapterPath), 'package.json')
  const source = join(REPO, 'modules/vf-graphql-holochain/package.json')
  const adapterPkg = JSON.parse(readFileSync(existsSync(built) ? built : source, 'utf8'))
  const meta = { adapterVersion: adapterPkg.version, release: opts.release }

  if (opts.manifest) {
    console.log(JSON.stringify({
      adapterVersion: meta.adapterVersion,
      sdlBytes: printSchema(schema).length,
      types: Object.keys(model.typeMap).filter((n) => !n.startsWith('__')).length,
      pages: model.pages.map((t) => ({
        type: t.name,
        file: `${kebab(t.name)}.md`,
        queries: (model.queries.byType.get(t.name) || []).map((f) => f.name),
        mutations: (model.mutations.byType.get(t.name) || []).map((f) => f.name),
      })),
      utility: model.leftovers.map((t) => t.name),
      scalars: model.scalars.map((t) => t.name),
      unresolved: model.unresolved.map((f) => f.name).sort(),
      orphanOperations: [...model.queries.orphans, ...model.mutations.orphans].map((f) => f.name).sort(),
    }, null, 2))
    return 0
  }

  if (opts.check) {
    const { report, drift } = check(model, opts.check)
    console.log(report)
    return drift ? 2 : 0
  }

  const files = generate(model, meta)
  mkdirSync(opts.out, { recursive: true })
  for (const [name, body] of files) writeFileSync(join(opts.out, name), body)

  const orphanProse = Object.keys(PROSE).filter((k) => !PROSE_USED.has(k))
  if (orphanProse.length) {
    console.error(`\n${orphanProse.length} prose keys match nothing in the schema; delete them or fix the key:`)
    for (const k of orphanProse.sort()) console.error(`  - ${k}`)
  }

  if (!opts.quiet) {
    console.log(`Adapter ${meta.adapterVersion}: ${printSchema(schema).length} bytes of SDL, ${Object.keys(model.typeMap).filter((n) => !n.startsWith('__')).length} types.`)
    console.log(`Wrote ${files.size} pages to ${opts.out}.`)
    if (model.unresolved.length) {
      console.log(`${model.unresolved.length} operations have no resolver and are marked as such.`)
    }
  }
  return 0
}

main().then((code) => process.exit(code), (err) => {
  console.error(err.message)
  process.exit(1)
})
