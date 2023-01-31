import type { RequestHandler } from '@sveltejs/kit'
import { getFeed } from '../feed'

export const prerender = true
export const GET = (async ({ setHeaders }) => {
  const feed = await getFeed()

  setHeaders({
    'Content-Type': 'application/json',
    'Cache-Control': 'max-age=86400',
  })
  return new Response(feed.json1())
}) satisfies RequestHandler
