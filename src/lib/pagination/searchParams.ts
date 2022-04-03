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

export function toParams(records: Record<string, string>) {
  return Object.entries(records)
    .map(([key, value]) => `${key}/${value}`)
    .join('/')
}

export interface PaginationSearchParams {
  pageSize: number
  page: number
  filters?: Record<string, string>
}

/**
 * Convert svelte `load` params into a `URLSearchParams` so they can be used to fetch endpoints with pagination queries
 */
export function getPaginationSearchParams({
  pageSize,
  page,
  filters,
}: PaginationSearchParams) {
  const offset = pageSize * (page - 1)
  const limit = pageSize
  return new URLSearchParams({ limit, offset, ...filters })
}
