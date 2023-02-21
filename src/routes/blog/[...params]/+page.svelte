<script lang="ts">
  import type { PageData } from './$types'
  import ArticlePreviewList from '$lib/components/articles/ArticlePreviewList/ArticlePreviewList.svelte'
  import { seeAllClass } from '$lib/components/articles/ArticlePreviewList/ArticlePreviewList.css'

  export let data: PageData
  $: ({ posts, filters } = data)
</script>

<svelte:head>
  <title>My blog @michalvankodev</title>
</svelte:head>

{#if posts.items.length === 0}
  <p class="no-posts">You've found void in the space.</p>
{:else}
  <h1>
    {#if filters.tags}
      <em>{filters.tags}</em>
    {:else}
      Blog
    {/if}
    posts
  </h1>
  {#if filters.tags}
    <div class={seeAllClass}>
      <a href="/blog">See all posts</a>
    </div>
  {/if}
{/if}

<ArticlePreviewList {...data} segment="blog" />
