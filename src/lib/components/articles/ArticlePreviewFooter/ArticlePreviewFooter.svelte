<script lang="ts">
  import { horizontalBorderTopClass } from '$lib/styles/scoops.css'

  import { format } from 'date-fns'
  import type { ArticleContent } from '$lib/content/articleContentListing'
  import {
    footerClass,
    publishedClass,
    publishedLabelClass,
    tagsListClass,
    tagsListLiClass,
  } from './ArticlePreviewFooter.css'

  export let segment: string
  export let article: ArticleContent
</script>

<footer class="{footerClass} {horizontalBorderTopClass}">
  <div class="article-tags">
    {#if article.tags.length > 0}
      <span class="lighten">Tags:</span>
      <ul class={tagsListClass}>
        {#each article.tags as tag}
          <li class={tagsListLiClass}>
            <a href="/{segment}/tags/{tag}">{tag}</a>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
  <div class="created-at">
    <span class={publishedLabelClass}>Published on</span>
    <time datetime={article.date} class={publishedClass}>
      {format(new Date(article.date), "do MMMM',' y")}
    </time>
  </div>
</footer>
