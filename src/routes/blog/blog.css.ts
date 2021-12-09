import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from 'src/styles/vars.css'

export const contentClass = style({})

globalStyle(`${contentClass} ul, ${contentClass} ol`, {
  lineHeight: vars.lineHeight['2x'],
})

globalStyle(`${contentClass} li`, {
  marginBottom: vars.space['2x'],
})

globalStyle(`${contentClass} img`, {
  maxHeight: vars.height.image,
})

globalStyle(`${contentClass} img:only-child`, {
  display: 'block',
  margin: '0 auto',
})
