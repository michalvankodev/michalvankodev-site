<script context="module" lang="ts">
  /**
   * @type {import('@sveltejs/kit').Load}
   */
  export function load({ fetch }) {
    return fetch(`blog.json`)
      .then((r) => r.json())
      .then((posts) => {
        return { props: { posts } }
      })
  }
</script>

<script lang="ts">
  import ArticleFooter from '../../components/blog/ArticleFooter.svelte'
  import { postListClass, seeAllClass } from './index.css'
  import type { PostContent } from './_content'

  export let posts: PostContent[]
  export let displayedPosts: PostContent[]
  export let tagQuery: string

  $: {
    if (typeof window !== 'undefined') {
      let params = new URLSearchParams(window.location.search)
      tagQuery = params.get('tag')
      displayedPosts = posts.filter((post) => post.tags.includes(tagQuery))
    } else {
      displayedPosts = posts
    }
  }
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
