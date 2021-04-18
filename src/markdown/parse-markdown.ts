import marked from 'marked'
import { renderer } from './renderer-extension'

marked.use({ renderer })

export function parseField<T>(field: string) {
  return (item: T) => ({
    ...item,
    [field]: marked(item[field]),
  })
}
