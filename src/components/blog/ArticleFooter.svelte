<script lang="ts">
  import { format } from 'date-fns'
  import type { PostContent } from '../../routes/blog/_content'
  import {
    footerClass,
    publishedClass,
    publishedLabelClass,
    tagsListClass,
    tagsListLiClass,
  } from './ArticleFooter.css'

  export let post: PostContent
</script>

<footer class={footerClass}>
  <div class="article-tags">
    {#if post.tags.length > 0}
      <span class="lighten">Tags:</span>
      <ul class={tagsListClass}>
        {#each post.tags as tag}
          <li class={tagsListLiClass}>
            <a href="/blog?tag={tag}">{tag}</a>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
  <div class="created-at">
    <span class={publishedLabelClass}>Published on</span>
    <time datetime={post.date} class={publishedClass}>
      {format(new Date(post.date), "do MMMM',' y")}
    </time>
  </div>
</footer>
