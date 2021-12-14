import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '../../src/styles/vars.css'
import { sprinkles } from '../../src/styles/sprinkles.css'

export const appContentClass = style([
  sprinkles({
    display: 'grid',
  }),
  {
    gridTemplateRows: 'auto 1fr auto',
    gridTemplateColumns: '100%',
  },
])

export const mainContentClass = sprinkles({
  position: 'relative',
  padding: '3x',
})

// Layout global styles

globalStyle(
  `${mainContentClass} h1, ${mainContentClass} h2, ${mainContentClass} h3, ${mainContentClass} h4, ${mainContentClass} h5, ${mainContentClass} h6, ${mainContentClass} p, ${mainContentClass} ul, ${mainContentClass} ol, ${mainContentClass} figure, ${mainContentClass} img, ${mainContentClass} blockquote, ${mainContentClass} iframe, ${mainContentClass} footer`,
  {
    maxWidth: vars.width.layoutMax,
    marginLeft: 'auto',
    marginRight: 'auto',
  }
)

globalStyle(`${mainContentClass} h1, ${mainContentClass} footer`, {
  maxWidth: vars.width.headerFooterMax,
})

globalStyle(`${mainContentClass} h2`, {
  maxWidth: vars.width.additionalBlockMax,
})

globalStyle(`${mainContentClass} iframe`, {
  maxWidth: vars.width.additionalBlockMax,
  display: 'block',
})

globalStyle(`${mainContentClass} img`, {
  width: vars.width.parent,
  borderRadius: 5,
  boxShadow: vars.boxShadow.contentBoxShadow,
})

globalStyle(`${mainContentClass} figure`, {
  maxWidth: vars.width.image,
})

globalStyle(
  `${mainContentClass} pre, ${mainContentClass} pre[class*="language-"]`,
  {
    maxWidth: vars.width.additionalBlockMax,
    marginLeft: 'auto',
    marginRight: 'auto',
  }
)
