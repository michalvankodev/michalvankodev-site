import marked from 'marked'

export function parseField(field) {
  return item => ({
    ...item,
    [field]: marked(item[field]),
  })
}
