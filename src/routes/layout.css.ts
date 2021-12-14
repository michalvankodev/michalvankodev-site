import { style } from '@vanilla-extract/css'
import { sprinkles } from '../../src/styles/sprinkles.css'

export const appContentClass = style([
  sprinkles({
    display: 'grid',
  }),
  {
    gridTemplateRows: 'auto 1fr auto',
    gridTemplateColumns: '100%',
    minHeight: '100vh',
  },
])

export const mainContentClass = sprinkles({
  position: 'relative',
  padding: '3x',
})
