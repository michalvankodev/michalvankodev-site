<script context="module">
  export function preload({ params, query }) {
    return this.fetch(`blog.json`)
      .then(r => r.json())
      .then(posts => {
        return { posts }
      })
  }
</script>

<script>
  import { format } from 'date-fns'

  export let posts
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

  .tags-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: inline;
  }

  .tags-list li {
    display: inline;
    font-style: italic;
  }

  time {
    font-style: italic;
  }

  footer {
    display: flex;
    font-size: 0.85em;
    justify-content: space-between;
    padding-top: 0.2em;
    margin-top: 0.4em;
    border-top: 1px solid #c0c1e1;
  }

  .lighten {
    color: #595a8f;
  }
</style>

<svelte:head>
  <title>My blog @michalvankodev</title>
</svelte:head>

<h1>Recent posts</h1>

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
        <footer>
          <div class="article-tags">
            {#if post.tags.length > 0}
              <span class="lighten">Tags:</span>
              <ul class="tags-list">
                {#each post.tags as tag}
                  <li>
                    <a href="blog?tag={tag}">{tag}</a>
                  </li>
                {/each}
              </ul>
            {/if}
          </div>
          <div class="created-at">
            <span class="lighten">Published on</span>
            <time datetime={post.date}>
              {format(new Date(post.date), "do MMMM',' y")}
            </time>
          </div>
        </footer>
      </article>

    </li>
  {/each}
</ul>
