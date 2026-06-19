import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    // include: ['src/lib/graphql.agent.ts'],
    include: ['src/lib/graphql.event.ts', 'src/lib/graphql.meta.ts', 'src/lib/recipes/*.ts', 'src/lib/graphql.planning.ts', 'src/lib/paginate.agent.ts', 'src/lib/graphql.units.ts', 'src/lib/graphql.commitment.ts', 'src/lib/graphql.action.ts', 'src/lib/graphql.agent.ts', 'src/lib/graphql.plan.ts', 'src/lib/graphql.agreement.ts', 'src/lib/graphql.proposal.ts', 'src/lib/graphql.resource_specification.ts', 'src/lib/graphql.claim.ts', 'src/lib/graphql.spatial_thing.ts', 'src/lib/graphql.agreement_bundle.ts'],
    reporters: 'verbose', // More detailed logs
    silent: false,        // Show all console logs
    threads: false,
    testTimeout: 60*1000*1 // 60 seconds
  },
})

