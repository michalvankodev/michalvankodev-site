import { Feed } from 'feed'
import { getBlogListing } from '../blog/_content'

export async function getFeed() {
  const feed = new Feed({
    title: 'michalvanko.dev latest posts',
    id: 'https://michalvanko.dev',
    link: 'https://michalvanko.dev',
    description: 'Latest posts published on michalvanko.dev',
    copyright: 'All rights reserved 2020, Michal Vanko',
    generator: 'sapper with Feed for node.js',
    updated: new Date(),
    image: 'https://michalvanko.dev/eye.png',
    favicon: 'https://michalvanko.dev/m-favicon-192x192.png',
    language: 'en',
    author: {
      name: 'Michal Vanko',
      email: 'michalvankosk@gmail.com',
      link: 'https://michalvanko.dev',
    },
    feedLinks: {
      json: 'https://michalvanko.dev/feed.json',
      rss: 'https://michalvanko.dev/feed.xml',
    },
  })

  const blogListing = await getBlogListing()
  blogListing.forEach((post) => {
    feed.addItem({
      title: post.title,
      id: `https://michalvanko.dev/blog/${post.slug}`,
      link: `https://michalvanko.dev/blog/${post.slug}`,
      description: post.preview,
      date: new Date(post.date),
      image: post.thumbnail
        ? `https://michalvanko.dev/${post.thumbnail}`
        : undefined,
    })
  })
  return feed
}
