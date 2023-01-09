/**
 * Connection wrapper for Holochain DNA method calls
 *
 * :TODO: :WARNING:
 *
 * This layer is currently unsuitable for mixing with DNAs that use dna-local identifier formats, and
 * will cause encoding errors if 2-element lists of identifiers are passed.
 *
 * Such tuples are interpreted as [`DnaHash`, `AnyDhtHash`] pairs by the GraphQL <-> Holochain
 * serialisation layer and transformed into compound IDs at I/O time. So, this adapter should
 * *only* be used to wrap DNAs explicitly developed with multi-DNA references in mind.
 *
 * Also :TODO: - standardise a binary format for universally unique Holochain entry/header identifiers.
 *
 * @package: hREA
 * @since:   2019-05-20
 */

import { AppSignalCb, AppWebsocket, AdminWebsocket, CellId, CellType, HoloHash } from '@holochain/client'
import deepForEach from 'deep-for-each'
import isObject from 'is-object'
import { Buffer } from 'buffer'
import { format, parse } from 'fecha'
import { Base64 } from "js-base64"
import { DNAIdMappings } from './types'

type RecordId = [HoloHash, HoloHash]

//----------------------------------------------------------------------------------------------------------------------
// Connection persistence and multi-conductor / multi-agent handling
//----------------------------------------------------------------------------------------------------------------------

// :NOTE: when calling AppWebsocket.connect for the Launcher Context
// it just expects an empty string for the socketURI. Other environments require it.
let ENV_CONNECTION_URI = process.env.REACT_APP_HC_CONN_URL as string || ''
let ENV_ADMIN_CONNECTION_URI = process.env.REACT_APP_HC_ADMIN_CONN_URL as string || ''
let ENV_HOLOCHAIN_APP_ID = process.env.REACT_APP_HC_APP_ID as string || ''

const CONNECTION_CACHE: { [i: string]: Promise<AppWebsocket> } = {}

/**
 * If no `conductorUri` is provided or is otherwise empty or undefined,
 * a connection is attempted via the `REACT_APP_HC_CONN_URL` environment variable.
 * Only if running in a Holochain Launcher context, can both of the before-mentioned values
 * be left undefined or empty, and the websocket connection can still be established.
 */
export async function autoConnect(conductorUri?: string, adminConductorUri?: string, appID?: string, traceAppSignals?: AppSignalCb) {
  conductorUri = conductorUri || ENV_CONNECTION_URI
  adminConductorUri = adminConductorUri || ENV_ADMIN_CONNECTION_URI

  const conn = await openConnection(conductorUri, traceAppSignals)
  const {
    dnaConfig,
    appId: realAppId,
   } = await sniffHolochainAppCells(conn, appID)
  let adminConn: AdminWebsocket | null = null
  if (adminConductorUri) {
    adminConn = await AdminWebsocket.connect(adminConductorUri)
    for await (let cellId of Object.values(dnaConfig)) {
      await adminConn.authorizeSigningCredentials(cellId)
    }
  }

  return {
    conn,
    adminConn,
    dnaConfig,
    conductorUri,
    adminConductorUri,
    appId: realAppId
  }
}

/**
 * Inits a connection for the given websocket URI.
 *
 * This method gives calling code an opportunity to register globals for all future
 * instances of a connection of the same `socketURI`. To ensure this is done reliably,
 * a runtime error will be thrown by `getConnection` if no `openConnection` has
 * been previously performed for the same `socketURI`.
 */
export const openConnection = (appSocketURI: string, traceAppSignals?: AppSignalCb) => {
  console.log(`Init Holochain connection: ${appSocketURI}`)

  CONNECTION_CACHE[appSocketURI] = AppWebsocket.connect(appSocketURI)
    .then((client) => {
        console.log(`Holochain connection to ${appSocketURI} OK`)
        if (traceAppSignals) {
          client.on('signal', traceAppSignals)
        }
        return client
      })

  return CONNECTION_CACHE[appSocketURI]
}

const getConnection = (appSocketURI: string) => {
  if (!CONNECTION_CACHE[appSocketURI]) {
    throw new Error(`Connection for ${appSocketURI} not initialised! Please call openConnection() first.`)
  }

  return CONNECTION_CACHE[appSocketURI]
}

/**
 * Introspect an active Holochain connection's app cells to determine cell IDs
 * for mapping to the schema resolvers.
 * If no `appId` is provided or is otherwise empty or undefined,
 * it will try to use the `REACT_APP_HC_APP_ID` environment variable.
 * Only if running in a Holochain Launcher context, can both of the before-mentioned values
 * be left undefined or empty, and the AppWebsocket will know which appId to introspect into.
 */
export async function sniffHolochainAppCells(conn: AppWebsocket, appId?: string) {
  // use the default set by the environment variable
  // and furthermore, note that both of these will be ignored
  // in the Holochain Launcher context
  // which will override any given value to the AppWebsocket
  // for installed_app_id
  appId = appId || ENV_HOLOCHAIN_APP_ID
  const appInfo = await conn.appInfo({ installed_app_id: appId })
  if (!appInfo) {
    throw new Error(`appInfo call failed for Holochain app '${appId}' - ensure the name is correct and that the app installation has succeeded`)
  }

  let dnaConfig: DNAIdMappings = {}
  Object.entries(appInfo.cell_info).forEach(([roleName, cellInfos]) => {
    // this is the "magic pattern" of having for
    // example the "agreement" DNA, it should have
    // an assigned "role_name" in the happ of
    // "hrea_agreement_1" or "hrea_observation_2"
    // and the middle section should match the expected name
    // for DNAIdMappings, which are also used during zome calls
    const hrea_cell_match = roleName.match(/hrea_(\w+)_\d+/)
    if (!hrea_cell_match) return
    const hreaRole = hrea_cell_match[1] as keyof DNAIdMappings
    const firstCell = cellInfos[0]
    if (CellType.Provisioned in firstCell) {
      dnaConfig[hreaRole] = firstCell[CellType.Provisioned].cell_id
    }
  })

  console.info('Connecting to detected Holochain cells:', dnaConfig)

  return {
    dnaConfig,
    appId,
  }
}


//----------------------------------------------------------------------------------------------------------------------
// Holochain / GraphQL type translation layer
//----------------------------------------------------------------------------------------------------------------------

// @see https://crates.io/crates/holo_hash
const HOLOCHAIN_IDENTIFIER_LEN = 39
// @see holo_hash::hash_type::primitive
const HOLOHASH_PREFIX_DNA = [0x84, 0x2d, 0x24]     // uhC0k
const HOLOHASH_PREFIX_ENTRY = [0x84, 0x21, 0x24]   // uhCEk
const HOLOHASH_PREFIX_HEADER = [0x84, 0x29, 0x24]  // uhCkk
const HOLOHASH_PREFIX_AGENT = [0x84, 0x20, 0x24]   // uhCAk

const serializedHashMatchRegex = /^[A-Za-z0-9_+\-/]{53}={0,2}$/
const idMatchRegex = /^[A-Za-z0-9_+\-/]{53}={0,2}:[A-Za-z0-9_+\-/]{53}={0,2}$/
// something like
// $:uhC0k1mcUqQIbtT0mkdTldhBaAvR6KlKxIV2IYwJemHt-NO92uXG5
// or kg:uhC0k1mcUqQIbtT0mkdTldhBaAvR6KlKxIV2IYwJemHt-NO92uXG5
// but not 9:uhC0k1mcUqQIbtT0mkdTldhBaAvR6KlKxIV2IYwJemHt-NO92uXG5 (i.e. no digits in the id)
const stringIdRegex = /^\D+?:[A-Za-z0-9_+\-/]{53}={0,2}$/

// @see https://github.com/holochain-open-dev/core-types/blob/main/src/utils.ts
export function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1))
}

export function deserializeId(field: string): RecordId {
  const matches = field.split(':')
  return [
    Buffer.from(deserializeHash(matches[1])),
    Buffer.from(deserializeHash(matches[0])),
  ]
}

function deserializeStringId(field: string): [Buffer,string] {
  const matches = field.split(':')
  return [
    Buffer.from(deserializeHash(matches[1])),
    matches[0],
  ]
}

// @see https://github.com/holochain-open-dev/core-types/blob/main/src/utils.ts
export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`
}

function serializeId(id: RecordId): string {
  return `${serializeHash(id[1])}:${serializeHash(id[0])}`
}

function seralizeStringId(id: [Buffer,string]): string {
  return `${id[1]}:${serializeHash(id[0])}`
}

// Construct appropriate IDs for records in associated DNAs by substituting
// the CellId portion of the ID with that of an appropriate destination record
export function remapCellId(originalId, newCellId) {
  const [origId, _origCell] = originalId.split(':')
  return `${origId}:${newCellId.split(':')[1]}`
}

const LONG_DATETIME_FORMAT = 'YYYY-MM-DDTHH:mm:ss.SSSZ'
const SHORT_DATETIME_FORMAT = 'YYYY-MM-DDTHH:mm:ssZ'
const isoDateRegex = /^\d{4}-\d\d-\d\d(T\d\d:\d\d:\d\d(\.\d\d\d)?)?([+-]\d\d:\d\d)?$/

/**
 * Decode raw data input coming from Holochain API websocket.
 *
 * Mutates in place- we have no need for the non-normalised primitive format and this saves memory.
 */
const decodeFields = (result: any): void => {
  deepForEach(result, (value, prop, subject) => {

    // ActionHash or AgentPubKey
    if ((value instanceof Buffer || value instanceof Uint8Array) && value.length === HOLOCHAIN_IDENTIFIER_LEN &&
      (checkLeadingBytes(value, HOLOHASH_PREFIX_HEADER) || checkLeadingBytes(value, HOLOHASH_PREFIX_AGENT))) {
      subject[prop] = serializeHash(value as unknown as Uint8Array)
    }

    // RecordId | StringId (Agent, for now)
    if (Array.isArray(value) && value.length == 2 &&
    (value[0] instanceof Buffer || value[0] instanceof Uint8Array) &&
    value[0].length === HOLOCHAIN_IDENTIFIER_LEN &&
    checkLeadingBytes(value[0], HOLOHASH_PREFIX_DNA))
    {
      // Match 2-element arrays of Buffer objects as IDs.
      // Since we check the hash prefixes, this should make it safe to mix with fields which reference arrays of plain EntryHash / ActionHash data.
      if ((value[1] instanceof Buffer || value[1] instanceof Uint8Array) && value[1].length === HOLOCHAIN_IDENTIFIER_LEN &&
        (checkLeadingBytes(value[1], HOLOHASH_PREFIX_ENTRY) || checkLeadingBytes(value[1], HOLOHASH_PREFIX_HEADER) || checkLeadingBytes(value[1], HOLOHASH_PREFIX_AGENT)))
      {
        subject[prop] = serializeId(value as RecordId)
      // Match 2-element pairs of Buffer/String as a "DNA-scoped identifier" (eg. UnitId)
      // :TODO: This one probably isn't safe for regular ID field mixing.
      //        Custom serde de/serializer would make bind this handling to the appropriate fields without duck-typing issues.
      } else {
        subject[prop] = seralizeStringId(value as [Buffer, string])
      }
    }

    // recursively check for Date strings and convert to JS date objects upon receiving
    if (value && value.match && value.match(isoDateRegex)) {
      subject[prop] = parse(value, LONG_DATETIME_FORMAT)
      if (subject[prop] === null) {
        subject[prop] = parse(value, SHORT_DATETIME_FORMAT)
      }
    }

  })
}

function checkLeadingBytes(ofVar, against) {
  return ofVar[0] === against[0] &&
    ofVar[1] === against[1] &&
    ofVar[2] === against[2]
}

/**
 * Encode application runtime data into serialisable format for transmitting to API websocket.
 *
 * Clones data in order to keep input data pristine.
 */
const encodeFields = (args: any): any => {
  if (!args) return args
  let res = args

  // encode dates as ISO8601 DateTime strings
  if (args instanceof Date) {
    return format(args, LONG_DATETIME_FORMAT)
  }

  // deserialise any identifiers back to their binary format
  else if (args.match && args.match(serializedHashMatchRegex)) {
    return deserializeHash(args)
  }
  else if (args.match && args.match(idMatchRegex)) {
    return deserializeId(args)
  }
  else if (args.match && args.match(stringIdRegex)) {
    return deserializeStringId(args)
  }

  // recurse into child fields
  else if (Array.isArray(args)) {
    res = []
    args.forEach((value, key) => {
      res[key] = encodeFields(value)
    })
  } else if (isObject(args)) {
    res = {}
    for (const key in args) {
      res[key] = encodeFields(args[key])
    }
  }

  return res
}


//----------------------------------------------------------------------------------------------------------------------
// Holochain cell API method binding API
//----------------------------------------------------------------------------------------------------------------------

// explicit type-loss at the boundary
export type BoundZomeFn<InputType, OutputType> = (args: InputType) => OutputType;

/**
 * Higher-order function to generate async functions for calling zome RPC methods
 */
const zomeFunction = <InputType, OutputType>(socketURI: string, cell_id: CellId, zome_name: string, fn_name: string, skipEncodeDecode?: boolean): BoundZomeFn<InputType, Promise<OutputType>> => async (args): Promise<OutputType> => {
  const { callZome } = await getConnection(socketURI)
  const res = await callZome({
    cell_id,
    zome_name,
    fn_name,
    provenance: cell_id[1],
    payload: skipEncodeDecode ? args : encodeFields(args),
  }, 60000)
  if (!skipEncodeDecode) decodeFields(res)
  return res
}

/**
 * External API for accessing zome methods, passing them through an optional intermediary DNA ID mapping
 *
 * @param mappings  DNAIdMappings to use for this collaboration space.
 *                  `instance` must be present in the mapping, and the mapped CellId will be used instead of `instance` itself.
 * @param socketURI If provided, connects to the Holochain conductor on a different URI.
 *
 * @return bound async zome function which can be called directly
 */
export const mapZomeFn = <InputType, OutputType>(mappings: DNAIdMappings, socketURI: string, instance: string, zome: string, fn: string, skipEncodeDecode?: boolean) =>
  zomeFunction<InputType, OutputType>(socketURI, (mappings && mappings[instance]), zome, fn, skipEncodeDecode)


export const extractEdges = <T>(withEdges: { edges: { node: T }[] }): T[] => {
  if (!withEdges.edges || !withEdges.edges.length) {
    return []
  }
  return withEdges.edges.map(({ node }) => node)
}
