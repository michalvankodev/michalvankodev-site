<script lang="ts">
  interface ArticleDetails {
    title: string
    slug: string
    preview: string
  }
  export let segment: string
  export let article: ArticleDetails
</script>

<article>
  <header>
    <h2>
      <a rel="prefetch" href={`/${segment}/${article.slug}`}>{article.title}</a>
    </h2>
  </header>
  {@html article.preview}
</article>
