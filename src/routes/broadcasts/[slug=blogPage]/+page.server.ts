import type { PageServerLoad } from './$types'
import { getArticleContent } from '$lib/articleContent/articleContent'

export const prerender = true

export const load = (async ({ params: { slug } }) => {
  const post = await getArticleContent(slug);
  return post
}) satisfies PageServerLoad
