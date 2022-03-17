import marked from 'marked'
import { renderer } from './renderer-extension'

marked.use({ renderer })

export interface ParsedMarkdown {
  parsed: string
  hasDiagrams: boolean
}

export function parseField<T>(field: keyof T) {
  return (item: T) => {
    const tokens = marked.lexer(item[field])
    const parsed = marked.parser(tokens)
    const hasDiagrams = tokens.some(({type, lang }) => type === 'code' && lang === 'mermaid')
    return {
      ...item,
      [field]: { parsed, hasDiagrams }
    }
  }
}
