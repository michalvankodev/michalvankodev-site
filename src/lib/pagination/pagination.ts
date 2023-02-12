import { identity, drop, take, pipe } from 'ramda'

export interface PaginationQuery {
  offset?: number
  limit?: number
  filters?: Record<string, string>
}

export interface PaginationResult<ItemType> {
  items: ItemType[]
  totalCount: number
}

export function dropAndTake<Item>({ offset = 0, limit = Infinity }) {
  return pipe(drop<Item>(offset), take<Item>(limit)) as (
    items: Item[]
  ) => Item[]
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function filterByPropContains<Item extends Record<string, any>>(filters: Record<string, string>) {
  return function (items: Item[]) {
    return items.filter((item) => {
      return Object.entries(filters).every(([fieldName, value]) =>
        item[fieldName].includes(value)
      )
    })
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function filterAndCount<Item extends Record<string, any>>({
  filters,
  ...dropTakeParams
}: PaginationQuery) {
  return function (items: Item[]) {
    const filterFunction = filters
      ? filterByPropContains<Item>(filters)
      : identity
    const filteredItems = filterFunction(items)
    return {
      items: dropAndTake<Item>(dropTakeParams)(filteredItems),
      totalCount: filteredItems.length,
    }
  }
}
