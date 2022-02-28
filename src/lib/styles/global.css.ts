import { globalStyle } from '@vanilla-extract/css'
import { breakpoints, colors, vars } from './vars.css'

globalStyle('html', {
  scrollBehavior: 'smooth',
})

globalStyle('body', {
  margin: 0,
  fontFamily:
    'cantarell, roboto, -apple-system, blinkmacsystemfont, segoe ui, oxygen, ubuntu, fira sans, droid sans, helvetica neue, sans-serif',
  fontSize: '16px',
  lineHeight: 1.65,
  color: vars.color.articleText,
  background: vars.color.background,
  minHeight: '100vh',
  '@media': {
    [`screen and (min-width: ${breakpoints.s}px)`]: {
      fontSize: '18px',
    },
    [`screen and (min-width: ${breakpoints.m}px)`]: {
      fontSize: '24px',
    },
    print: {
      fontSize: '12px',
    }
  },
})

globalStyle('h1, h2, h3, h4, h5, h6', {
  marginTop: vars.space['2x'],
  marginBottom: vars.space['1x'],
  marginLeft: vars.space.none,
  marginRight: vars.space.none,
  lineHeight: vars.lineHeight['1x'],
  color: vars.color.header,
  fontWeight: 500,
  letterSpacing: '-0.01em',
})

globalStyle('h1', {
  fontSize: vars.fontSize['5x'],
  fontWeight: 800,
})

globalStyle('h2', {
  fontSize: vars.fontSize['4x'],
  fontWeight: 700,
})

globalStyle('h3', {
  fontSize: vars.fontSize['3x'],
})

globalStyle('h4', {
  fontSize: vars.space['2x'],
})

globalStyle('a', {
  textDecoration: 'none',
  transition: 'color 0.2s',
})

globalStyle('a:link', {
  color: vars.color.link,
})

globalStyle('a:hover', {
  color: vars.color.linkHover,
  textDecoration: 'underline',
})

globalStyle('a:visited', {
  color: vars.color.linkVisited,
})

globalStyle('a:visited:hover', {
  color: vars.color.linkVisitedHover,
})

globalStyle('main pre, main pre[class*="language-"], main :not(pre) > code', {
  fontFamily: 'menlo, inconsolata, monospace',
  backgroundColor: vars.color.codeBackground,
  paddingTop: vars.space['1x'],
  paddingBottom: vars.space['1x'],
  paddingLeft: vars.space['1x'],
  paddingRight: vars.space['1x'],
  color: vars.color.code,
  lineHeight: vars.lineHeight['0x'],
  boxShadow: vars.boxShadow.codeBoxShadow,
  borderRadius: 3,
})

globalStyle('main code, main code[class*="language-"]', {
  fontSize: vars.fontSize.sm,
  '@media': {
    [`screen and (min-width: ${breakpoints.m}px)`]: {
      fontSize: vars.fontSize.xs,
    },
  },
})

globalStyle('code', {
  whiteSpace: 'pre-line',
})

globalStyle('pre code', {
  whiteSpace: 'pre',
})

globalStyle('figure', {
  marginTop: vars.space['2x'],
  marginBottom: vars.space['2x'],
  marginLeft: vars.space['1x'],
  marginRight: vars.space['1x'],
  textAlign: 'center',
})

globalStyle('figcaption', {
  fontSize: vars.fontSize.xs,
  fontStyle: 'italic',
})

globalStyle('blockquote', {
  lineHeight: vars.lineHeight['2x'],
  margin: vars.space['0x'],
  paddingLeft: vars.space['2x'],
  paddingRight: vars.space['0x'],
  paddingTop: vars.space['1x'],
  paddingBottom: vars.space['2x'],
  background: vars.color.quoteBackground,
  borderRadius: 3,
  borderLeft: `2px solid ${colors.tearkiss}`,
  boxShadow: vars.boxShadow.contentBoxShadow,
  fontSize: vars.fontSize.sm,
  overflow: 'auto',
})

globalStyle('blockquote p', {
  marginTop: vars.space['0x'],
  marginBottom: vars.space['0x'],
})

globalStyle('p', {
  marginTop: vars.space['1x'],
  marginBottom: vars.space['1x'],
})

globalStyle('b, strong', {
  fontWeight: 600,
})

globalStyle('::selection', {
  background: vars.color.selection,
})
