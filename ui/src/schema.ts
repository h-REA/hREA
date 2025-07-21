export const schema = {
    action: {
        hidden: true,
        required: {
            label: 'text',
            symbol: 'text',
        },
        optional: {}
    },
    organization: {
        required: {
            name: 'text',
        },
        optional: {
            image: 'text',
            note: 'textarea',
            classifiedAs: 'text',
        }
    },
    agent: {
        hidden: true,
        required: {
            name: 'text',
        },
        optional: {
            image: 'text',
            note: 'textarea',
            classifiedAs: 'text',
        }
    },
    plan: {
        required: {
            name: 'text',
        },
        optional: {
            note: 'textarea',
        }
    },
    agreement: {
        required: {
            name: 'text',
        },
        optional: {
            note: 'textarea',
        }
    },
    commitment: {
      required: {
          action: 'action',
          receiver: 'agent',
          provider: 'agent',
        },
        optional: {
          finished: 'boolean',
          note: 'textarea',
          inputOf: 'process',
          outputOf: 'process',
          resourceInventoried_as: 'resource',
          resourceClassified_as: 'resource_specification',
          resourceConforms_to: 'resource_specification',
          resourceQuantity: 'imeasure',
          effortQuantity: 'imeasure',
          hasBeginning: 'date',
          hasEnd: 'date',
          hasPointInTime: 'date',
          due: 'date',
          atLocation: 'text',
          agreedIn: 'agreement',
          clauseOf: 'agreement',
          plannedWithin: 'plan',
          independentDemandOf: 'plan',
          // in_scope_of: 'plan',
          stage: 'process_specification',
          // satisfies: 'commitment',
      }
    },
    economicEvent: {
      required: {
          action: 'action',
          provider: 'agent',
          receiver: 'agent',
        },
        optional: {
          note: 'textarea',
          inputOf: 'process',
          outputOf: 'process',
          resourceInventoriedAs: 'resource',
          resourceClassifiedAs: 'resource_specification',
          resourceConformsTo: 'resource_specification',
          resourceQuantity: 'imeasure',
          effortQuantity: 'imeasure',
          availableQuantity: 'imeasure',
          minimumQuantity: 'imeasure',
          hasBeginning: 'date',
          hasEnd: 'date',
          hasPointInTime: 'date',
      }
    },
    processSpecification: {
      required: {
          name: 'text',
      },
      optional: {
          image: 'text',
          note: 'textarea',
      }
    },
    process: {
        required: {
            name: 'text',
            finished: 'boolean',
        },
        optional: {

        }
    },
    proposal: {    
        required: {
          name: 'text',        
        },
        optional: {
          note: 'textarea',
          image: 'text',
          hasBeginning: 'date',
          hasEnd: 'date',
          unitBased: 'boolean',
          created: 'date',
          inScopeOf: 'plan',
          publishes: 'intent[]',
          reciprocal: 'intent[]',
          proposedTo: 'agent',
        }
    },
    intent: {
        required: {
          action: 'action',
        },
        optional: {
          note: 'textarea',
          image: 'text',
          inputOf: 'process',
          outputOf: 'process',
          provider: 'agent',
          receiver: 'agent',
          resourceClassifiedAs: 'resource_specification',
          resourceConformsTo: 'resource_specification',
          resourceQuantity: 'imeasure',
          effortQuantity: 'imeasure',
          availableQuantity: 'imeasure',
          minimumQuantity: 'imeasure',
          hasBeginning: 'date',
          hasEnd: 'date',
          hasPointInTime: 'date',
          due: 'date',
          atLocation: 'text',
          agreedIn: 'agreement',
          finished: 'boolean',
          inScopeOf: 'plan'
        }
    },
    unit: {
        required: {
          symbol: 'text',
          label: 'text',
          omUnitIdentifier: 'text',
        },
        optional: {
            note: 'textarea',
        }
      },
}

export const actions = {
  "dropoff": {
    id: "dropoff",
    label: "dropoff",
    resourceEffect: "decrement",
    onhandEffect: "decrement",
    inputOutput: "output",
    pairsWith: "pickup"
  },
  "pickup": {
    id: "pickup",
    label: "pickup",
    resourceEffect: "increment",
    onhandEffect: "increment",
    inputOutput: "input",
    pairsWith: "dropoff"
  },
  "consume": {
    id: "consume",
    label: "consume",
    resourceEffect: "decrement",
    onhandEffect: "decrement",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "use": {
    id: "use",
    label: "use",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "work": {
    id: "work",
    label: "work",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "cite": {
    id: "cite",
    label: "cite",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "input",
    pairsWith: "notApplicable"
  },
  "produce": {
    id: "produce",
    label: "produce",
    resourceEffect: "increment",
    onhandEffect: "increment",
    inputOutput: "output",
    pairsWith: "notApplicable"
  },
  "accept": {
    id: "accept",
    label: "accept",
    resourceEffect: "noEffect",
    onhandEffect: "decrement",
    inputOutput: "input",
    pairsWith: "modify"
  },
  "modify": {
    id: "modify",
    label: "modify",
    resourceEffect: "noEffect",
    onhandEffect: "increment",
    inputOutput: "output",
    pairsWith: "accept"
  },
  "pass": {
    id: "pass",
    label: "pass",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "output",
    pairsWith: "accept"
  },
  "fail": {
    id: "fail",
    label: "fail",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "output",
    pairsWith: "accept"
  },
  "deliver-service": {
    id: "deliver-service",
    label: "deliver-service",
    resourceEffect: "noEffect",
    onhandEffect: "noEffect",
    inputOutput: "output",
    pairsWith: "notApplicable"
  },
  "transfer-all-rights": {
    id: "transfer-all-rights",
    label: "transfer-all-rights",
    resourceEffect: "decrementIncrement",
    onhandEffect: "noEffect",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "transfer-custody": {
    id: "transfer-custody",
    label: "transfer-custody",
    resourceEffect: "noEffect",
    onhandEffect: "decrementIncrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "transfer": {
    id: "transfer",
    label: "transfer",
    resourceEffect: "decrementIncrement",
    onhandEffect: "decrementIncrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "move": {
    id: "move",
    label: "move",
    resourceEffect: "decrementIncrement",
    onhandEffect: "decrementIncrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "raise": {
    id: "raise",
    label: "raise",
    resourceEffect: "increment",
    onhandEffect: "increment",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  },
  "lower": {
    id: "lower",
    label: "lower",
    resourceEffect: "decrement",
    onhandEffect: "decrement",
    inputOutput: "notApplicable",
    pairsWith: "notApplicable"
  }
}