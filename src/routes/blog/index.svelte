<script context="module" lang="ts">
  // TODO Fix query & seeAll functionality
  /**
   * @type {import('@sveltejs/kit').Load}
   */
  export function load({ fetch, page: { params, query } }) {
    const blogQuery = query ? '?' + query.toString() : ''
    return fetch(`blog.json${blogQuery}`)
      .then((r) => r.json())
      .then((posts) => {
        return { props: { posts, query } }
      })
  }
</script>

<script lang="ts">
  import ArticleFooter from '../../components/blog/ArticleFooter.svelte'
  import { postListClass, seeAllClass } from './index.css'
  import type { PostContent } from './_content'

  export let posts: PostContent[]
  export let query
  export let tagQuery

  $: tagQuery = query.get('tag')
</script>

<svelte:head>
  <title>My blog @michalvankodev</title>
</svelte:head>

{#if posts.length === 0}
  <p class="no-posts">You've found void in the space.</p>
{:else}
  <h1>
    Recent
    {#if tagQuery}
      <em>{tagQuery}</em>
    {/if}
    posts
  </h1>
  {#if tagQuery}
    <div class={seeAllClass}>
      <a href="/blog">See all posts</a>
    </div>
  {/if}
{/if}
<ul class="post-list {postListClass}">
  {#each posts as post}
    <!-- we're using the non-standard `rel=prefetch` attribute to
				tell Sapper to load the data for the page as soon as
				the user hovers over the link or taps it, instead of
				waiting for the 'click' event -->
    <li>
      <article>
        <header>
          <h2>
            <a rel="prefetch" href="blog/{post.slug}">{post.title}</a>
          </h2>
        </header>
        {@html post.preview}
      </article>
      <ArticleFooter {post} />
    </li>
  {/each}
</ul>
