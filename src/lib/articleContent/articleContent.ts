import { error } from '@sveltejs/kit'
import fm from 'front-matter'
import { readFile } from 'fs'
import { parseField } from '$lib/markdown/parse-markdown'
import { promisify } from 'util'

export interface ArticleAttributes {
  slug: string
  layout: string
  segments: string[]
  title: string
  published: boolean
  date: string
  thumbnail: string
  tags: string[]
  body: string
}

export async function getArticleContent(slug: string) {
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

  const parsedPost = fm<ArticleAttributes>(postSource)

  const post = parseField<ArticleAttributes>('body')({
    ...parsedPost.attributes,
    body: parsedPost.body,
  })
  return post
}
