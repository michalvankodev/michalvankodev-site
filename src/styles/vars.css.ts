import { createGlobalTheme } from '@vanilla-extract/css'
import {
  desaturate,
  lighten,
  mix,
  modularScale,
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

const createScale =
  (base: number, ratio: number, unit = 'em') =>
  (steps: number) =>
    `${modularScale(steps, base, ratio)}${unit}`

const spaceScale = createScale(0.2, 2)
const fontSizeScale = createScale(1, 1.125)
const lineHeightScale = createScale(1.05, 1.125)
// const borderRadiusScale = createScale(1.5, 4)

export const vars = createGlobalTheme(':root', {
  space: {
    none: '0',
    auto: 'auto',
    '0x': spaceScale(0),
    '1x': spaceScale(1),
    '2x': spaceScale(2),
    '3x': spaceScale(3),
    '4x': spaceScale(4),
    '5x': spaceScale(5),
    '6x': spaceScale(6),
    '7x': spaceScale(7),
    '8x': spaceScale(8),
  },
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
  fontFamily: {
    body: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol"',
  },
  fontSize: {
    xs: fontSizeScale(-2),
    sm: fontSizeScale(-1),
    base: fontSizeScale(0),
    xl: fontSizeScale(1),
    '2x': fontSizeScale(2),
    '3x': fontSizeScale(3),
    '4x': fontSizeScale(4),
    '5x': fontSizeScale(5),
    '6x': fontSizeScale(6),
  },
  lineHeight: {
    '0x': lineHeightScale(0),
    '1x': lineHeightScale(1),
    '2x': lineHeightScale(2),
    '3x': lineHeightScale(3),
    '4x': lineHeightScale(4),
    '5x': lineHeightScale(5),
  },
  fontWeight: {
    thin: 'thin',
    normal: 'normal',
    bold: 'bold',
  },
  width: {
    s: '400px',
    m: '700px',
    image: '800px',
    l: '1000px',
    max: '1140px',
    full: '100vw',
    parent: '100%',
  },
})
