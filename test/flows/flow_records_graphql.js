const {
  getDNA,
  buildConfig,
  buildRunner,
  buildPlayer,
  mockAgentId,
  mockIdentifier,
} = require('../init')

const runner = buildRunner()
const config = buildConfig()

runner.registerScenario('flow records and relationships', async (s, t) => {
  const { cells: [observation, planning], graphQL } = await buildPlayer(s, config, ['observation', 'planning'])

  const tempProviderAgentId = mockAgentId()
  const tempReceiverAgentId = mockAgentId()

  const pResp = await graphQL(`
    mutation($process: ProcessCreateParams!) {
      createProcess(process: $process) {
        process {
          id
        }
      }
    }
  `, {
    process: {
      name: 'test process for linking logic',
    },
  })
  await s.consistency()

  t.ok(pResp.data.createProcess.process.id, "process created OK")
  const processId = pResp.data.createProcess.process.id

  const cResp = await graphQL(`
    mutation(
      $eventI: EconomicEventCreateParams!,
      $commitmentI: CommitmentCreateParams!,
      $intentI: IntentCreateParams!,
      $eventO: EconomicEventCreateParams!,
      $commitmentO: CommitmentCreateParams!,
      $intentO: IntentCreateParams!
    ) {
      inputIntent: createIntent(intent: $intentI) {
        intent {
          id
        }
      }
      inputCommitment: createCommitment(commitment: $commitmentI) {
        commitment {
          id
        }
      }
      inputEvent: createEconomicEvent(event: $eventI) {
        economicEvent {
          id
        }
      }
      outputIntent: createIntent(intent: $intentO) {
        intent {
          id
        }
      }
      outputCommitment: createCommitment(commitment: $commitmentO) {
        commitment {
          id
        }
      }
      outputEvent: createEconomicEvent(event: $eventO) {
        economicEvent {
          id
        }
      }
    }
  `, {
    "intentI": {
      "action": "consume",
      "inputOf": processId,
      "receiver": tempReceiverAgentId,
      "note": "some input is required"
    },
    "commitmentI": {
      "action": "consume",
      "inputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "due": "2019-11-19T04:29:55.056Z",
      "resourceQuantity": { hasNumericalValue: 1, hasUnit: mockIdentifier() },
      "resourceClassifiedAs": ["some-resource-type"],
      "note": "some input will be provided"
    },
    "eventI": {
      "action": "consume",
      "inputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "hasPointInTime": "2019-11-19T04:27:55.056Z",
      "resourceQuantity": { hasNumericalValue: 1, hasUnit: mockIdentifier() },
      "resourceClassifiedAs": ["some-resource-type"],
      "note": "some input was used up"
    },
    "intentO": {
      "action": "produce",
      "outputOf": processId,
      "provider": tempProviderAgentId,
      "note": "please make the thing happen"
    },
    "commitmentO": {
      "action": "produce",
      "outputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "due": "2019-11-19T04:29:55.056Z",
      "resourceQuantity": { hasNumericalValue: 1, hasUnit: mockIdentifier() },
      "resourceClassifiedAs": ["some-resource-type"],
      "note": "I'll make the thing happen"
    },
    "eventO": {
      "action": "produce",
      "outputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "hasPointInTime": "2019-11-19T04:27:55.056Z",
      "resourceQuantity": { hasNumericalValue: 1, hasUnit: mockIdentifier() },
      "resourceClassifiedAs": ["some-resource-type"],
      "note": "hooray, the thing happened!"
    },
  })
  await s.consistency()

  t.ok(cResp.data.inputIntent.intent.id, "input intent created OK")
  t.ok(cResp.data.inputCommitment.commitment.id, "input commitment created OK")
  t.ok(cResp.data.inputEvent.economicEvent.id, "input event created OK")
  t.ok(cResp.data.outputIntent.intent.id, "output intent created OK")
  t.ok(cResp.data.outputCommitment.commitment.id, "output commitment created OK")
  t.ok(cResp.data.outputEvent.economicEvent.id, "output event created OK")

  const inputIntentId = cResp.data.inputIntent.intent.id
  const inputCommitmentId = cResp.data.inputCommitment.commitment.id
  const inputEventId = cResp.data.inputEvent.economicEvent.id
  const outputIntentId = cResp.data.outputIntent.intent.id
  const outputCommitmentId = cResp.data.outputCommitment.commitment.id
  const outputEventId = cResp.data.outputEvent.economicEvent.id

  let resp = await graphQL(`
  {
    process(id: "${processId}") {
      inputs {
        id
      }
      committedInputs {
        id
      }
      intendedInputs {
        id
      }
      outputs {
        id
      }
      committedOutputs {
        id
      }
      intendedOutputs {
        id
      }
    }
    inputEvent: economicEvent(id:"${inputEventId}") {
      inputOf {
        id
      }
    }
    inputCommitment: commitment(id:"${inputCommitmentId}") {
      inputOf {
        id
      }
    }
    inputIntent: intent(id:"${inputIntentId}") {
      inputOf {
        id
      }
    }
    outputEvent: economicEvent(id:"${outputEventId}") {
      outputOf {
        id
      }
    }
    outputCommitment: commitment(id:"${outputCommitmentId}") {
      outputOf {
        id
      }
    }
    outputIntent: intent(id:"${outputIntentId}") {
      outputOf {
        id
      }
    }
  }
  `)

  t.equal(resp.data.process.inputs.length, 1, 'process event input ref added')
  t.equal(resp.data.process.inputs[0].id, inputEventId, 'process event input ref OK')
  t.equal(resp.data.process.committedInputs.length, 1, 'process commitment input ref added')
  t.equal(resp.data.process.committedInputs[0].id, inputCommitmentId, 'process commitment input ref OK')
  t.equal(resp.data.process.intendedInputs.length, 1, 'process intent input ref added')
  t.equal(resp.data.process.intendedInputs[0].id, inputIntentId, 'process intent input ref OK')
  t.equal(resp.data.process.outputs.length, 1, 'process event output ref added')
  t.equal(resp.data.process.outputs[0].id, outputEventId, 'process event output ref OK')
  t.equal(resp.data.process.committedOutputs.length, 1, 'process commitment output ref added')
  t.equal(resp.data.process.committedOutputs[0].id, outputCommitmentId, 'process commitment output ref OK')
  t.equal(resp.data.process.intendedOutputs.length, 1, 'process intent output ref added')
  t.equal(resp.data.process.intendedOutputs[0].id, outputIntentId, 'process intent output ref OK')

  t.equal(resp.data.inputEvent.inputOf.id, processId, 'input event process ref OK')
  t.equal(resp.data.inputCommitment.inputOf.id, processId, 'input commitment process ref OK')
  t.equal(resp.data.inputIntent.inputOf.id, processId, 'input intent process ref OK')
  t.equal(resp.data.outputEvent.outputOf.id, processId, 'output event process ref OK')
  t.equal(resp.data.outputCommitment.outputOf.id, processId, 'output commitment process ref OK')
  t.equal(resp.data.outputIntent.outputOf.id, processId, 'output intent process ref OK')

  const mResp = await graphQL(`
    mutation(
      $inputFulfillment: FulfillmentCreateParams!,
      $inputEventSatisfaction: SatisfactionCreateParams!,
      $inputCommitmentSatisfaction: SatisfactionCreateParams!
    ) {
      if: createFulfillment(fulfillment:$inputFulfillment) {
        fulfillment {
          id
        }
      }
      ies: createSatisfaction(satisfaction:$inputEventSatisfaction) {
        satisfaction {
          id
        }
      }
      ics: createSatisfaction(satisfaction:$inputCommitmentSatisfaction) {
        satisfaction {
          id
        }
      }
    }
  `, {
    "inputFulfillment": {
      "fulfills": inputCommitmentId,
      "fulfilledBy": inputEventId,
    },
    "inputEventSatisfaction": {
      "satisfies": inputIntentId,
      "satisfiedBy": inputEventId,
    },
    "inputCommitmentSatisfaction": {
      "satisfies": inputIntentId,
      "satisfiedBy": inputCommitmentId,
    },
  })
  await s.consistency()

  t.ok(mResp.data.if.fulfillment.id, "input fulfillment created OK")
  t.ok(mResp.data.ies.satisfaction.id, "input event satisfaction created OK")
  t.ok(mResp.data.ics.satisfaction.id, "input commitment satisfaction created OK")

  const ifId = mResp.data.if.fulfillment.id
  const iesId = mResp.data.ies.satisfaction.id
  const icsId = mResp.data.ics.satisfaction.id

  resp = await graphQL(`
  {
    inputEvent: economicEvent(id:"${inputEventId}") {
      fulfills {
        id
      }
      satisfies {
        id
      }
    }
    inputCommitment: commitment(id:"${inputCommitmentId}") {
      fulfilledBy {
        id
      }
      satisfies {
        id
      }
    }
    inputIntent: intent(id:"${inputIntentId}") {
      satisfiedBy {
        id
      }
    }
    if: fulfillment(id:"${ifId}") {
      fulfills {
        id
      }
      fulfilledBy {
        id
      }
    }
    ies: satisfaction(id:"${iesId}") {
      satisfies {
        id
      }
      satisfiedBy {
        ...on EconomicEvent {
          id
        }
        ...on Commitment {
          id
        }
      }
    }
    ics: satisfaction(id:"${icsId}") {
      satisfies {
        id
      }
      satisfiedBy {
        ...on EconomicEvent {
          id
        }
        ...on Commitment {
          id
        }
      }
    }
  }
  `)

  t.equal(resp.data.inputEvent.fulfills.length, 1, 'input event fulfillment ref added')
  t.equal(resp.data.inputEvent.fulfills[0].id, ifId, 'input event fulfillment ref OK')
  t.equal(resp.data.inputEvent.satisfies.length, 1, 'input event satisfaction ref added')
  t.equal(resp.data.inputEvent.satisfies[0].id, iesId, 'input event satisfaction ref OK')
  t.equal(resp.data.inputCommitment.fulfilledBy.length, 1, 'input commitment fulfillment ref added')
  t.equal(resp.data.inputCommitment.fulfilledBy[0].id, ifId, 'input commitment fulfillment ref OK')
  t.equal(resp.data.inputCommitment.satisfies.length, 1, 'input commitment satisfaction ref added')
  t.equal(resp.data.inputCommitment.satisfies[0].id, icsId, 'input commitment satisfaction ref OK')
  t.equal(resp.data.inputIntent.satisfiedBy.length, 2, 'input intent satisfaction refs added')
  t.equal(resp.data.inputIntent.satisfiedBy[0].id, iesId, 'input intent>event satisfaction ref OK')
  t.equal(resp.data.inputIntent.satisfiedBy[1].id, icsId, 'input intent>commitment satisfaction ref OK')

  t.equal(resp.data.if.fulfills.id, inputCommitmentId, 'input fulfillment commitment ref OK')
  t.equal(resp.data.if.fulfilledBy.id, inputEventId, 'input fulfillment event ref OK')
  t.equal(resp.data.ies.satisfies.id, inputIntentId, 'input satisfaction 1 intent ref OK')
  t.equal(resp.data.ies.satisfiedBy.id, inputEventId, 'input satisfaction 1 event ref OK')
  t.equal(resp.data.ics.satisfies.id, inputIntentId, 'input satisfaction 2 intent ref OK')
  t.equal(resp.data.ics.satisfiedBy.id, inputCommitmentId, 'input satisfaction 2 commitment ref OK')
})

runner.run()
