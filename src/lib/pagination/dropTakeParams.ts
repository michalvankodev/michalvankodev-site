import { init, splitEvery } from 'ramda'

export function parseParams(params: string) {
  let splittedParams = params.split('/')
  if (splittedParams.length % 2 !== 0) {
    splittedParams = init(splittedParams)
  }
  const splits = splitEvery(2, splittedParams)
  return Object.fromEntries(splits)
}

export function toParams(records: Record<string, string>) {
  return Object.entries(records)
    .map(([key, value]) => `${key}/${value}`)
    .join('/')
}

export interface PaginationParams {
  pageSize: number
  page: number
  filters?: Record<string, string>
}

export interface DropTakeParams {
  offset: number
  limit: number
}

/**
 * Convert svelte `load` params into a `offset` and `limit` so they can be used to fetch endpoints with pagination queries
 */
export function getDropTakeFromPageParams(
  pageSize: number,
  page: number
): DropTakeParams {
  const offset = pageSize * (page - 1)
  const limit = pageSize
  return {
    offset,
    limit,
  }
}
