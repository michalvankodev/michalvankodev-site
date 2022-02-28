---
layout: blog
title: Error handling with Either<Type>
published: true
date: 2022-02-27T16:03:10.093Z
tags:
  - Development
  - Guide
---
We have started a new small internal project for automating a few workflows around counting worked hours and time offs.

## Application architecture

The **application is a slack bot** on top of node js, TypeScript and PostgreSQL database. We use 3rd party APIs to fetch data about the times which we need to accumulate and process to calculate valuable information to our users.

It will run on a server with a possibility of migrating into serverless when we decide if it's a good use case for it. We've decided that we won't experiment too much for development as we want to make it useful first.

## API design

As it is not a classic web server application I had to come up with slightly different error handling as we are used to. I've been trying to find a semi-functional API with all the good practices described in my [guide on error handling](https://michalvanko.dev/blog/2020-12-09-guide-on-error-handling). The main goal is to not let users be presented with internal information about errors. We want to show user-friendly messages instead.
I call this API semi-functional as **I didn't want to use monads** and go 100% functional. We use simple asynchronous functions to handle interactions.
The goal is to handle errors that are expected. Unexpected errors should still be thrown and caught by an _"Error boundary"_ around the whole app that will handle and log the error.

## Error types

Let's create 2 types of `Error`s that all the other `Error`s can be extended from.
These will allow us to distinguish if the thrown `Error` should be presented to the user. We want to handle `InternalError`s differently. We might log them to different logs or trigger alarms before we convert them to a different `PublishableError`. 

```typescript
export class PublishableError extends Error {
  publishable = true
  constructor(message: string) {
    super(message)
    Object.setPrototypeOf(this, PublishableError.prototype)
  }
}

export class InternalError extends Error {
  publishable = false
  constructor(message: string) {
    super(message)
    Object.setPrototypeOf(this, InternalError.prototype)
  }
}
```
Then, we can create multiple app-specific errors by extending from these two.

> We need to set `Object.setPrototypeOf(...)` as TypeScript [introduced a breaking change](https://github.com/Microsoft/TypeScript-wiki/blob/main/Breaking-Changes.md#extending-built-ins-like-error-array-and-map-may-no-longer-work) that may cause the inheritance to now work properly. 


## Handling errors

I wanted to have a similar style of handling the error as the [Ramda's `tryCatch`](https://ramdajs.com/docs/#tryCatch) function. I couldn't just use Ramda's `tryCatch` as it doesn't support asynchronous functions. I've found inspiration in the [`fp-ts` `TaskEither` type](https://gcanti.github.io/fp-ts/modules/TaskEither.ts.html).

I've come up with the following solution:

```typescript
/**
 * Tuple of error and a result of an asynchronous task that might throw an error
 */
export type Either<ResultType, ErrorType extends Error> = Promise<
  [error: ErrorType | null, result?: ResultType]
>

/**
 * Try to execute an async function and return a tuple of `[Error, ResultType]`
 * If the operation is successful error will be `null`
 * If the operation fails, the `errorHandler` runs with the exception that was thrown, Result is `undefined`
 * It catches all errors that might happen in the passed async task.
 * It will not catch any errors that are thrown from the `errorHander`
 */
export function wrapWithErrorHandler<
  ErrorType extends Error,
  // TS is still able to inherit function parameters
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  FunctionArgs extends Array<any>,
  ResultType
>(
  errorHandler: (error: ErrorType | unknown) => ErrorType,
  fn: (...args: FunctionArgs) => Promise<ResultType>
): (...args: FunctionArgs) => Either<ResultType, ErrorType> {
  return async (...args: FunctionArgs) => {
    try {
      const result = await fn(...args)
      return [null, result]
    } catch (exception) {
      const errorHandlerResult = errorHandler(exception)
      return [errorHandlerResult, undefined]
    }
  }
}
```

I struggled with the typings for a few minutes but I was impressed with the result.
The goal of this function is to wrap an asynchronous function that might throw errors and let the `errorHandler` process the error and return an alternative result.

As most of the errors will be possibly handled by the same `errorHandle` we can create a `genericErrorHandler` that might be used in most situations.

```typescript
/**
 * Generic Error handler that should be used in most common use cases
 * It will throw an error if any unexpected error is thrown
 * When used with `wrapWithErrorHandler` it is able to catch most of the expected errors
 */
export function genericErrorHandler<ErrorType extends Error>(exception: ErrorType | unknown) {
  if (exception instanceof PublishableError) {
    // These errors are usually handled no need to log them
    return exception
  }
  if (exception instanceof InternalError) {
    // Log error and return publishable generic error
    return new PublishableError(
      "I'm sorry for the inconvenience, but my circuits have been overloaded :zap:"
    )
  }

  if (exception instanceof Error) {
    // This is unexpected error, better log it and let rethrow it further away
    throw exception
  }

  return exception as ErrorType
}
```

To spare some characters of redundant code I've applied a partial application principle to use these functions together. At first, I've tried to use [Ramda's `partial`](https://ramdajs.com/docs/#partial), but it was not able to inherit correct types, so I've just written the partially applied function myself:

```typescript
/**
 * Helper function with applied `genericErrorHandler` to `wrapWithErrorHandler`
 */
export function wrapWithGenericErrorHandlerFunction<FunctionArgs extends Array<any>, ResultType>(
  fn: (...args: FunctionArgs) => Promise<ResultType>
) {
  return wrapWithErrorHandler(genericErrorHandler, fn)
}
```

## Usage

To sum it up, I'd like to present you the way how this API is used:

```typescript
const [error, memberId] = await wrapWithGenericErrorHandlerFunction(getUserId)(user.token)
    if (error) {
      await respond(error.message)
      return
    }
```

This API allows me to keep the separation of concerns between the tasks and error handling into two separate functions. It also allows us to use the `wrapWithErrorHandler` with different `errorHandler` when we need to take special care. Also, I am still able to throw errors from the `errorHandler` that might be caught by a different `errorHandler` from an upper scope.

Thank you for reading!
