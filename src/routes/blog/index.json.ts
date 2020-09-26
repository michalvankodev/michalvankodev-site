import { getBlogListing } from './_content'

export async function get(req, res) {
  const { tag } = req.query
  const filteredContents = await getBlogListing(tag)
  res.writeHead(200, {
    'Content-Type': 'application/json',
  })
  res.end(JSON.stringify(filteredContents))
}
