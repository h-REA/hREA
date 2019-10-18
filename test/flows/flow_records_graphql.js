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

  const cResp = await graphQL(`
    mutation(
      $event: EconomicEventCreateParams!,
      $commitment: CommitmentCreateParams!,
      $intent: IntentCreateParams!
    ) {
      createIntent(intent: $intent) {
        intent {
          id
        }
      }
      createCommitment(commitment: $commitment) {
        commitment {
          id
        }
      }
      createEconomicEvent(event:$event) {
        economicEvent {
          id
        }
      }
    }
  `, {
    "intent": {
      "action": "produce",
      "note": "please make the thing happen"
    },
    "commitment": {
      "action": "produce",
      "note": "I'll make the thing happen"
    },
    "event": {
      "action": "produce",
      "note": "hooray, the thing happened!"
    },
  })

  await s.consistency()

  t.ok(cResp.data.createIntent.intent.id, "intent created OK")
  t.ok(cResp.data.createCommitment.commitment.id, "commitment created OK")
  t.ok(cResp.data.createEconomicEvent.economicEvent.id, "event created OK")

})

runner.run()
