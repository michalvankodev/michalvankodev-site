<script context="module" lang="ts">
  import { take } from 'ramda'
  import type { LoadInput, LoadOutput } from '@sveltejs/kit/types/page'

  export async function load({ fetch, url }: LoadInput): Promise<LoadOutput> {
    const blogPostsResponse = await fetch(`/blog/articles?limit=5`)
    const blogPostsContent = await blogPostsResponse.json()
    return {
      props: {
        latestPosts: blogPostsContent.posts.items,
        // TODO Check if not bugged FIXME
        segment: '',
      },
    }
  }
</script>

<script lang="ts">
  import Nav from '../components/Nav.svelte'
  import Footer from '../components/Footer.svelte'
  import 'modern-normalize/modern-normalize.css'
  import '$lib/styles/global.css'
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
