<script lang="ts">
  import ArticleFooter from '$lib/components/articles/ArticlePreviewFooter/ArticlePreviewFooter.svelte'
  import type { PageData } from './$types'
  import { contentClass } from '$lib/styles/article/article.css'
  import { onMount } from 'svelte'
  import { runOnMountScripts } from '$lib/articleContent/onMountScripts'

  export let data: PageData

  onMount(() => {
    runOnMountScripts()
  })
</script>

<svelte:head>
  <title>{data.title}</title>
</svelte:head>

<h1>{data.title}</h1>

<article class="content {contentClass}">
  {@html data.body}
</article>
<ArticleFooter article={data} segment="broadcasts" />
