---
layout: blog
title: Introduction to JavaScript Application testing 
segments:
  - blog
published: true
date: 2021-05-07T14:44:57.102Z # update date accordingly
tags:
  - Development
  - Testing
---

An automated testing suite for your application is like a feature requirements document.
When this suite is passing, it means that the features are delivered correctly as described in the requirements.
Automated tests help reduce manual testing time and as long as they are passing, it means that the feature is working as described.
Not only do they reduce costs and time of manual testing but they also can help while development.

Different language ecosystems offer a different set of tools to develop automated tests.
In this article, I am going to describe how various types of tests can be utilized to cover the most common testing scenarios for Web/Mobile applications development with 3rd party integrations.

- [Unit tests](#unit-tests)
  - [Tools that help writing unit tests](#tools-that-help-writing-unit-tests)
  - [Additional resources for unit tests](#additional-resources-for-unit-tests)
- [Snapshots](#snapshots)
  - [Additional resources on snapshots](#additional-resources-on-snapshots)
- [Integration testing](#integration-testing)
  - [Tools that help writing integration tests](#tools-that-help-writing-integration-tests)
  - [Additional resources for integration tests](#additional-resources-for-integration-tests)
- [End to End (e2e) testing](#end-to-end-e2e-testing)
  - [Element selectors](#element-selectors)
  - [Form validation](#form-validation)
  - [Authentication](#authentication)
  - [Dedicated API routes](#dedicated-api-routes)
  - [Screenshot matching](#screenshot-matching)
  - [Tools that help writing e2e tests](#tools-that-help-writing-e2e-tests)
  - [Additional resources for e2e tests](#additional-resources-for-e2e-tests)
- [Component tests](#component-tests)
  - [Tools and resources for component testing](#tools-and-resources-for-component-testing)
- [Code coverage](#code-coverage)
  - [Additional resources for code coverage](#additional-resources-for-code-coverage)

## Unit tests

Unit tests are functions that test a single specific part of the application in small units.
_Function_ can be interpreted as a logical unit.
Unit tests verify if this _function_ has correct behavior with different arguments or its dependencies.
A test consists of a description of the requirement that **should** be satisfied.

```javascript
describe('Location formatter', () => {

  it('should format location based on a passed primaryLocation', () => {
    const formattedLocation = formatLocation({
      primaryLocation: 'Pennsylvania',
      city: null,
      countryIsoCode: 'US',
    })
    expect(formattedLocation).toBe('Pennsylvania, US')
  })
  
  it('should format location based on a passed city', () => {
    const formattedLocation = formatLocation({
      primaryLocation: null,
      city: 'Svidník',
      countryIsoCode: 'SK',
    })
    expect(formattedLocation).toBe('Svidník, SK')
  })

  it('should format location based on a passed primaryLocation with city being ignored', () => {
    const formattedLocation = formatLocation({
      primaryLocation: 'Pennsylvania',
      city: 'Košice',
      countryIsoCode: 'US',
    })
    expect(formattedLocation).toBe('Pennsylvania, US')
  })
})
```

These tests can run after any code change and they don't affect other resources other than _CPU_.
They are also good for practicing _test-driven development_ as they are fast to execute and give immediate feedback to the developer.

If the tested unit is dependent on some external resource like _time_, browser setting, API response, or some service,_mocking_ can be used to substitute the resource to the unit.
_Mocking_ is especially useful with a technique called [_dependency injection_](https://en.wikipedia.org/wiki/Dependency_injection)

The effect of the unit can be then tested on these _mocks_:

```javascript
const service = {
  fetch: jest.fn(() => responseFixture)
}

// Using dependency injection
const result = fetchAndParseResponse(service, 'path')
// -- snip expecting correct result
// It's expected that `fetchAndParseResponse` will call the `fetch` only once
expect(service.fetch).toHaveBeenCalled(1)
```

Unit tests can also make use of [snapshots](#snapshots).

### Tools that help writing unit tests

- [Jest testing framework](https://jestjs.io/)
- [mocha test runner](https://mochajs.org/) - Usually paired with assertion library of choice
- [Chai assertion library](https://www.chaijs.com/)
- [jest-date-mock](https://github.com/hustcc/jest-date-mock) - Testing functionality that is dependent on the Date

### Additional resources for unit tests

- [Jest mocking guide](https://jestjs.io/docs/manual-mocks)
- [How to Start Unit Testing Your JavaScript Code](https://www.freecodecamp.org/news/how-to-start-unit-testing-javascript/) by [Ondrej Polesny @ FreeCodeCamp](https://www.freecodecamp.org/news/author/ondrej/)
- [The importance of test driven development](https://medium.com/@gondy/the-importance-of-test-driven-development-f80b0d02edd8) by [Godswill Okwara @ medium.com](https://medium.com/@gondy)

## Snapshots

Snapshot matchers will save the input object as an artifact and they will use that artifact to compare the snapshot with the snapshot of the next test run.

```javascript
it('should create a JSON response', () => {
  const feed = createFeedResponse();
  expect(JSON.parse(feed)).toMatchSnapshot('parsed feed');
});
```

> On subsequent test runs, Jest will compare the output with the previous snapshot. If they match, the test will pass.
> If they don't match, either the test runner found a bug in your code that should be fixed, or the implementation has changed and the snapshot needs to be updated.

These snapshots help to detect changes in the output of some functionality.
They can also be used with a component renderer to detect changes in the output of component's _HTML_, _DOM_ or virtual _DOM_.
Snapshots are created and updated by the test runner automatically so they lower the churn of updating tests over time.

Snapshot artifacts should be committed into the code repository alongside the tests, so they can be used as an input for the next test runs.
Snapshots can be utilized for many types of tests.

If the expected result includes some varying data,
[property matchers](https://jestjs.io/docs/snapshot-testing#property-matchers) can be utilized to verify the type of the data instead.

### Additional resources on snapshots

- [Jest snapshot testing](https://jestjs.io/docs/snapshot-testing)

## Integration testing

Integration tests are meant to validate the behavior of the individual components and their interaction with different system modules.

Take an API resolver as an example:

1. Resolver is given different parameters
2. It fetches some data from a database
3. Makes an action given the data
4. Returns a response to the client

An integration test should test and validate the functionality of this resolver.
It should simulate a client request and assert desired response output.
It should also verify the actions that the resolver is making.

```javascript
// Mock the e-mail service, we don't want to send real emails
jest.mock('../email.js')

it('should send activation email', async () => {
  const client = await Client.query().findOne({
    firstName: 'testing',
    lastName: 'user',
  })
  const sendActivationEmail = await apolloClient.mutate({
    mutation: SEND_ACTIVATION_EMAIL_MUTATION,
    variables: { clientId: client.id },
  })

  expect(sendActivationEmail.errors).not.toBeDefined()
  // One email should be sent
  expect(email.send).toHaveBeenCalledTimes(1)

  // Verify that the email was sent with correct parameters
  expect(email.send).toHaveBeenCalledWith({
    templateId: EMAIL_TEMPLATE_IDS.CLIENT_EMAIL_ACTIVATION,
    to: client.contactEmail,
    data: {
      client_first_name: client.firstName,
    },
  })
})
```

It might be useful to **mock** some **dependencies** such as a database or a 3rd party API to ensure that these tests can run multiple times in sandboxed environment.

Sometimes it is desired to have a database, that is set up just for integration tests to avoid mocking.
It can be also used for validating integration in between the tests.
In this scenario, it is very useful to _seed_ the database. The database can be then truncated and seeded in between the tests ensuring predictable and stable test results.

Integration tests can be run with the same test runner as the [unit tests](#unit-tests), but with a different configuration.
Configuration might include a [setup](https://jestjs.io/docs/configuration#globalsetup-string) and a [teardown](https://jestjs.io/docs/configuration#globalteardown-string) process.
When running tests with a database in place the tests could fail because the same data dependency is being modified in multiple tests at once, therefore it is required for these tests to run in [serial mode](https://jestjs.io/docs/cli#--runinband).

Keep in mind that you should **avoid testing** 3rd party libraries or **code that is not part** of the application logic.
This code is usually tested by the authors of those libraries.

### Tools that help writing integration tests

- [casual](https://www.npmjs.com/package/casual) or [faker](https://www.npmjs.com/package/faker) - Fake data generators
- [Utilities for testing Apollo Server](https://www.apollographql.com/docs/apollo-server/testing/testing/)
- [supertest](https://github.com/visionmedia/supertest) - HTTP request assertion library
- [jest-fetch-mock](https://github.com/jefflau/jest-fetch-mock#readme) - Request mocking
- [SinonJS](https://sinonjs.org/) - Standalone test spies, stubs and mocks

### Additional resources for integration tests

- [What is API testing](https://www.edureka.co/blog/what-is-api-testing) by [Archana Choudary @ edureka.com](https://www.edureka.co/blog/author/archana-cedureka-co/)
- [GraphQL integration tests with apollo-server-testing, jest-mongodb and nock](https://medium.com/@jdeflaux/graphql-integration-tests-with-apollo-server-testing-jest-mongodb-and-nock-af5a82e95954) by [Julien Deflaux @ medium.com](https://medium.com/@jdeflaux)

## End to End (e2e) testing

e2e tests should guarantee that our **users can browse and use the application**.
Every part of the application, whether it is a front-end, back-end, or a 3rd party integration, will be tested.
We look to verify the behavior of the application in response to events triggered by the user or any different side-effect.
The goal is to test the application from the user's perspective.

### Element selectors

There are different frameworks and approaches for creating _DOM_ elements.
These elements can have different representations of their state in the _HTML_.
Developers should agree on the practice of selecting these elements in tests that suits them best.
You should try to target elements in a way that you can be sure that the **results will be consistent** even after different layout changes that might happen throughout the lifetime of the project.
Therefore I'd say that [XPath](https://developer.mozilla.org/en-US/docs/Web/XPath) is not the recommended way.
To have selectors that are reliable and not changing with different layout changes we can use `id` attributes or even more sophisticated `data-*` attributes.
With `data-*` attributes we can target multiple nodes with the same identifier and establish a convention that will make sure that the elements with these attributes are being used in the tests so if they are changed/deleted, there will have to be a change in the tests as well.

```html
<button data-test-id="close-modal-button">Close</button>
```

You don't have to mark all targeted elements with `id` you can use _CSS_ selectors as well.
Ensure, that you will not have to change stable test cases because of new additional features, which are not relevant to the established tested features.

### Form validation

Forms should be tested for correctly evaluating validation.
The errors should appear when the validation fails.
It's also good user experience testing when tests require validation errors to be displayed only if the elements were previously touched or the form was submitted.

```javascript
it('should display validation errors', () => {
  // Form is submitted with `enter`
  cy.get('[data-test-id="first-name-input"]').type('{enter}')

  // All validation errors should be visible
  cy.get('[data-test-id="first-name-hint"]')
    .should('exist')
    .and('contain', 'First Name is a required field')
  cy.get('[data-test-id="last-name-hint"]')
    .should('exist')
    .and('contain', 'Last Name is a required field')
  cy.get('[data-test-id="email-hint"]')
    .should('exist')
    .and('contain', 'Email is a required field')
  cy.get('[data-test-id="password-hint"]')
    .should('exist')
    .and('contain', 'Password is a required field')
  cy.get('[data-test-id="terms-accepted-hint"]')
    .should('exist')
    .and('contain', 'You have to accept the terms of use')

  // Email field validation
  cy.get('[data-test-id="email-input"]').type(
    `invalid{enter}`
  )
  cy.get('[data-test-id="email-hint"]')
    .should('exist')
    .and('contain', 'Invalid email')
})
```

Form submission should be tested as well, to see if the back-end works correctly.
It should be expected to see correct error messages being shown to the user when the submission fails.
For example logging in with wrong password:

```javascript
it('should toast wrong password', () => {
  cy.get('[data-test-id="email-input"]').type('user@login.test')
  cy.get('[data-test-id="password-input"]')type(
    `wrongpassword{enter}`
  // -- snip -- wait for submission
  cy.get('[data-test-id="error-message"]')
    .should('exist')
    .and('contain', 'The password is invalid or the user does not have a password.')
})
```

While submission is in progress, the loading state of the form can be expected for having disabled input fields.
There should be a way to wait for asynchronous operations to be completed, so we can reliably test the loading behavior.
_Cypress_ has an [intercept method](https://docs.cypress.io/api/commands/intercept.html), that allows us to **hold the response** and do as many expectations **until it is desired** for the response to be let go with the [wait method](https://docs.cypress.io/api/commands/wait.html).

```javascript
cy.intercept('**/submit').as('submission')
// -- snip -- fill the form
cy.get('[data-test-id="submit-button"]').click()
// Expecting loading state
cy.get('[data-test-id="submit-button"]').should('have.class', 'loading').and('be.disabled')
// If submission request is already done if will be witheld from resolving until `cy.wait` is called
// Otherwise it will wait until the request is completed
cy.wait('@submission')
```

### Authentication

**Don't try to log in to the application for every test** that needs authenticated user.
Test the login flow as a proper test case, but just for that sake only.
For tests that require a user to be logged in, create a separate log-in functionality that will authenticate the user in the background without any browser interaction. This will speed up the testing process tremendously.
Read [cypress documentation on login](https://docs.cypress.io/guides/getting-started/testing-your-app#Logging-in) for more examples.

### Dedicated API routes

Sometimes it's beneficial to implement a dedicated testing API route/resolver which will help seed individual tests or perform a clean up after some tests.
Also, can be used for simulating a result of integration with 3rd party API without accessing 3rd party resources.
Make sure that, these routes can only be run in a non-production environment.

### Screenshot matching

Many plugins can be integrated into the testing environment, that enable **testing visual regressions**.

Screenshot snapshot matching allows to:

- Ensure, that the layout is not broken for different window sizes
- Check visual changes of every component as part of a review
- Prevent accidental visual regression in components

Screenshots can be taken of the whole viewport or individual components.
Be sure to not include any variable data (Dates, IDs, etc.) in the screenshots as they will easily cause a mismatch.

### Tools that help writing e2e tests

- [Cypress testing framework](https://www.cypress.io/) - Next generation front-end testing framework
- [Playwright](https://playwright.dev/) - Playwright is a library to automate Chromium, Firefox and WebKit with a single API
- [Puppeteer](https://pptr.dev/) - High level API to control Chromium
- [Jest-image-snapshot](https://github.com/americanexpress/jest-image-snapshot) Matching image comparisons
- [Percy](https://percy.io/) - Visual review platform for visual regression tests

### Additional resources for e2e tests

- [Cypress testing strategies](https://docs.cypress.io/guides/getting-started/testing-your-app#Testing-strategies)
- [BrowserStack e2e testing guide](https://www.browserstack.com/guide/end-to-end-testing) by Shreya Bose

## Component tests

Component tests take individual components and test them as a unit.
These can run in the browser or just _DOM_ simulator.

These tests, just like unit tests, should ensure that the expected output of the component is rendered based on the component input.
They should also test the behavior of the component when any internal state change happens.
For example, when you open an accordion, you should expect the children to be shown in the output.
You can also expect, that the passed callbacks are being called according to event triggers.

Most often, these tests rely on [snapshots](#snapshots).
They reveal changes in the output of the components.
Also, they save a tremendous amount of time and pain while maintaining these tests.

### Tools and resources for component testing

- [Cypress component testing](https://docs.cypress.io/guides/component-testing/introduction/)
- [React testing overview](https://reactjs.org/docs/testing.html)

## Code coverage

While being an important metric it can also be misleading.
It is a measure, that indicates how much of the source code is being executed while testing.
Coverage can **identify branches of code**, that haven't been validated.
Having 100% coverage **does not mean**, that you have **successfully tested** your application.
High coverage does not guarantee to have bug-free code.
Therefore, it's important to test all **possible scenarios** and **validate the code according to requirements**.

### Additional resources for code coverage

- [Jest code coverage report explained](https://www.emgoto.com/jest-code-coverage/)
- [Code Coverage Tutorial](https://www.softwaretestinghelp.com/code-coverage-tutorial/#What_Is_Code_Coverage)
