import { readdir, readFile } from 'fs'
import { promisify } from 'util'
import { basename } from 'path'
import { pipe, prop, sortBy, reverse, filter } from 'ramda'
import fm from 'front-matter'
import marked from 'marked'
import {
  filterAndCount,
  type PaginationQuery,
} from '$lib/pagination/pagination'

const { NODE_ENV } = process.env
// TODO remove ramda and migrate to ts-belt
// TODO Pagination component for routing
// TODO Tag filtering visualization

export interface PostAttributes {
  layout: string
  title: string
  published: boolean
  date: string
  thumbnail: string
  tags: string[]
}

export interface PostContent extends PostAttributes {
  preview: string
  slug: string
  published: boolean
}

export async function getBlogListing(paginationQuery: PaginationQuery) {
  const files = await promisify(readdir)(`_posts/blog/`, 'utf-8')
  const filteredFiles = filterDevelopmentFiles(files)

  const contents = await Promise.all(
    filteredFiles.map(async (file) => {
      const fileContent = await promisify(readFile)(
        `_posts/blog/${file}`,
        'utf-8'
      )
      const parsedAttributes = fm<PostAttributes>(fileContent)

      const lineOfTextRegExp = /^(?:\w|\[).+/gm
      const lines = parsedAttributes.body
        .match(lineOfTextRegExp)
        .slice(0, 2)
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
    sortBy<PostContent>(prop('date')),
    (items) => reverse(items),
    filter<typeof contents[0]>((article) => article.published),
    filterAndCount(paginationQuery)
  )(contents)

  return filteredContents
}

function filterDevelopmentFiles(files: string[]) {
  return NODE_ENV !== 'production'
    ? files
    : files.filter((file) => !file.startsWith('dev-'))
}
