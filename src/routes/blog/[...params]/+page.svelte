<script lang="ts">
  import ArticleFooter from '../../../components/blog/ArticleFooter.svelte'
  import Paginator from '../../../components/paginator/Paginator.svelte'
  import { postListClass, seeAllClass } from './index.css'
  import type { PageData } from './$types'

  export let data: PageData
  let { posts, filters, page, pageSize } = data
  $: ({ posts, filters, page, pageSize } = data)
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
<header>
  <Paginator
    href="blog"
    {page}
    {pageSize}
    {filters}
    totalCount={posts.totalCount}
  />
</header>
<ul class="post-list {postListClass}">
  {#each posts.items as post (post.slug)}
    <li>
      <article>
        <header>
          <h2>
            <a rel="prefetch" href="/blog/{post.slug}">{post.title}</a>
          </h2>
        </header>
        {@html post.preview}
      </article>
      <ArticleFooter {post} />
    </li>
  {/each}
</ul>
<footer>
  <Paginator
    href="blog"
    {page}
    {pageSize}
    {filters}
    totalCount={posts.totalCount}
  />
</footer>
