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
  image: {
    source: string
    image_description: string
  }
}

export interface PortfolioAttributes {
  title: string
  work_history: RecordAttributes[]
  work_history_prelude: string
  projects: ProjectAttributes[]
  education: RecordAttributes[]
}

export type PortfolioContent = {
  title: string
  workHistory: RecordAttributes[]
  workHistoryPrelude: string
  projects: ProjectAttributes[]
  education: RecordAttributes[]
  body: string
}

export async function get() {
  let pageSource: string
  try {
    pageSource = await promisify(readFile)('_pages/portfolio.md', 'utf-8')
  } catch (e) {
    return {
      status: 500,
      body: 'Error loading portfolio source file. \n' + e.toString(),
    }
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

  const response: PortfolioContent = {
    title: parsed.attributes.title,
    body: marked(parsed.body),
    workHistoryPrelude: marked(parsed.attributes.work_history_prelude),
    workHistory,
    projects,
    education,
  }

  return {
    status: 200,
    body: response,
  }
}
