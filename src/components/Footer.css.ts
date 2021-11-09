import { style } from '@vanilla-extract/css'
import { radialGradient, transparentize } from 'polished'
import {
  breakpoints,
  mediaAt,
  menuBackground,
  transparent,
  vars,
} from '../styles/vars.css'

export const siteFooterStyle = style({
  fontSize: '0.9em',
  padding: '2em 0.8em 0',
  color: vars.color.menuLink,

  ...radialGradient({
    colorStops: [
      `${menuBackground}  56%`,
      `${transparentize(0.4, menuBackground)} 100%`,
    ],
    extent: '160% 100% at 100% 100%',
    fallback: transparent,
  }),

  '@media': {
    [mediaAt(breakpoints.m)]: {
      ...radialGradient({
        colorStops: [
          `${menuBackground} 48%`,
          `${transparentize(1, menuBackground)} 100%`,
        ],
        extent: '140% 100% at 100% 100%',
        fallback: transparent,
      }),
    },
    [mediaAt(breakpoints.l)]: {
      fontSize: '0.8em',
    },
  },
})
