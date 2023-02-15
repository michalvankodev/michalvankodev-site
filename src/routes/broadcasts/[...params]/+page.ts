import { parseParams } from '$lib/pagination/dropTakeParams'
import type { PageLoad } from './$types'
import type { ArticleContent } from '$lib/content/articleContentListing'
import type { PaginationResult } from '$lib/pagination/pagination'

export const load = (async ({ fetch, params }) => {
  const { page = 1, pageSize = 7, ...filters } = parseParams(params.params)
  const articleResponse = await fetch(
    `/articles/segments/broadcasts${params.params ? `/${params.params}` : ''}.json`
  ).then((r) => r.json())

  return {
    posts: articleResponse.posts as PaginationResult<ArticleContent>,
    page: Number(page),
    pageSize,
    filters,
  }
}) satisfies PageLoad
