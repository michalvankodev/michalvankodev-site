<script context="module" lang="typescript">
  export function preload({ params, query }) {
    const blogQuery = query
      ? '?' +
        Object.entries(query)
          .map(q => q.join('='))
          .join('&')
      : ''
    return this.fetch(`blog.json${blogQuery}`)
      .then(r => r.json())
      .then(posts => {
        return { posts, query }
      })
  }
</script>

<script lang="typescript"> 
  import ArticleFooter from '../../components/blog/article-footer.svelte'
  import type { PostContent } from './_content';

  export let posts: PostContent[]
  export let query 
</script>

<style>
  .post-list {
    margin: 0;
    padding: 0;
    line-height: 1.5;
    list-style: none;
  }

  .post-list > li:not(:last-child) {
    margin-bottom: 2em;
  }

  .see-all {
    text-align: end;
    margin-top: -1.5em;
  }
</style>

<svelte:head>
  <title>My blog @michalvankodev</title>
</svelte:head>

{#if posts.length === 0}
  <p class="no-posts">You've found void in the space.</p>
{:else}
  <h1>
    Recent
    {#if query.tag}
      <em>{query.tag}</em>
    {/if}
    posts
  </h1>
  {#if query.tag}
    <div class="see-all">
      <a href="/blog" class="">See all posts</a>
    </div>
  {/if}
{/if}
<ul class="post-list">
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
      <ArticleFooter post={post} />
    </li>
  {/each}
</ul>
