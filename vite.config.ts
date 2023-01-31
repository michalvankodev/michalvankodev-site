import { sveltekit } from '@sveltejs/kit/vite'
import type { UserConfig } from 'vite'
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'

const config: UserConfig = {
  plugins: [vanillaExtractPlugin(), sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
  },
  server: { fs: { allow: ['static/build'] } },

  // kit: {
  //   adapter: adapterStatic(),
  //   vite: {
  //     plugins: [vanillaExtractPlugin()],
  //     server: { fs: { allow: ['static/build'] } },
  //   },
  //   prerender: { default: true },
  // },
  // preprocess: preprocess({
  //   sourceMap: dev,
  // }),
}

export default config
