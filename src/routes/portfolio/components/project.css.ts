import { globalStyle, style } from '@vanilla-extract/css'

export const projectScopeClass = style({})

globalStyle(`${projectScopeClass} img`, {
  float: 'right',
  width: '25%',
})

// We need to get rid off the global selectors LOL
