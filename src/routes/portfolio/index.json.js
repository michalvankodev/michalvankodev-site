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
  const workHistory = (parsed.attributes.work_history || []).map(parseField('description'))
  const projects = (parsed.attributes.projects || [])
    .filter(project => project.displayed)
    .map(parseField('description'))
  const education = (parsed.attributes.education || [])
    .filter(education => education.displayed)
    .map(parseField('description'))
  
  const response = {
    title: parsed.attributes.title,
    body: marked(parsed.body),
    workHistoryPrelude: marked(parsed.attributes.work_history_prelude),
    workHistory,
    projects,
    education,
  }

  res.setHeader('Content-Type', 'application/json')
  res.end(JSON.stringify(response))
}

function parseField(field) {
  return item => ({
    ...item,
    [field]: marked(item[field])
  })
}