import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import { parseField } from '../../../markdown/parse-markdown'
import { error, json } from '@sveltejs/kit'
import type { PostAttributes } from '../content'
import type { PageServerLoad } from './$types'

export const prerender = true

export interface SinglePost {
  body: string
}

export const load = (async ({ params: { slug } }) => {
  let postSource: string
  try {
    postSource = await promisify(readFile)(`_posts/blog/${slug}.md`, 'utf-8')
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (e: any) {
    if (e.code === 'ENOENT') {
      throw error(404, 'Post not found \n' + e.toString())
    }
    throw e
  }

  const parsedPost = fm<PostAttributes>(postSource)

  const post = parseField<SinglePost>('body')({
    ...parsedPost.attributes,
    body: parsedPost.body,
  })

  return post
}) satisfies PageServerLoad
