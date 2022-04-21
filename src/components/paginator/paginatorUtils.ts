import { toParams } from '$lib/pagination/dropTakeParams'
import { last, range } from 'ramda'

export const Divider = 'divider'

export function getPaginatorPages({
  page,
  pageSize,
  totalCount,
}: {
  page: number
  pageSize: number
  totalCount: number
}) {
  const maxLinksLength = 7
  const linksAroundActive = 2
  const totalPages = Math.ceil(totalCount / pageSize)
  const daco = range(1, totalPages + 1).reduce((acc, link) => {
    const isFirst = link === 1
    const isLast = link === totalPages
    const isPageOnStart = page <= 3 && link < maxLinksLength
    const isPageOnEnd =
      page >= totalPages - 3 && link > totalPages - maxLinksLength + 1

    if ([isFirst, isLast, isPageOnStart, isPageOnEnd].some((value) => value)) {
      return [...acc, link]
    }

    if (link < page - linksAroundActive || link > page + linksAroundActive) {
      if (last(acc) === Divider) {
        return acc
      }
      return [...acc, Divider]
    }

    return [...acc, link]
  }, [])

  return daco
}

export function createHref(
  href: string,
  filters: Record<string, string>,
  pageNumber: number
) {
  const filtersPath = toParams(filters)
  return `/${href}/${filtersPath ? filtersPath + '/' : ''}page/${pageNumber}`
}
