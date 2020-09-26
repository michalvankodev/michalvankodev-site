import marked from 'marked'

export function parseField<T>(field: string) {
  return (item: T) => ({
    ...item,
    [field]: marked(item[field]),
  })
}
