import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import { parseField } from '../../markdown/parse-markdown'

export async function get(req, res, next) {
  // the `slug` parameter is available because
  // this file is called [slug].json.js
  const { slug } = req.params

  let postSource
  try {
    postSource = await promisify(readFile)(`_posts/blog/${slug}.md`, 'utf-8')
  } catch (e) {
    if (e.code === 'ENOENT') {
      res.statusCode = 404
      res.end('Post not found \n' + e.toString())
      return
    }
    res.statusCode = 500
    res.end('Error loading post source file. \n' + e.toString())
    return
  }

  const parsedPost = fm(postSource)
  const response = parseField('body')(parsedPost)

  res.setHeader('Content-Type', 'application/json')
  res.end(JSON.stringify(response))
}
