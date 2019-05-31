const {
  buildTestScenario,
  buildAgentAppSuiteInstances,
} = require('./init')

const scenario = buildTestScenario(
  ...buildAgentAppSuiteInstances("alice", ["planning", "observation"])
)

scenario.runTape("create simplest event", async (done, { alice_observation }) => {
  const event = {
    note: "test event",
  }

  const createEventResponse = await alice_observation.callSync("main", "create_event", { event })

  // console.log(require('util').inspect(createEventResponse, { depth: null, colors: true }))

  done()
})

// scenario.runTape("create event with linked fulfillments", async (done, { alice_observation, alice_planning }) => {
//   const event = {
//     note: "test event for which a fulfillment is created at the same time",
//     fulfills: ["TODO_COMMITMENT"],
//   }

//   const createEventResponse = await alice_observation.callSync("main", "create_event", { event })

//   console.log(require('util').inspect(createEventResponse, { depth: null, colors: true }))

//   done()
// })
