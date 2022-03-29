import { getPaginationQueryFromSearchParams } from '$lib/pagination/searchParams'
import { getBlogListing } from './_content'

export async function get({ url: { searchParams } }) {
  const paginationQuery = getPaginationQueryFromSearchParams(searchParams)
  const filteredContents = await getBlogListing(paginationQuery)

  return {
    status: 200,
    body: {
      posts: filteredContents,
    },
  }
}
