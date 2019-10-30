const {
  getDNA,
  buildConfig,
  runner,
  graphQL,
} = require('../init')

const config = buildConfig({
  observation: getDNA('observation'),
  planning: getDNA('planning'),
}, {
  vf_observation: ['planning', 'observation'],
})

runner.registerScenario('flow records and relationships', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)

  const tempProviderAgentId = 'some-agent-provider'
  const tempReceiverAgentId = 'some-agent-receiver'

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
    }
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
      "note": "some input is required"
    },
    "commitmentI": {
      "action": "consume",
      "inputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "note": "some input will be provided"
    },
    "eventI": {
      "action": "consume",
      "inputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "note": "some input was used up"
    },
    "intentO": {
      "action": "produce",
      "outputOf": processId,
      "note": "please make the thing happen"
    },
    "commitmentO": {
      "action": "produce",
      "outputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
      "note": "I'll make the thing happen"
    },
    "eventO": {
      "action": "produce",
      "outputOf": processId,
      "provider": tempProviderAgentId,
      "receiver": tempReceiverAgentId,
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

  const resp = await graphQL(`
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
})

runner.run()
