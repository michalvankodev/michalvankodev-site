import { style } from '@vanilla-extract/css'
import { desaturate, transparentize } from 'polished'
import { colors } from '../../styles/vars.css'
import { sprinkles } from '../../styles/sprinkles.css'

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

export const footerClass = style([
  sprinkles({
    display: 'flex',
    fontSize: 'sm',
    justifyContent: 'space-between',
    paddingTop: '1x',
    marginTop: '2x',
  }),
  {
    borderTop: `1px solid ${transparentize(
      0.6,
      desaturate(0.5, colors.tearkiss)
    )}`,
  },
])
