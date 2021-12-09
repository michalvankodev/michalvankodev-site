<script context="module" lang="ts">
  import type { LoadInput, LoadOutput } from '@sveltejs/kit/types/page'

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

  export let post
</script>

<svelte:head>
  <title>{post.title}</title>
</svelte:head>

<h1>{post.title}</h1>

<div class="content {contentClass}">
  {@html post.body}
</div>
<ArticleFooter {post} />
