import { createGlobalTheme } from '@vanilla-extract/css'
import {
  desaturate,
  lighten,
  mix,
  saturate,
  tint,
  transparentize,
} from 'polished'

export const colors = {
  tearkiss: '#42a6f0',
  pinky: '#fea6eb',
  lightCyan: '#d8f6ff',
  midnightBlue: '#171664',
  frenchViolet: '#7332c3',
}

export const menuBackground = transparentize(0.6, colors.tearkiss)
export const transparent = transparentize(1, '#ffffff')

export enum breakpoints {
  s = 400,
  m = 700,
  image = 800,
  l = 1000,
  max = 1140,
}

export function mediaAt(breakpoint: breakpoints) {
  return `screen and (min-width: ${breakpoint}px)`
}

export const vars = createGlobalTheme(':root', {
  color: {
    articleText: desaturate(0.16, colors.midnightBlue),
    selection: tint(0.4, colors.pinky),
    link: saturate(0.2, mix(0.66, colors.tearkiss, colors.midnightBlue)),
    linkHover: colors.tearkiss,
    linkVisited: colors.frenchViolet,
    linkVisitedHover: lighten(0.1, colors.frenchViolet),

    menu: colors.midnightBlue,
    menuLink: colors.midnightBlue,
    menuLinkHover: lighten(0.15, colors.midnightBlue),

    header: lighten(0.1, colors.midnightBlue),
    background: tint(0.7, colors.lightCyan),
    menuBackground,
  },
})
