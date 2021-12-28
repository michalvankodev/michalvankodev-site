import { style } from '@vanilla-extract/css'
import { desaturate, transparentize } from 'polished'
import { colors } from './vars.css'

export const horizontalBorderTopClass = style({
  borderTop: `1px solid ${transparentize(
    0.6,
    desaturate(0.5, colors.tearkiss)
  )}`,
})
