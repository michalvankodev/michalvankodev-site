import type { LayoutLoad } from './$types'

export const load = (async ({ fetch }) => {
  const blogPostsResponse = await fetch(`/blog/articles/pageSize/5`)
  const blogPostsContent = await blogPostsResponse.json()

  return {
    latestPosts: blogPostsContent.posts.items,
  }
}) satisfies LayoutLoad
