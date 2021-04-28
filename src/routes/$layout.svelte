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
    max-width: 34em;
    padding: 2em;
    margin: 0 auto;
    box-sizing: border-box;
  }
</style>
