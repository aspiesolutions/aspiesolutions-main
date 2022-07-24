import CypressTestRelay from "../../components/cypressTestRelay"
describe('TestRelay.cy.ts', () => {
  it('mounts', () => {
    cy.mount(<CypressTestRelay />)
  })
})