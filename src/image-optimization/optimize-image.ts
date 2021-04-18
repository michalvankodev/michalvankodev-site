import { optimizeImage } from './image-optimization'

const args = process.argv.slice(2)

args.forEach((file) => optimizeImage(file))
