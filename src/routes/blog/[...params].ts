import { getBlogListing } from './_content'

export async function get({ url: { searchParams } }) {
  console.log('bloglistingparams', searchParams)

  //Regexp for getting an optional tag and a page from the params
  const tag = undefined
  const filteredContents = await getBlogListing(tag)
  return {
    status: 200,
    body: {
      posts: filteredContents,
    },
  }
}
