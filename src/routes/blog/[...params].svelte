<script lang="ts" context="module">
  import { parseParams } from '$lib/pagination/dropTakeParams'

  /**
   * @type {import('@sveltejs/kit').Load}
   */
  export async function load({ fetch, params }) {
    const { page = 1, pageSize = 7, ...filters } = parseParams(params.params)
    const articleResponse = await fetch(
      `/blog/articles/${params.params ? params.params : 'index'}.json`
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
  import type { PostContent } from './content'
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
