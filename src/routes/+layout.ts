import type { LayoutLoad } from './$types';

export const load = (async ({ fetch, url }) => {
  const blogPostsResponse = await fetch(`/blog/articles/pageSize/5.json`)
  const blogPostsContent = await blogPostsResponse.json()

  return {
    latestPosts: blogPostsContent.posts.items,
    // TODO Check if not bugged FIXME
    segment: '',
  }
}) satisfies LayoutLoad
