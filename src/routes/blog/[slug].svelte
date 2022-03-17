<script context="module" lang="ts">
  import type { LoadInput, LoadOutput } from '@sveltejs/kit/types/page'
  import { onMount } from 'svelte';

  export async function load({
    fetch,
    page: { params },
  }: LoadInput): Promise<LoadOutput> {
    try {
      const res = await fetch(`${params.slug}.json`)
      const data = await res.json()

      if (res.ok) {
        return { props: { post: data } }
      }
      return {
        status: res.status,
        error: new Error(`Could not load ${params.slug} post`),
      }
    } catch (e) {
      return {
        status: 500,
        error: e,
      }
    }
  }
</script>

<script lang="ts">
  import ArticleFooter from '../../components/blog/ArticleFooter.svelte'
  import { contentClass } from './blog.css'
  import type { PostContent } from './_content';

  export let post: PostContent
  export let loadMermaid = false

  
  onMount(() => {
    // We have to load mermaid after the mount so svelte doesn't rerender the parsed body on mount
    // This way we create the svg diagram after whole page is rendered
    loadMermaid = post.body.hasDiagrams
  })
</script>

<svelte:head>
  <title>{post.title}</title>
  {#if loadMermaid}
    <script id="mermaid-script" src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script>
      document.getElementById('mermaid-script').addEventListener('load', () => {
        mermaid.init()
      })
    </script>
  {/if}
</svelte:head>

<h1>{post.title}</h1>

<div class="content {contentClass}">
  {@html post.body.parsed}
</div>
<ArticleFooter {post} />
