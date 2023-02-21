import { sprinkles } from '$lib/styles/sprinkles.css'

export const tagsListClass = sprinkles({
  listStyle: 'none',
  margin: 'none',
  padding: 'none',
  display: 'inline',
})

export const tagsListLiClass = sprinkles({
  display: 'inline',
  fontStyle: 'italic',
})

export const publishedClass = sprinkles({
  whiteSpace: 'nowrap',
  fontStyle: 'italic',
})

export const publishedLabelClass = sprinkles({
  color: 'tintedText',
})

export const footerClass = sprinkles({
  display: 'flex',
  fontSize: 'sm',
  justifyContent: 'space-between',
  paddingTop: '1x',
  marginTop: '2x',
})
