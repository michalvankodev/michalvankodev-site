import { getBlogListing } from './_content'

export async function get({ query }) {
  const tag = query.get('tag')
  const filteredContents = await getBlogListing(tag)
  return {
    status: 200,
    body: filteredContents,
  }
}
