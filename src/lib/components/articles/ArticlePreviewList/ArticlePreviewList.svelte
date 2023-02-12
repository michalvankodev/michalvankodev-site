<script lang="ts">
  import ArticleFooter from '$lib/components/articles/ArticlePreviewFooter/ArticlePreviewFooter.svelte'
  import Paginator from '$lib/components/paginator/Paginator.svelte'
  import { postListClass } from './ArticlePreviewList.css'
  import ArticlePreviewCard from '$lib/components/articles/ArticlePreviewCard/ArticlePreviewCard.svelte'
  import type { PaginationResult } from '$lib/pagination/pagination'
  import type { ArticleContent } from '$lib/content/articleContentListing'

  export let page: number
  export let pageSize: number
  export let filters: Record<string, string>
  export let posts: PaginationResult<ArticleContent>
  export let segment: string
</script>

<header>
  <Paginator
    {segment}
    {page}
    {pageSize}
    {filters}
    totalCount={posts.totalCount}
  />
</header>
<ul class="post-list {postListClass}">
  {#each posts.items as article (article.slug)}
    <li>
      <ArticlePreviewCard {article} {segment} />
      <ArticleFooter {article} {segment} />
    </li>
  {/each}
</ul>
<footer>
  <Paginator
    {segment}
    {page}
    {pageSize}
    {filters}
    totalCount={posts.totalCount}
  />
</footer>
