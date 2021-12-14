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
  import 'modern-normalize/modern-normalize.css'
  import '../styles/global.css'
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
