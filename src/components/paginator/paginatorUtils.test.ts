import { describe, expect, test } from 'vitest'
import { Divider, getPaginatorPages } from './paginatorUtils'

describe('Paginator component', () => {
  describe('Paginator generates feasable pages to display', () => {
    test('Page: 1/5', () => {
      expect(
        getPaginatorPages({ page: 1, totalCount: 5, pageSize: 1 })
      ).toEqual([1, 2, 3, 4, 5])
    })
    test('Page: 4/7', () => {
      expect(
        getPaginatorPages({ page: 4, totalCount: 7, pageSize: 1 })
      ).toEqual([1, 2, 3, 4, 5, 6, 7])
    })
    test('Page: 4/8', () => {
      expect(
        getPaginatorPages({ page: 4, totalCount: 8, pageSize: 1 })
      ).toEqual([1, 2, 3, 4, 5, 6, Divider, 8])
    })
    test('Page: 1/10', () => {
      expect(
        getPaginatorPages({ page: 1, totalCount: 10, pageSize: 1 })
      ).toEqual([1, 2, 3, 4, 5, 6, Divider, 10])
    })
    test('Page: 2/10', () => {
      expect(
        getPaginatorPages({ page: 2, totalCount: 10, pageSize: 1 })
      ).toEqual([1, 2, 3, 4, 5, 6, Divider, 10])
    })
    test('Page: 5/10', () => {
      expect(
        getPaginatorPages({ page: 5, totalCount: 10, pageSize: 1 })
      ).toEqual([1, Divider, 3, 4, 5, 6, 7, Divider, 10])
    })
    test('Page: 7/10', () => {
      expect(
        getPaginatorPages({ page: 7, totalCount: 10, pageSize: 1 })
      ).toEqual([1, Divider, 5, 6, 7, 8, 9, 10])
    })
    test('Page: 8/10', () => {
      expect(
        getPaginatorPages({ page: 8, totalCount: 10, pageSize: 1 })
      ).toEqual([1, Divider, 5, 6, 7, 8, 9, 10])
    })
    test('Page: 10/10', () => {
      expect(
        getPaginatorPages({ page: 10, totalCount: 10, pageSize: 1 })
      ).toEqual([1, Divider, 5, 6, 7, 8, 9, 10])
    })
  })
})
