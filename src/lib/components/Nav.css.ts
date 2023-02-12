import { globalStyle, style } from '@vanilla-extract/css'
import { radialGradient, transparentize } from 'polished'
import { menuBackground, transparent, vars } from '$lib/styles/vars.css'
import { sprinkles } from '$lib/styles/sprinkles.css'

export const navigationClass = style([
  sprinkles({
    paddingTop: '1x',
    paddingBottom: '2x',
    paddingX: '1x',
    color: 'menu',
    textShadow: 'menuLinkShadow',
  }),
  radialGradient({
    colorStops: [
      `${menuBackground} 0%`,
      `${transparentize(1, menuBackground)} 100%`,
    ],
    extent: '120% 100% at 0% 0%',
    fallback: transparent,
  }),
])

export const navigationContentClass = sprinkles({
  display: 'flex',
  maxWidth: 'max',
  marginY: 'none',
  marginX: 'auto',
})

export const navigationLinksClass = sprinkles({
  listStyle: 'none',
  margin: 'none',
  padding: 'none',
  display: 'flex',
  flex: '1',
})

export const logoSectionClass = sprinkles({
  lineHeight: 'none',
})

export const logoLinkClass = sprinkles({
  padding: 'none',
  display: 'block',
})

globalStyle(`${navigationClass} a:not(${logoLinkClass})`, {
  color: vars.color.menuLink,
  padding: vars.space['1x'],
})

globalStyle(`${navigationClass} a:hover`, {
  color: vars.color.menuLinkHover,
})

export const logoImgClass = style({
  height: vars.space['3x'],
})

export const selectedClass = sprinkles({
  textShadow: 'menuActiveLinkShadow',
})

export const portfolioPageNavigation = style({
  position: 'sticky',
  top: '0px',
  zIndex: 1,
  width: '100%',
  fontSize: vars.fontSize.sm,
  padding: vars.space['1x'],
  background: vars.color.background,
  boxShadow: `0px 0.5em 0.5em ${vars.color.background}`,
})

export const portfolioPageNavigationLinksClass = sprinkles({
  maxWidth: 'l',
  marginX: 'auto',
  marginY: 'none',
})

export const portfolioPageNavigationLinkClass = sprinkles({
  padding: '1x',
})
