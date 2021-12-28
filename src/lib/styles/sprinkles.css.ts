import { createSprinkles, defineProperties } from '@vanilla-extract/sprinkles'
import { breakpoints, vars } from './vars.css'

const responsiveProperties = defineProperties({
  conditions: {
    mobile: {},
    tablet: { '@media': `screen and (min-width: ${breakpoints.m}px)` },
    desktop: { '@media': `screen and (min-width: ${breakpoints.l}px)` },
  },
  defaultCondition: 'mobile',
  properties: {
    display: ['none', 'flex', 'block', 'inline', 'inline-block', 'grid'],
    position: ['relative', 'absolute', 'fixed'],
    flexDirection: ['row', 'column'],
    flexWrap: ['wrap', 'nowrap'],
    flexShrink: [0],
    flexGrow: [0, 1],
    justifyContent: [
      'stretch',
      'start',
      'center',
      'end',
      'space-around',
      'space-between',
    ],
    justifyItems: [
      'stretch',
      'start',
      'center',
      'end',
      'space-around',
      'space-between',
    ],
    alignItems: ['stretch', 'flex-start', 'center', 'flex-end'],
    flex: ['1'],
    textAlign: ['center', 'justify', 'start', 'end'],
    textShadow: vars.textShadow,
    paddingTop: vars.space,
    paddingBottom: vars.space,
    paddingLeft: vars.space,
    paddingRight: vars.space,
    marginTop: vars.space,
    marginBottom: vars.space,
    marginRight: vars.space,
    marginLeft: vars.space,
    columnGap: vars.space,
    fontSize: vars.fontSize,
    fontFamily: vars.fontFamily,
    fontWeight: vars.fontWeight,
    fontStyle: ['italic', 'normal'],
    lineHeight: vars.lineHeight,
    whiteSpace: ['normal', 'nowrap'],
    width: vars.width,
    maxWidth: vars.width,
    height: ['100vh', '100&'],
    listStyle: ['none'],
    overflow: ['auto'],
  },
  shorthands: {
    padding: ['paddingTop', 'paddingBottom', 'paddingLeft', 'paddingRight'],
    paddingX: ['paddingLeft', 'paddingRight'],
    paddingY: ['paddingTop', 'paddingBottom'],
    placeItems: ['justifyContent', 'alignItems'],
    typeSize: ['fontSize', 'lineHeight'],
    margin: ['marginTop', 'marginBottom', 'marginLeft', 'marginRight'],
    marginX: ['marginLeft', 'marginRight'],
    marginY: ['marginTop', 'marginBottom'],
  },
})

const colorProperties = defineProperties({
  properties: {
    color: vars.color,
    background: vars.color,
  },
})

export const sprinkles = createSprinkles(responsiveProperties, colorProperties)
