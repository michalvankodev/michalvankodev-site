import { parseParams } from '$lib/pagination/dropTakeParams'
import type { PageLoad } from './$types'
import type { ArticlePreviewAttributes } from '$lib/articleContent/articleContentListing'
import type { PaginationResult } from '$lib/pagination/pagination'

export const load = (async ({ fetch, params }) => {
  const { page = 1, pageSize = 7, ...filters } = parseParams(params.params)
  const articleResponse = await fetch(
    `/articles/segments/blog${params.params ? `/${params.params}` : ''}.json`
  ).then((r) => r.json())
  return {
    posts: articleResponse.posts as PaginationResult<ArticlePreviewAttributes >,
    page: Number(page),
    pageSize,
    filters,
  }
}) satisfies PageLoad
