import path from 'path'
import sharp from 'sharp'

const outputOptions = {
  jpeg: {
    quality: 90,
    mozjpeg: true,
  },
  png: {},
  webp: {
    quality: 90,
    // lossless: true,
  },
}
/**
 * Transform image into multiple optimized version for web browsing
 * Create a optimized image of same filetype and extra `webp`
 */
export async function optimizeImage(imgPath: string) {
  const imgPathProps = path.parse(imgPath)
  const image = sharp(imgPath)
  const metadata = await image.metadata()

  const formats = [metadata.format, 'webp']

  const results = await Promise.all(
    formats.map((format) =>
      image
        .toFormat(format, outputOptions[format])
        .resize({ width: 640, height: 640, fit: 'inside' })
        .toFile(
          `${path.join(imgPathProps.dir, 'optimized', imgPathProps.name)}.${
            format === 'jpeg' ? 'jpg' : format
          }`
        )
    )
  )
  console.log(results)
}

/** Transform all uploaded images into optimized versions */
// TODO Optimize all uploaded images
// Think of a strategy which would be the best
// Upload 3MB images to git ?
// Have the script be run on the build? or be good person and upload optimized images by yourself?
// What about images smaller then 640? they should not be resized right?
