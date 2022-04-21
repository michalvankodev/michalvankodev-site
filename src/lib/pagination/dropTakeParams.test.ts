import { describe, test, expect } from 'vitest'
import { getDropTakeFromPageParams } from './dropTakeParams'

describe('convert search params', () => {
  test('should convert from page size and page to offset and limit', () => {
    expect(getDropTakeFromPageParams(7, 2)).toEqual({
      offset: 7,
      limit: 7,
    })
  })
})
