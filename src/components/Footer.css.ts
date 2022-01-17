import { globalStyle, style } from '@vanilla-extract/css'
import { radialGradient, rgba, transparentize } from 'polished'
import { sprinkles } from '$lib/styles/sprinkles.css'
import {
  breakpoints,
  colors,
  mediaAt,
  menuBackground,
  transparent,
  vars,
} from '$lib/styles/vars.css'

export const siteFooterClass = style([
  sprinkles({
    fontSize: { mobile: 'base', desktop: 'sm' },
    paddingX: '2x',
    paddingTop: '1x',
    color: 'menuLink',
  }),

  radialGradient({
    colorStops: [
      `${menuBackground}  56%`,
      `${transparentize(0.4, menuBackground)} 100%`,
    ],
    extent: '160% 100% at 100% 100%',
    fallback: transparent,
  }),
  {
    '@media': {
      [mediaAt(breakpoints.m)]: radialGradient({
        colorStops: [
          `${menuBackground} 48%`,
          `${transparentize(1, menuBackground)} 100%`,
        ],
        extent: '140% 100% at 100% 100%',
        fallback: transparent,
      }),
    },
  },
])

export const headerClass = sprinkles({
  fontWeight: 'bold',
  fontSize: 'base',
  color: 'menuLink',
  margin: 'none',
  lineHeight: '3x',
  marginBottom: '1x',
})

export const sectionListsClass = style([
  sprinkles({
    display: 'grid',
    justifyItems: { mobile: 'center', desktop: 'start' },
    textAlign: { mobile: 'center', desktop: 'start' },
    maxWidth: 'max',
    columnGap: '3x',
    margin: 'auto',
  }),
  {
    '@media': {
      [mediaAt(breakpoints.l)]: {
        gridTemplateColumns: 'auto auto auto',
      },
    },
  },
])

export const sectionListSectionClass = sprinkles({
  marginY: '3x',
})

export const noWrapClass = sprinkles({
  whiteSpace: 'nowrap',
})

export const listUlClass = sprinkles({
  listStyle: 'none',
  padding: 'none',
  margin: 'none',
})

export const listLiClass = sprinkles({
  marginLeft: '1x',
})

export const nestedListLiClass = style([
  listLiClass,
  sprinkles({
    fontSize: 'sm',
  }),
])

export const socialLinkLabelClass = sprinkles({
  paddingX: '1x',
})

export const svgClass = style({
  fill: vars.color.menuLink,
  height: '1em',
  width: '1em',
})

export const strokeSvgClass = style([
  svgClass,
  {
    stroke: vars.color.menuLink,
    strokeWidth: '2px',
  },
])

export const socialLinkClass = sprinkles({
  display: 'flex',
  alignItems: 'center',
  justifyContent: {
    mobile: 'center',
    desktop: 'start',
  },
})

export const bottomLineClass = sprinkles({
  display: 'flex',
  justifyContent: 'space-between',
  marginX: 'auto',
  paddingBottom: '1x',
  marginTop: '2x',
  maxWidth: 'max',
})

export const dateClass = sprinkles({
  fontSize: 'xs',
  whiteSpace: 'nowrap',
})

export const boldClass = sprinkles({
  fontWeight: 'bold',
})

export const hrClass = style([
  sprinkles({
    marginY: '2x',
    marginX: '1x',
  }),
  {
    color: rgba(colors.midnightBlue, 0.14),
    borderWidth: '1px 0 0',
  },
])

export const latestPostsClass = style({})

globalStyle(`${siteFooterClass} a`, {
  color: vars.color.menuLink,
})

globalStyle(`${headerClass} a:link, ${headerClass} a:visited`, {
  color: vars.color.menuLink,
})

globalStyle(`${siteFooterClass} a:hover`, {
  color: vars.color.menuLinkHover,
})

globalStyle(`${latestPostsClass} li a:visited:not(:hover)`, {
  color: vars.color.linkVisited,
})

