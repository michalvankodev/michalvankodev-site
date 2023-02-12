<script lang="ts">
  import ArticleFooter from '$lib/components/articles/ArticlePreviewFooter/ArticlePreviewFooter.svelte'
  import type { PageData } from './$types'
  import { contentClass } from './page.css'

  export let data: PageData
</script>

<svelte:head>
  <title>{data.title}</title>
</svelte:head>

<h1>{data.title}</h1>

<div class="content {contentClass}">
  {@html data.body}
</div>
<ArticleFooter article={data} segment="blog" />
