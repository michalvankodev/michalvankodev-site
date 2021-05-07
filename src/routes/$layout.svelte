<script context="module" lang="typescript">
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

<script lang="typescript">
  import Nav from '../components/Nav.svelte'
  import Footer from '../components/Footer.svelte'
  export let segment
  export let latestPosts
</script>

<div class="app-content">
  <Nav {segment} />

  <main>
    <slot />
  </main>
  <Footer {latestPosts} />
</div>

<style global>
  @import '../styles/global.module.less';

  .app-content {
    display: grid;
    grid-template-rows: auto 1fr auto;
    min-height: 100vh;
  }

  main {
    position: relative;
    max-width: 100vw;
    padding: 1.2em;

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
      box-shadow: @content-box-shadow;
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
      margin: 0 0 1em 0;
      border-radius: 5px;
      box-shadow: @content-box-shadow;
    }
  }
</style>
