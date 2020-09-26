import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import marked from 'marked'
import { parseField } from '../../markdown/parse-markdown'

export interface RecordAttributes {
  name: string
  description: string
  displayed: boolean
}

export interface ProjectAttributes extends RecordAttributes {
  image: string
}

export interface PortfolioAttributes {
  title: string
  work_history_prelude: string
  work_history: string[]
  projects: ProjectAttributes[]
  education: RecordAttributes[]
}

export async function get(req, res, next) {
  let pageSource: string
  try {
    pageSource = await promisify(readFile)('_pages/portfolio.md', 'utf-8')
  } catch (e) {
    res.statusCode = 500
    res.end('Error loading portfolio source file. \n' + e.toString())
    return
  }

  const parsed = fm<PortfolioAttributes>(pageSource)
  const workHistory = (parsed.attributes.work_history || []).map(
    parseField('description')
  )
  const projects = (parsed.attributes.projects || [])
    .filter((project) => project.displayed)
    .map(parseField('description'))
  const education = (parsed.attributes.education || [])
    .filter((education) => education.displayed)
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
