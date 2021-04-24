const adapterStatic = require('@sveltejs/adapter-static')
const sveltePreprocess = require('svelte-preprocess')

const mode = process.env.NODE_ENV
const dev = mode === 'development'

/** @type {import('@sveltejs/kit').Config} */
module.exports = {
  kit: {
    adapter: adapterStatic(),
  },
  preprocess: sveltePreprocess({
    sourceMap: dev,
    defaults: {
      script: 'typescript',
    },
  }),
}
