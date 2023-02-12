<script lang="ts">
  import type { PageData } from './$types'
  import ArticlePreviewList from '$lib/components/articles/ArticlePreviewList/ArticlePreviewList.svelte'
  import { seeAllClass } from '$lib/components/articles/ArticlePreviewList/ArticlePreviewList.css'

  export let data: PageData
  $: ({ posts, filters } = data)
</script>

<svelte:head>
  <title>Broadcasts @michalvankodev</title>
</svelte:head>

{#if posts.items.length === 0}
  <p class="no-posts">You've found void in the space.</p>
{:else}
  <h1>
    {#if filters.tags}
      <em>{filters.tags}</em>
    {/if}
    Broadcasts
  </h1>
  {#if filters.tags}
    <div class={seeAllClass}>
      <a href="/broadcasts">See all broadcasts</a>
    </div>
  {/if}
{/if}

<ArticlePreviewList {...data} segment="broadcasts" />
