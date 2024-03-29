import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '$lib/styles/vars.css'

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

globalStyle(`${contentClass} .video-embed`, {
  margin: '0 auto',
  maxWidth: vars.width.image,
  aspectRatio: vars.aspectRatio.monitor,
})
