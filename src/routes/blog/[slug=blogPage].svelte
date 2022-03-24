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
