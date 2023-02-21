import { globalStyle } from '@vanilla-extract/css'
import { vars } from '$lib/styles/vars.css'
import { sprinkles } from '$lib/styles/sprinkles.css'

export const postListClass = sprinkles({
  padding: 'none',
  lineHeight: '3x',
  listStyle: 'none',
})

export const seeAllClass = sprinkles({
  textAlign: 'end',
  width: 'parent',
  maxWidth: 'max',
  margin: 'auto',
})

globalStyle(`${postListClass} > li:not(:last-child)`, {
  marginBottom: vars.space['4x'],
})
