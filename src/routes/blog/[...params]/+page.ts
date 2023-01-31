import { parseParams } from '$lib/pagination/dropTakeParams'
import type { PageLoad } from './$types'
import type { PostContent } from './../content'
import type { PaginationResult } from '$lib/pagination/pagination'

export const load = (async ({ fetch, params }) => {
  const { page = 1, pageSize = 7, ...filters } = parseParams(params.params)
  const articleResponse = await fetch(
    `/blog/articles/${params.params ? params.params : 'index'}`
  ).then((r) => r.json())

  return {
    posts: articleResponse.posts as PaginationResult<PostContent>,
    page: Number(page),
    pageSize,
    filters,
  }
}) satisfies PageLoad
