import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import marked from 'marked'
import { parseField } from '../../markdown/parse-markdown'
import type { PageServerLoad } from './$types'

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

export interface WorkAttributes extends RecordAttributes {
  address: {
    name: string
    location: string
    zipcode: string
    city: string
    country: string
  }
}

export interface PresentationAttributes extends RecordAttributes {
  link: string
}

export interface PortfolioAttributes {
  title: string
  work_history: WorkAttributes[]
  work_history_prelude: string
  projects: ProjectAttributes[]
  education: RecordAttributes[]
  presentations: PresentationAttributes[]
}

export type PortfolioContent = {
  title: string
  workHistory: WorkAttributes[]
  workHistoryPrelude: string
  projects: ProjectAttributes[]
  education: RecordAttributes[]
  presentations: PresentationAttributes[]
  body: string
}

export const load = (async () => {
  const pageSource = await promisify(readFile)('_pages/portfolio.md', 'utf-8')

  const parsed = fm<PortfolioAttributes>(pageSource)
  const workHistory = (parsed.attributes.work_history || [])
    .filter((workHistory) => workHistory.displayed)
    .map(parseField('description'))
  const projects = (parsed.attributes.projects || [])
    .filter((project) => project.displayed)
    .map(parseField('description'))
  const education = (parsed.attributes.education || [])
    .filter((education) => education.displayed)
    .map(parseField('description'))
  const presentations = (parsed.attributes.presentations || []).filter(
    (education) => education.displayed
  )

  const response: PortfolioContent = {
    title: parsed.attributes.title,
    body: marked(parsed.body),
    workHistoryPrelude: marked(parsed.attributes.work_history_prelude),
    workHistory,
    projects,
    education,
    presentations,
  }

  return response;
}) satisfies PageServerLoad
