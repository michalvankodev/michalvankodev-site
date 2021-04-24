<script context="module" lang="typescript">
  import type { LoadInput, LoadOutput } from '@sveltejs/kit/types/page'

  export async function load({
    fetch,
    page: { params },
  }: LoadInput): Promise<LoadOutput> {
    try {
      const res = await fetch(`${params.slug}.json`)
      const data = await res.json()

      if (res.ok) {
        return { props: { post: data } }
      }
      return {
        status: res.status,
        error: new Error(`Could not load ${params.slug} post`),
      }
    } catch (e) {
      return {
        status: 500,
        error: e,
      }
    }
  }
</script>

<script lang="typescript">
  import ArticleFooter from '../../components/blog/article-footer.svelte'

  export let post
</script>

<svelte:head>
  <title>{post.title}</title>
  <link rel="stylesheet" href="/prism.css" />
</svelte:head>

<h1>{post.title}</h1>

<div class="content">
  {@html post.body}
</div>
<ArticleFooter {post} />

<style>
  /*
		By default, CSS is locally scoped to the component,
		and any unused styles are dead-code-eliminated.
		In this page, Svelte can't know which elements are
		going to appear inside the {{{post.html}}} block,
		so we have to use the :global(...) modifier to target
		all elements inside .content
	*/
  .content :global(pre) {
    background-color: #f9f9f9;
    box-shadow: inset 1px 1px 5px rgba(0, 0, 0, 0.05);
    padding: 0.5em;
    border-radius: 4px;
    overflow-x: auto;
  }

  .content :global(pre) :global(code) {
    background-color: transparent;
    /* padding: 0; */
  }

  .content :global(ul) {
    line-height: 1.5;
  }

  .content :global(li) {
    margin: 0 0 0.5em 0;
  }

  .content :global(img) {
    max-width: 100%;
    max-height: 640px;
    border-radius: 5px;
    box-shadow: 0px 0px 8px 1px #2d3935;
  }

  .content :global(img:only-child) {
    display: block;
    margin: 0 auto;
  }
</style>
