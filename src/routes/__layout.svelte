<script context="module" lang="ts">
  import { take } from 'ramda'
  import type { LoadInput, LoadOutput } from '@sveltejs/kit/types/page'

  export function load({ fetch, page }: LoadInput): Promise<LoadOutput> {
    return fetch(`/blog.json`)
      .then((r) => r.json())
      .then((posts) => {
        return {
          props: {
            latestPosts: take(5, posts),
            segment: page.path,
          },
        }
      })
  }
</script>

<script lang="ts">
  import Nav from '../components/Nav.svelte'
  import Footer from '../components/Footer.svelte'
  import { mainContentClass } from './layout.css'

  export let segment
  export let latestPosts
</script>

<div class="app-content">
  <Nav {segment} />

  <main class={mainContentClass}>
    <slot />
  </main>
  <Footer {latestPosts} />
</div>

<style global lang="less">
  @import '../styles/global.module.less';

  main {
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    p,
    ul,
    ol,
    figure,
    img,
    blockquote,
    iframe,
    footer {
      max-width: 42rem;
      margin-left: auto;
      margin-right: auto;
    }

    h1 {
      max-width: 52rem;
    }
    h2 {
      max-width: 46rem;
    }
    pre,
    pre[class*='language-'] {
      max-width: 48rem;
      margin-left: auto;
      margin-right: auto;
    }
    figure {
      max-width: @max-image-size;
    }
    iframe {
      max-width: 46rem;
      display: block;
    }
    footer {
      max-width: 52rem;
    }

    img {
      width: 100%;
      border-radius: 5px;
      box-shadow: @content-box-shadow;
    }
  }
</style>
