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

})

runner.run()
