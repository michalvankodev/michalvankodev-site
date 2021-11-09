<script lang="ts">
  import { format } from 'date-fns'
  import type { PostContent } from 'src/routes/blog/_content'
  import SvgIcon from './SvgIcon.svelte'
  import { siteFooterStyle } from './Footer.css'

  export let latestPosts: PostContent[]
</script>

<footer class="site-footer navigation-theme {siteFooterStyle}">
  <div class="lists">
    <section class="site-map">
      <ul>
        <li>
          <a href="/">Introduction</a>
        </li>
        <li>
          <a href="/portfolio">Portfolio</a>
          <ul>
            <li>
              <a href="/portfolio#personal-information">About</a>
            </li>
            <li>
              <a href="/portfolio#skills">Skills</a>
            </li>
            <li>
              <a href="/portfolio#work-history">Work History</a>
            </li>
            <li>
              <a href="/portfolio#projects">Projects</a>
            </li>
            <li>
              <a href="/portfolio#education">Education</a>
            </li>
          </ul>
        </li>
      </ul>
    </section>
    <section class="latest-posts">
      <h3>
        <a href="/blog">Latest posts</a>
      </h3>
      <ul>
        {#each latestPosts as post}
          <li>
            <a rel="prefetch" href="/blog/{post.slug}">
              <span>{post.title}</span>
              <span class="date">
                - {format(new Date(post.date), 'do MMM, yyyy')}
              </span>
            </a>
          </li>
        {/each}
      </ul>
      <hr />
      <section class="subscribe">
        <a href="/feed.xml" title="RSS feed" class="rss">
          Subscribe
          <SvgIcon name="rss" className="svg-icon" />
        </a>
        <a
          href="/feed.json"
          title="JSON feed"
          class="json-feed"
          aria-label="Subscribe with JSON feed"
        >
          <SvgIcon name="json-feed" className="svg-icon" />
        </a>
      </section>
    </section>
    <section class="socials">
      <h3>Contact</h3>
      <ul class="social-links">
        <li class="email">
          <a href="mailto: michalvankosk@gmail.com" title="E-mail address">
            <SvgIcon name="mail" className="svg-icon" />
            <span>michalvankosk@gmail.com</span>
          </a>
        </li>
        <li class="twitter">
          <a href="https://twitter.com/michalvankodev" title="Twitter profile">
            <SvgIcon name="twitter" className="svg-icon" />
            <span>Twitter</span>
          </a>
        </li>
        <li class="github">
          <a href="https://github.com/michalvankodev" title="Github profile">
            <SvgIcon name="github" className="svg-icon" />
            <span>Github</span>
          </a>
        </li>
        <li class="twitch">
          <a href="https://twitch.tv/michalvankodev" title="Twitch profile">
            <SvgIcon name="twitch" className="svg-icon" />
            <span>Twitch</span>
          </a>
        </li>
        <li class="instagram">
          <a
            href="https://www.instagram.com/michalvankodev/"
            title="Instagram profile"
          >
            <SvgIcon name="instagram" className="svg-icon" />
            <span>Instagram</span>
          </a>
        </li>
      </ul>
    </section>
  </div>
  <footer class="bottom-line">
    <span class="no-wrap">Created by @michalvankodev</span>
    <span class="no-wrap">&copy; 2020</span>
  </footer>
</footer>

<style lang="less">
  @import '../styles/variables.module.less';

  /* .site-footer {
    font-size: 0.9em;
    padding: 2em 0.8em 0em;
    color: @menu-link-color;

    background: radial-gradient(
      160% 100% at 100% 100%,
      @menu-bg-color 56%,
      fade(@menu-bg-color, 0) 100%
    );

    @media (min-width: @media-m) {
      background: radial-gradient(
        140% 100% at 100% 100%,
        @menu-bg-color 48%,
        fade(@menu-bg-color, 0) 100%
      );
    }

    @media (min-width: @media-l) {
      font-size: 0.8em;
    }
  } */

  h3 {
    font: inherit;
    font-weight: bold;
    font-size: 1.15em;
    text-shadow: inherit;
    color: @menu-link-color;
    margin: 0;
  }

  .lists {
    display: grid;
    justify-items: center;
    text-align: center;
    max-width: @media-max;
    column-gap: 1em;
    margin: auto;

    @media only screen and (min-width: @media-l) {
      grid-template-columns: auto auto auto;
      justify-items: start;
      text-align: start;
    }
  }

  .lists > section {
    margin: 1em 0;
  }

  .no-wrap {
    white-space: nowrap;
  }

  .lists ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .lists li {
    margin-left: 0.5em;
  }

  .lists li li {
    font-size: 0.9em;

    @media only screen and (min-width: @media-l) {
      font-size: 0.8em;
    }
  }

  .social-links a span {
    /* display: none; */
    padding: 0 0.4em;
  }

  .email :global(svg) {
    fill: @menu-link-color;
  }

  .twitter :global(svg) {
    stroke: @menu-link-color;
    stroke-width: 2px;
    fill: @menu-link-color;
  }

  .github :global(svg) {
    stroke: @menu-link-color;
    stroke-width: 2px;
  }

  .twitch :global(svg),
  .twitch :global(svg rect) {
    fill: @menu-link-color;
  }

  .instagram :global(svg) {
    fill: @menu-link-color;
  }

  .rss :global(svg) {
    fill: @menu-link-color;
  }

  .json-feed :global(svg) {
    fill: @menu-link-color;
  }

  :global(.svg-icon) {
    height: 1em;
    width: 1em;
  }

  .social-links a {
    display: flex;
    align-items: center;
    justify-content: center;

    @media only screen and (min-width: @media-l) {
      justify-content: start;
    }
  }

  .bottom-line {
    display: flex;
    justify-content: space-between;
    margin: 1em auto 0.4em;
    max-width: @media-max;
  }

  .date {
    font-size: 0.7em;
    white-space: nowrap;
  }

  .subscribe {
    font-weight: bold;
  }

  hr {
    color: fade(@menu-link-color, 14%);
    margin: 0.75em 0.2em;
    border-width: 1px 0 0;
  }

  a {
    color: @menu-link-color;

    &:hover {
      color: @menu-link-hover-color;
    }
  }

  .latest-posts li a:visited:not(:hover) {
    color: @link-visited-color;
  }
</style>
