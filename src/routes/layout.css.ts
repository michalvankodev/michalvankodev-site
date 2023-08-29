import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '$lib/styles/vars.css'
import { sprinkles } from '$lib/styles/sprinkles.css'

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
  padding: {
    mobile: '3x',
    desktop: 'none',
  },
})

// Layout global styles
// atomic design needs to get rid off these global selectors LOL
// There should be written markdown renderer for each type of output
// where every component gets the layout atomic class
// TODO Create atomic classes for maxWidhts and use them everywhere in the content

globalStyle(
  `${mainContentClass} h1, ${mainContentClass} h2, ${mainContentClass} h3, ${mainContentClass} h4, ${mainContentClass} h5, ${mainContentClass} h6, ${mainContentClass} p, ${mainContentClass} ul, ${mainContentClass} ol, ${mainContentClass} figure, ${mainContentClass} img, ${mainContentClass} blockquote, ${mainContentClass} iframe:not(.embed), ${mainContentClass} footer, ${mainContentClass} table`,
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

globalStyle(`${mainContentClass} iframe:not(.embed)`, {
  maxWidth: vars.width.additionalBlockMax,
  display: 'block',
})

globalStyle(`${mainContentClass} img`, {
  maxWidth: vars.width.parent,
  borderRadius: 5,
  boxShadow: vars.boxShadow.contentBoxShadow,
})

globalStyle(`${mainContentClass} table`, {
  maxWidth: vars.width.image,
  fontSize: vars.fontSize.sm,
  lineHeight: vars.lineHeight['3x'],
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
