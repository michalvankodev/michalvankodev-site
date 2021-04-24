import { getFeed } from './_feed'

export async function get(req, res) {
  const feed = await getFeed()

  return {
    status: 200,
    headers: {
      'Content-Type': 'application/xml',
    },
    body: feed.rss2(),
  }
}
