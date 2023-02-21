---
layout: blog
title: Logging recommendations
segments:
  - blog
published: true
date: 2020-12-18T14:39:53.533Z
tags:
  - Development
  - Guide
---
## Back-end

### 1. Use a logger

**Do not** use `console` directly. Programs should be able to define logging strategies dependant on the environment.
We strongly recommend using a logging library such as [Winston](https://github.com/winstonjs/winston) (commonly used in _node.js_ programs).

Libraries should provide an easy way to choose a _transport_ which will be used as on output for logs.
Examples of transports:

- File
- Console
- 3rd party monitoring tools (e.g: [loggly](https://www.loggly.com/))

### 2. Log levels

You can set up as many log levels as needed.
_npm_ uses these 7:

```json
{
  error: 0,
  warn: 1,
  info: 2,
  http: 3,
  verbose: 4,
  debug: 5,
  silly: 6
}
```

These are sorted by priority and should be used appropriately.
What is logged depends on the environment(dev, test, prod...) in which the program operates.
_In the production environment, `debug` logs won't be displayed since they would cause a lot of unnecessary noise._
_Verbose logs might be useful for staging environments where you want to see more information about what is happening in the system._

### 3. What to log

It is very important to log all the information required for developers and DevOps to retrieve as much valuable information from the logs as possible.

Messages are usually generated in a way where developers don't have to add things like timestamps, log levels, etc., and repeat themselves.
Timestamps and log levels are attached to the messages automatically.

#### Errors

Not every error should be logged.
Validation errors and expected errors such as handled errors
are common and they're expected to happen frequently, so they shouldn't interrupt the system in any way whatsoever.
Therefore they should not be seen in logs. They can be shown in the access logs.

Unexpected errors should be logged with the highest priority (error). These errors should be monitored, prevented, and handled.

#### System state changes

Back-end apps strive to be stateless but they also do have some state like:

- served endpoint, port
- database connection
- 3rd party integrations

Changes in the system with valuable information should be logged with the `info` level. They serve as a great source of information about the system prior to the failure.

#### Debug

Don't be hesitant to use `debug` logs. They can always be turned off or turned on back again while debugging.
Most of the time developers tend to use `console.log` and then remove it before committing.
I'd encourage you to use `debug` level and leave it there.
It can be a big help in the future which can point you to the part of the code which is responsible for the manipulation of the data.

### 4. Don't log sensitive information

**Never, ever log credentials, passwords, or any sensitive information.**

## Front-end

Front-end logging is fundamentally different from the back-end.
There is a new instance of the program for every opened window and it is only opened for 1 particular client.
Developers should **not** expose the functionality to users.

There is an _npm package_ [debug](https://www.npmjs.com/package/debug) that
enables developers to write debug logs that persist in the codebase.
To reveal the logs in the console, they have to be enabled.

There are 3rd party **error monitoring** services, that can help us collect information from errors that occur in different parts of the application.

- [bugsnag](https://www.bugsnag.com/)
- [Sentry](https://sentry.io/welcome/)
