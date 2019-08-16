import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import marked from 'marked'

export async function get(req, res, next) {
  let pageSource
  try {
    pageSource = await promisify(readFile)('_pages/portfolio.md', 'utf-8')
  } catch (e) {
    res.statusCode = 500
    res.end('Error loading portfolio source file. \n' + e.toString())
    return
  }

  const parsed = fm(pageSource)
  const projects = (parsed.attributes.projects || []).map(project => ({
    ...project,
    description: marked(project.description)
  }))
  const response = {
    title: parsed.attributes.title,
    body: marked(parsed.body),
    projects,
  }

  res.setHeader('Content-Type', 'application/json')
  res.end(JSON.stringify(response))
}