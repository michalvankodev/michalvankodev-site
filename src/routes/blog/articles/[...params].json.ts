import {
  getDropTakeFromPageParams,
  parseParams,
} from '$lib/pagination/dropTakeParams'
import { getBlogListing } from '../_content'

export async function get({ params }) {
  console.log('article-params', params)
  const handledParams = params.params === 'index' ? '' : params.params
  const { page = 1, pageSize = 7, ...filters } = parseParams(handledParams)
  const paginationParams = getDropTakeFromPageParams(
    Number(pageSize),
    Number(page)
  )
  const paginationQuery = { ...paginationParams, filters }
  const filteredContents = await getBlogListing(paginationQuery)

  return {
    status: 200,
    body: {
      posts: filteredContents,
    },
  }
}
