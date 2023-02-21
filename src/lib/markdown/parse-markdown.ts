import marked from 'marked'
import { renderer } from './renderer-extension'

marked.use({ renderer })

export function parseField<T>(field: keyof T) {
  return (item: T): T => ({
    ...item,
    [field]: marked(item[field]),
  })
}
