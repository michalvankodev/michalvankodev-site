<script lang="ts">
  import { format } from 'date-fns'
  import type { PostContent } from '../../routes/blog/_content'

  export let post: PostContent
</script>

<footer>
  <div class="article-tags">
    {#if post.tags.length > 0}
      <span class="lighten">Tags:</span>
      <ul class="tags-list">
        {#each post.tags as tag}
          <li>
            <a href="/blog?tag={tag}">{tag}</a>
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

<style lang="less">
  @import '../../styles/variables.module.less';

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

  .lighten {
    color: tint(@article-text-color, 25%);
  }

  time {
    white-space: nowrap;
    font-style: italic;
  }

  footer {
    display: flex;
    font-size: 0.8em;
    justify-content: space-between;
    padding-top: 0.6em;
    margin-top: 1em;
    border-top: 1px solid fade(desaturate(@tearkiss, 50%), 40%);
  }
</style>
