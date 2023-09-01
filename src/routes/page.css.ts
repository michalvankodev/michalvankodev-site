import { sprinkles } from '$lib/styles/sprinkles.css'
import {
  transparent,
  twitchEmbedBackground,
  mediaAt,
  breakpoints,
} from '$lib/styles/vars.css'
import { style } from '@vanilla-extract/css'
import { radialGradient, transparentize } from 'polished'

export const profilePicClass = sprinkles({
  textAlign: 'center',
  marginX: 'auto',
  marginY: 'none',
})

export const profilePicImgClass = style({
  aspectRatio: "auto 800 / 709",
  maxHeight: '66vh'
})

export const mottoClass = sprinkles({
  textAlign: 'center',
  marginX: 'auto',
  marginY: '2x',
  fontSize: '2x',
})

export const welcomeNoteClass = sprinkles({
  textAlign: 'center',
  marginX: 'auto',
})

export const citeOwnerClass = sprinkles({
  whiteSpace: 'nowrap',
})

export const twitchStreamPromoClass = style([
  radialGradient({
    colorStops: [
      `${twitchEmbedBackground} 0%`,
      `${transparentize(1, twitchEmbedBackground)} 60%`,
    ],
    extent: '90% 40% at 50% 70%',
    fallback: transparent,
  }),
  {
    '@media': {
      [mediaAt(breakpoints.l)]: radialGradient({
        colorStops: [
          `${twitchEmbedBackground} 0%`,
          `${transparentize(1, twitchEmbedBackground)} 100%`,
        ],
        extent: '180% 50% at 100% 50%',
        fallback: transparent,
      }),
    },
  },
])

export const twitchIframeClass = sprinkles({
  flexGrow: 1,
  maxWidth: 'image',
  aspectRatio: 'monitor',
  width: {
    mobile: 'parent',
    desktop: 'auto',
  },
})

export const twitchEmbedClass = sprinkles({
  display: 'flex',
  padding: '3x',
  justifyContent: 'center',
  alignItems: 'center',
  flexDirection: {
    mobile: 'column',
    desktop: 'row',
  },
  gap: '4x',
  width: {
    mobile: 'parent',
    desktop: 'auto',
  },
})

export const twitchAsideClass = sprinkles({
  width: {
    mobile: 'parent',
    desktop: 's',
  },
})
