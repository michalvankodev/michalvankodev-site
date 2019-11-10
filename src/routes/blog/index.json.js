import { readdir, readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import marked from 'marked'

export async function get(req, res) {
  const files = await promisify(readdir)(`_posts/blog/`, 'utf-8')

  const contents = await Promise.all(
    files.map(async file => {
      const fileContent = await promisify(readFile)(
        `_posts/blog/${file}`,
        'utf-8'
      )
      const parsedAttributes = fm(fileContent)

      const lineOfTextRegExp = /^(?:\w|\[).+/gm
      const lines = parsedAttributes.body
        .match(lineOfTextRegExp)
        .slice(0, 4)
        .join('\n')

      const preview = marked(lines)
      return {
        ...parsedAttributes.attributes,
        preview,
      }
    })
  )

  res.writeHead(200, {
    'Content-Type': 'application/json',
  })

  res.end(JSON.stringify(contents))
}
