import { map, multiply } from 'ramda'

export interface ImageOptions {
  width?: number
  height?: number
}

/**
 * Get the URL for resource with specified parameters for Netlify Large Media trasformation
 *
 * @see https://docs.netlify.com/large-media/transform-images/
 */
export function getNFResize(href: string, { width, height }: ImageOptions) {
  return `${href}?nf_resize=fit${height ? `&h=${height}` : ''}${
    width ? `&w=${width}` : ''
  }`
}

export const PIXEL_DENSITIES = [1, 1.5, 2, 3, 4]

function multiplyImageOptions(
  multiplier,
  imageOptions: ImageOptions
): ImageOptions {
  return map(multiply(multiplier), imageOptions)
}

/**
 * Generate `srcset` attribute for all `PIXEL_DENSITIES` to serve images in appropriate quality
 * for each device with specific density
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset
 */
export function generateSrcSet(href: string, imageOptions: ImageOptions) {
  return PIXEL_DENSITIES.map(
    (density) =>
      `${getNFResize(
        href,
        multiplyImageOptions(density, imageOptions)
      )} ${density}x`
  ).join(',')
}
