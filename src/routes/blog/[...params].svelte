<script lang="ts" context="module">
  import {
    getPaginationSearchParams,
    parseParams,
  } from '$lib/pagination/searchParams'

  const pageSize = 7

  /**
   * @type {import('@sveltejs/kit').Load}
   */
  export async function load({ fetch, params }) {
    console.log('params', params)
    const { page = 1, ...filters } = parseParams(params.params)
    const searchParams = getPaginationSearchParams({ pageSize, page, filters })
    console.log('searchpprsm', searchParams)
    const articleResponse = await fetch(
      `/blog/articles?${searchParams.toString()}`
    ).then((r) => r.json())

    return {
      props: {
        posts: articleResponse.posts,
        page: Number(page),
        pageSize,
        filters,
      },
    }
  }
</script>

<script lang="ts">
  import ArticleFooter from '../../components/blog/ArticleFooter.svelte'
  import Paginator from '../../components/paginator/Paginator.svelte'
  import { postListClass, seeAllClass } from './index.css'
  import type { PostContent } from './_content'
  import type { PaginationResult } from '$lib/pagination/pagination'

  export let posts: PaginationResult<PostContent>
  export let filters: Record<string, string>
  export let page: number
  export let pageSize: number
  let totalPages = Math.ceil(posts.totalCount / pageSize)
</script>

<svelte:head>
  <title>My blog @michalvankodev</title>
</svelte:head>

{@debug posts}
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
  {#each posts.items as post}
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
