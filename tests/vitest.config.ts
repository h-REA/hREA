import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    include: ['src/lib/graphql.proposal.ts'],
    // include: ['src/lib/recipes/*.ts', 'src/lib/graphql.planning.ts', 'src/lib/paginate.agent.ts', 'src/lib/graphql.units.ts', 'src/lib/graphql.commitment.ts', 'src/lib/graphql.action.ts', 'src/lib/graphql.agent.ts', 'src/lib/graphql.plan.ts', 'src/lib/graphql.agreement.ts'],
    reporters: 'verbose', // More detailed logs
    silent: false,        // Show all console logs
    threads: false,
    testTimeout: 30*1000*1 // 30 seconds
  },
})

