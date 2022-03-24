<script lang="ts">
  import ArticleFooter from '../../components/blog/ArticleFooter.svelte'
  import { postListClass, seeAllClass } from './index.css'
  import type { PostContent } from './_content'

  export let posts: PostContent[]
  export let tagQuery: string
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
