import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import { parseField } from '../../markdown/parse-markdown'
import type { PostAttributes } from './_content'
import type { Request, Response } from '@sveltejs/kit'

export async function get({ params }: Request): Promise<Response> {
  // the `slug` parameter is available because
  // this file is called [slug].json.js
  const { slug } = params

  let postSource: string
  try {
    postSource = await promisify(readFile)(`_posts/blog/${slug}.md`, 'utf-8')
  } catch (e) {
    if (e.code === 'ENOENT') {
      return {
        status: 404,
        body: 'Post not found \n' + e.toString(),
        headers: {},
      }
    }
    return {
      status: 500,
      body: 'Error loading post source file. \n' + e.toString(),
      headers: {},
    }
  }

  const parsedPost = fm<PostAttributes>(postSource)

  const response = parseField('body')({
    ...parsedPost.attributes,
    body: parsedPost.body,
  })

  return {
    body: response,
  }
}
