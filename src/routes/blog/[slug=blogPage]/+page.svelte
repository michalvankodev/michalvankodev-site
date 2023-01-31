<script lang="ts">
  import ArticleFooter from '../../../components/blog/ArticleFooter.svelte'
  import type { PageData } from './$types'
  import { contentClass } from './blog.css'

  export let data: PageData
</script>

<svelte:head>
  <title>{data.title}</title>
</svelte:head>

<h1>{data.title}</h1>

<div class="content {contentClass}">
  {@html data.body}
</div>
<ArticleFooter post={data} />
