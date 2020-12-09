---
layout: blog
title: Guide on error handling
published: true
date: 2020-12-09T14:44:11.948Z
tags:
  - Development
---
Having a good _error handling_ practice is very important when building complex applications.
Applications which consist of multiple applications such as web applications,
where you have multiple parts like _client_, _server_ and _database_ create a system where
errors can happen in any individual parts or in communication between them.

Let me show you some examples of what kind of errors there might happen and how to treat them.

## Common scenarios

Don't pay attention to the coding style here.
There might be a different kind of libraries and conventions used in applications.
Code samples are written just for demonstration of the various scenarios and what should happen in those scenarios.
Please read through the comments as they explain how errors should be treated.

### Back-end application - Single run script or a CRON job

```javascript
  try {
    // This can be any type of action that might fail
    Database.connect().insert({ value: 'Some information' })
  } catch (err) {
    // Is important to **log these** errors with highest priority (error)
    logger.error('Error happened while trying to insert some information', err) // Error context is passed here

    // Should the script continue?
    return
  }
```

It is necessary to pass the error context to the logger so the error can be traced later.

### Back-end application - Responding to client requests

This is usually how web applications are built and most common scenario where errors should be properly handled as it might be a security threat.

```javascript
  app.post('/message', (req, res) => {
    const { message, author } = req
    try {
      Database.connect().insert({ message, authorId: author.id })
    } catch (err) {
      if (err instanceof ValidationError) {
        // Validation errors are expected to happen often.
        // They are caused by a bad input from the user.
        // There is no need to log these errors as they don't cause a system crash.

        // Client should be informed on the matter what went wrong with the request.
        res.status(400).json({
          message: `Invalid input. ${err.message}`
        })
      else if (err instanceof RecoverableError) {
        // As a `RecovarableError` we can think of an error that most usually doesn't happen
        // but can and we have a way to continue the rest of the process and handle the error recovery later.
        // As an example we can imagine a case of sending an email to users with some information.
        logger.error('Error while sending mail', err)

        // Sending an email is not a crucial part of the functionality and can be done later.
        // So we continue the functionality but we log the error so we know that we have to handle it. 
        res.status(200).json(data)
      } else {
        // This is an example of an **unexpected** error.
        // It doesn't only apply to an `Database`. You can replace the `Database` with any other 3rd party system.
        // These type of errors can be programmers fault of not handling certain scenarios correctly,
        // but also be caused by an unexpected events happening in the system
        // (ex. Database server is down, usage of wrong ID's of relations in SQL queries)

        // In this case we want to `log` this error with full context,
        // so it can be found and fixed if it is an programmers fault.
        logger.error('Unexpected error happened', err)

        // Client should be informed of such an error but without the context as it might reveal proprietary information about the system
        res.status(500).json({
          message: 'Unexpected error happened'
        })
      }
    }
  })
```

It is very important to **keep all proprietary information** about the system **secure and private** only to developers and system administrators.
It is a **security threat** and it might be abused by malicious hackers when any of the information gets leaked.
It doesn't matter if the information is shown to the users or not, while it's being sent to the client it can be discovered. I

However if there is an additional error handling functionality in place which
is able to filter out the error context in the `production` environment,
it might be a good practice to include the error context in the `development` environment,
as it will speed up the error discoverability.

To implement such feature I'd recommend to split `message` types sent to the client,
so there isn't a chance that someone would accidentally send proprietary information to the client.

```javascript
  function sendError(body) {
    if (process.env === 'production') {
      return omit(body, 'errorContext')
    }
    return body
  }

  res.status(500).json(sendError({
    message: 'Unexpected error happened',
    errorContext: {req, message: `Error while submitting ${req.path}`, err}
  })
```

### Front-end application - Handle an error from back-end

When an error happens on the back-end we should show this error on the client so the user knows that his action was not successfully fulfilled.

If back-end handles errors correctly and sends client an appropriate message, client should be able to just show the informative description of the error that occurred.

#### Translating errors

If your application is built with any **internationalization** framework or with a translation system in general,
there has to be some kind of mapping from error sent from back-end to the translated message.

Simple mapping could be done with having a table of error codes for any application error that might happen.
These codes don't have to be strictly numeric. They can consist of some rules like having separators to distinguish
services or parts which fail and which way. Examples `RESOURCE-404`, `POST-404`.
It's a good practice to have always a default message for unexpected errors which often do happen.

### Front-end application - Network Error

These type of errors are least expected by developers but they happen regularly.
Users usually lose connection when they browse while traveling or are connected to unreliable mobile connection.
It should always be anticipated that these errors can happen and the application should recover
from the failure and allowed to retry their action.

### Front-end application - unexpected error in application code

Errors which happen only on front-end are often a coding error.
Most common examples would be not covering all possible cases of application logic or accessing properties
that are undefined because of unexpected shape of response data.
While back-end applications can recover from their errors by restarting the process,
it doesn't apply to client applications where these type of **errors might cause a crash**, or a **freeze** of an application.
Some frameworks allow to **recover from these crashes** by implementing an **error boundaries**.
See [_React_ Error Boundaries for example](https://reactjs.org/docs/error-boundaries.html).
I can only highly recommend of using such type of recovery.

If you use an external error monitoring tool like [Sentry](https://sentry.io/welcome/) or [bugsnag](https://www.bugsnag.com/), don't forget to log these errors from the error boundary.

## Rules

To summarize this we can create an a list of rules:

- Error messages have to be informative for client, but they **can't reveal private and proprietery information** about the system architecture
- Errors should be logged only when they require additional action
- Errors that are recoverable should not crash the system
- Unexpected errors should turn into expected errors and be handled properly
- Failures between connections of the systems should be anticipated
- Client applications should be always able to **recover from unexpected errors** just as from expected ones
