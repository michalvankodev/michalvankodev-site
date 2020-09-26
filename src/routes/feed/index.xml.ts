import { getFeed } from './_feed'

export async function get(req, res) {
  const feed = await getFeed()

  res.writeHead(200, {
    'Content-Type': 'application/xml',
  })
  res.end(feed.rss2())
}
