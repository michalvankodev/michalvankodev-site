import {
  getDropTakeFromPageParams,
  parseParams,
} from '$lib/pagination/dropTakeParams'
import { json } from '@sveltejs/kit'
import { getBlogListing } from '../../content'
import type { RequestHandler } from './$types'

export const prerender = true
export const GET = (async ({ params }) => {
  const handledParams = params.params === 'index' ? '' : params.params
  const { page = 1, pageSize = 7, ...filters } = parseParams(handledParams)
  const paginationParams = getDropTakeFromPageParams(
    Number(pageSize),
    Number(page)
  )
  const paginationQuery = { ...paginationParams, filters }
  const filteredContents = await getBlogListing(paginationQuery)

  return json({
    posts: filteredContents,
  })
}) satisfies RequestHandler
