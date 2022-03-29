import { splitEvery } from 'ramda'
import type { PaginationQuery } from './pagination'

export function getPaginationQueryFromSearchParams(
  searchParams: URLSearchParams
) {
  return Array.from(searchParams).reduce<PaginationQuery>(
    (acc, [key, value]) => {
      const isDropTake = ['offset', 'limit'].includes(key)
      if (isDropTake) {
        return {
          ...acc,
          [key]: Number(value),
        }
      }
      return {
        ...acc,
        filters: {
          ...acc.filters,
          [key]: value,
        },
      }
    },
    {}
  )
}

export function parseParams(params: string) {
  const splittedParams = params.split('/')
  if (splittedParams.length % 2 !== 0) {
    return []
  }
  const splits = splitEvery(2, splittedParams)
  return Object.fromEntries(splits)
}

/**
 * Convert svelte `load` params into a `URLSearchParams` so they can be used to fetch endpoints with pagination queries
 */
export function getPaginationSearchParams(pageSize: number, params: string) {
  const { page = 1, ...filters } = parseParams(params)

  const offset = pageSize * (page - 1)
  const limit = pageSize
  return new URLSearchParams({ limit, offset, ...filters })
}
