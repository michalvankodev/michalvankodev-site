import { range } from 'ramda'
import { describe, expect, test } from 'vitest'
import { filterByPropContains, dropAndTake, filterAndCount } from './pagination'

describe('pagination', () => {
  test('does not drop any items by default', () => {
    const items = range(0, 100)
    expect(dropAndTake({})(items)).toHaveLength(100)
  })

  test('limits out exact number of items', () => {
    const items = range(0, 100)
    expect(dropAndTake({ limit: 10 })(items)).toHaveLength(10)
    expect(dropAndTake({ limit: 10 })(items)[0]).toBe(0)
    expect(dropAndTake({ limit: 10 })(items)[9]).toBe(9)
  })

  test('offset is skipping a number of items from the front', () => {
    const items = range(0, 100)
    expect(dropAndTake({ offset: 10 })(items)).toHaveLength(90)
    expect(dropAndTake({ offset: 10 })(items)[0]).toBe(10)
  })

  test('is able to combine limit and offset', () => {
    const items = range(0, 100)
    expect(dropAndTake({ offset: 10, limit: 10 })(items)).toHaveLength(10)
    expect(dropAndTake({ offset: 10, limit: 10 })(items)[0]).toBe(10)
    expect(dropAndTake({ offset: 10, limit: 10 })(items)[9]).toBe(19)
  })

  test('is able to filter by a field', () => {
    const items = [
      {
        id: 1,
        prop: ['yes'],
      },
      {
        id: 2,
        prop: ['yes', 'no'],
      },
    ]

    expect(filterByPropContains({ prop: 'no' })(items)).toHaveLength(1)
    expect(filterByPropContains({ prop: 'no' })(items)[0].id).toBe(2)

    expect(filterByPropContains({ prop: 'yes' })(items)[0].id).toBe(1)
    expect(filterByPropContains({ prop: 'yes' })(items)).toHaveLength(2)
  })

  describe('is able to combine limit and offset while filtering by field', () => {
    const items = [
      {
        id: 1,
        prop: ['yes'],
      },
      {
        id: 2,
        prop: ['yes', 'no'],
      },
      {
        id: 3,
        prop: ['yes', 'no'],
      },
    ]

    test('combine all parameters', () => {
      const result = filterAndCount({
        offset: 1,
        limit: 1,
        filters: { prop: 'no' },
      })(items)
      expect(result.totalCount).toBe(2)
      expect(result.items[0].id).toBe(3)
    })

    test('with 0 offset', () => {
      const result = filterAndCount({
        offset: 0,
        limit: 1,
        filters: { prop: 'no' },
      })(items)
      expect(result.totalCount).toBe(2)
      expect(result.items[0].id).toBe(2)
    })

    test('without filter', () => {
      const result = filterAndCount({ offset: 1, limit: 1 })(items)
      expect(result.totalCount).toBe(3)
      expect(result.items[0].id).toBe(2)
    })

    test('without any params', () => {
      const result = filterAndCount({})(items)
      expect(result.totalCount).toBe(3)
      expect(result.items.length).toEqual(result.totalCount)
    })
  })
})
