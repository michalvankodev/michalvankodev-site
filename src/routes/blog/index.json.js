import { readdir, readFile } from 'fs'
import { promisify } from 'util'
import { basename } from 'path'
import { pipe, partial, prop, sortBy, reverse, filter } from 'ramda'
import fm from 'front-matter'
import marked from 'marked'

const { NODE_ENV } = process.env

export async function get(req, res) {
  const { tag } = req.query
  const files = await promisify(readdir)(`_posts/blog/`, 'utf-8')
  const filteredFiles = filterDevelopmentFiles(files)

  const contents = await Promise.all(
    filteredFiles.map(async file => {
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
        slug: basename(file, '.md'),
      }
    })
  )
  const filteredContents = pipe(
    sortBy(prop('date')),
    reverse,
    filter(article => article.published),
    partial(filterByTag, [tag])
  )(contents)

  res.writeHead(200, {
    'Content-Type': 'application/json',
  })
  res.end(JSON.stringify(filteredContents))
}

function filterDevelopmentFiles(files) {
  return NODE_ENV !== 'production'
    ? files
    : files.filter(file => !file.startsWith('dev-'))
}

function filterByTag(tag, contents) {
  return tag ? contents.filter(content => content.tags.includes(tag)) : contents
}

function filterPublished(article) {
  return article.published
}
