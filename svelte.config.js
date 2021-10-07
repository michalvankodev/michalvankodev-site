import adapterStatic from '@sveltejs/adapter-static'
import preprocess from 'svelte-preprocess'
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'

const mode = process.env.NODE_ENV
const dev = mode === 'development'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapterStatic(),
    vite: { plugins: [vanillaExtractPlugin()] },
  },
  preprocess: preprocess({
    sourceMap: dev,
  }),
}

export default config
