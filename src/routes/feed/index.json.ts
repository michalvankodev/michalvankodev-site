import { getFeed } from './_feed'

export async function get() {
  const feed = await getFeed()

  return {
    status: 200,
    body: feed.json1(),
  }
}
