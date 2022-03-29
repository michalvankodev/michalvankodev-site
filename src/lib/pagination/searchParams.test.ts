import { describe, test, expect } from 'vitest'
import {
  getPaginationQueryFromSearchParams,
  getPaginationSearchParams,
  parseParams,
} from './searchParams'

describe('convert search params', () => {
  test('drop take params are not taken as filters', () => {
    expect(
      getPaginationQueryFromSearchParams(
        new URLSearchParams('offset=2&limit=5')
      )
    ).toEqual({ offset: 2, limit: 5 })
  })

  test('return empty paginationQuery if ', () => {
    expect(getPaginationQueryFromSearchParams(new URLSearchParams(''))).toEqual(
      {}
    )
  })

  test('other than drop take params are moved to filters ', () => {
    expect(
      getPaginationQueryFromSearchParams(new URLSearchParams('tag=news'))
    ).toEqual({ filters: { tag: 'news' } })
  })

  test('offset and filter combined', () => {
    expect(
      getPaginationQueryFromSearchParams(
        new URLSearchParams('offset=3&tag=news')
      )
    ).toEqual({ offset: 3, filters: { tag: 'news' } })
  })
})

describe('get search params', () => {
  test('parse params', () => {
    const params = 'tags/News/page/1'
    expect(parseParams(params)).toEqual({ tags: 'News', page: '1' })
  })

  test('should parse values into searchParams for first page', () => {
    const params = 'tags/News/page/1'
    expect(getPaginationSearchParams(7, params).toString()).toEqual(
      'limit=7&offset=0&tags=News'
    )
  })

  test('should parse values into searchParams for third page', () => {
    const params = 'tags/News/page/3'
    expect(getPaginationSearchParams(7, params).toString()).toEqual(
      'limit=7&offset=14&tags=News'
    )
  })

  test('should return first page without any params specified', () => {
    const params = ''
    expect(getPaginationSearchParams(7, params).toString()).toEqual(
      'limit=7&offset=0'
    )
  })
})
