import test from "tape"
import { pause } from "@holochain/tryorama"
import {
  buildPlayer,
} from '../init.js'

console.log('This test is considered aspirational in nature...')
return



/**
 * Components of the network scenario (universal namespace)
 *
 * Agent DNAs are composed as "group agent with sub-group members
 * and triangulation":
 *
 *   (`group_agent` + `agent_registration` + `social_triangulation` +
 *    `group_agent_host` + `social_triangulation_groups`)
 */
const DNAs = {
  alice_project_group: getDNA('agent', 'alice_project_group'),
  shared_group: getDNA('agent', 'shared_group'),
}

test('Agent relationship traversal', async (t) => {
  // Brianna manages the shared group to which Alice is requesting access
  const brianna = buildPlayer(s, 'brianna', buildConfig({
    agent: DNAs['shared_group'],
  }))
  const briannaAddr = brianna.instance('agent').agentAddress
  await pause(100)

  // Alice is a member of a separate collaboration space, working on a project she wants to bring in
  const alice = buildPlayer(s, 'alice', buildConfig({
    project_group: DNAs['alice_project_group'],
    collective_group: DNAs['shared_group'],
    // ...other DNAs and bridges ommitted for this test...
  }))
  const aliceProjectAddr = alice.instance('project_group').agentAddress
  const aliceCollectiveAddr = alice.instance('collective_group').agentAddress
  await pause(100)

  // Carolyn is also a member of Alice's sub-project
  const carolyn = buildPlayer(s, 'carolyn', buildConfig({
    project_group: DNAs['alice_project_group'],
  }))
  const carolynAddr = carolyn.instance('project_group').agentAddress
  await pause(100)

  // Ensure Brianna's role is that of the shared group originator
  let resp = await brianna.call('agent', 'agent_registration', 'get_registered_agents', {})
  t.equal(resp.Ok[0], briannaAddr, 'initial shared group membership configured by host')
  t.equal(resp.Ok.length, 1, 'single initial membership OK to host')
  // and is synced to both agents
  resp = await alice.call('collective_group', 'agent_registration', 'get_registered_agents', {})
  t.equal(resp.Ok[0], briannaAddr, 'shared group membership checks out to joiner')
  t.equal(resp.Ok.length, 1, 'single initial membership OK to joiner')

  // also ensure Alice's project group initial membership
  resp = await alice.call('project_group', 'agent_registration', 'get_registered_agents', {})
  t.equal(resp.Ok.length, 2, 'initial subgroup membership OK')
  t.equal(resp.Ok[0], carolynAddr, 'initial subgroup membership B OK')
  t.equal(resp.Ok[1], aliceProjectAddr, 'initial subgroup membership A OK')

  // Get the project group's ID
  resp = await alice.call('project_group', 'group_agent', 'get_group_id', {})
  t.ok(resp.Ok, 'group Address OK') // :TODO: how to assert this against Tryorama DNA ID?
  const projectGroupId = resp.Ok

  resp = await carolyn.call('project_group', 'group_agent', 'get_group_id', {})
  t.equal(resp.Ok, projectGroupId, 'Project sub-group ID is consistent between peers')

  // Ensure initial capability configuration for triangulated zome
  resp = await alice.call('project_group', 'group_agent', 'has_group_capability', {
    address: aliceProjectAddr,
    capability: 'register_group',
  })
  t.equal(resp.OK, true, 'founding agent of the group has capability to register it with other groups')

  resp = await alice.call('project_group', 'group_agent', 'has_group_capability', {
    address: carolynAddr,
    capability: 'register_group',
  })
  t.equal(resp.OK, true, 'participant of the group has capability to register it with other groups')

  resp = await alice.call('project_group', 'group_agent', 'has_group_capability', {
    address: briannaAddr,
    capability: 'register_group',
  })
  t.equal(resp.OK, false, 'non-participant of the group has no capabilities to perform group actions')

  // Assert sub-group registration initial status
  resp = await brianna.graphQL(`query {
    people {
      id
    }
    organizations {
      id
    }
  }`)
  t.equal(resp.data.people.length, 1, 'host network initial agent registration status OK')
  t.equal(resp.data.organizations.length, 0, 'host network initial sub-group registration status OK')
  t.equal(resp.data.people[0].id, briannaAddr, 'host agent in host network synced with GraphQL API')

  // Have Brianna first vouch for Alice so that she can join the shared group
  resp = await brianna.call('agent', 'social_triangulation', 'vouch_for', {
    agent_address: aliceCollectiveAddr,
  })
  await pause(100)

  // Alice vouches for sub-group to join the 'shared' group
  resp = await alice.call('collective_group', 'group_social_triangulation', 'vouch_for', {
    group_address: projectGroupId,
  })
  await pause(100)

  // Brianna must vouch as well
  // (:TODO: test failure case prior to adding)
  resp = await brianna.call('agent', 'group_social_triangulation', 'vouch_for', {
    group_address: projectGroupId,
  })
  await pause(100)

  // now, Alice can add the sub-group to the shared group
  resp = await alice.call('collective_group', 'group_agent_host', 'register_group', {
    group_address: projectGroupId,
  })
  await pause(100)

  // check updated sub-group status
  resp = await brianna.graphQL(`query {
    people {
      id
    }
    organizations {
      id
    }
  }`)
  t.equal(resp.data.people.length, 2, 'host network agent registration status updated')
  t.equal(resp.data.organizations.length, 1, 'host network sub-group registration status updated')
  t.equal(resp.data.people[0].id, aliceCollectiveAddr, 'alice joined host network')
  t.equal(resp.data.people[1].id, briannaAddr, 'brianna host network participation OK')
  t.equal(resp.data.organizations[0].id, projectGroupId, 'sub-group joined host network')

  // :TODO: should people visible in sub-groups be included in "people" for the 'host' network?
})


