import { identity } from 'ramda'
import { flow, A } from '@mobily/ts-belt'
const { drop, take } = A

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
  return flow(drop<Item>(offset), take<Item>(limit))
}

export function filterByPropContains<Item>(filters: Record<string, string>) {
  return function (items: Item[]) {
    return items.filter((item) => {
      return Object.entries(filters).every(([fieldName, value]) =>
        item[fieldName].includes(value)
      )
    })
  }
}

export function filterAndCount<Item>({
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
