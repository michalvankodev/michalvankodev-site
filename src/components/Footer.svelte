<script>
  import { format } from 'date-fns'
  import twitterLogo from '../svg/iconfinder_Twitter_4070150.svg'
  import githubLogo from '../svg/iconfinder_github_2561442.svg'
  import twitchLogo from '../svg/iconfinder_twitch_306173.svg'
  import instagramLogo from '../svg/iconfinder_38-instagram_1161953.svg'
  import emailIcon from '../svg/iconfinder_mail_5474819.svg'
  import rssIcon from '../svg/iconfinder_icon-social-rss_211914.svg'
  import jsonFeedIcon from '../svg/json_feed_icon.svg'

  export let latestPosts
</script>

<style>
  h3 {
    font: inherit;
    font-weight: bold;
    font-size: 1.15em;
    text-shadow: inherit;
    color: #eae9be;
    margin: 0;
  }

  .site-footer {
    font-size: 0.9em;
    padding: 0.4em;
    box-shadow: 1px 3px #959595;
  }

  .lists {
    display: flex;
    justify-content: space-between;
    flex-direction: column;
    text-align: center;
  }

  .lists > section {
    margin-bottom: 2em;
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
  }

  .social-links a span {
    /* display: none; */
    padding: 0 0.4em;
  }

  .email :global(svg path) {
    fill: #eae9be;
  }

  .twitter :global(svg path) {
    /* fill: rgb(29, 161, 242); */
    stroke: #eae9be;
    stroke-width: 2px;
    fill: #eae9be;
  }

  .github :global(svg path) {
    stroke: #eae9be;
    stroke-width: 2px;
  }

  .twitch :global(svg path),
  .twitch :global(svg rect) {
    /* fill: rgb(169, 112, 255); */
    fill: #eae9be;
  }

  .instagram :global(svg) {
    fill: #eae9be;
  }

  .rss :global(svg) {
    fill: #eae9be;
  }

  .json-feed :global(svg) {
    fill: #eae9be;
  }

  :global(svg) {
    height: 1em;
    width: 1em;
  }

  .social-links a {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .bottom-line {
    display: flex;
    justify-content: space-between;
    margin-top: 1em;
  }

  .date {
    font-size: 0.7em;
  }

  .latest-posts li a:visited:not(:hover) {
    color: #a7a574;
  }

  .subscribe {
    font-weight: bold;
  }

  hr {
    color: #86856f;
    margin: 0.2em 0;
  }

  @media only screen and (min-width: 900px) {
    .site-footer {
      font-size: 0.8em;
    }

    .lists li li {
      font-size: 0.8em;
    }

    .lists {
      flex-direction: row;
      text-align: start;
      max-width: 900px;
      margin: auto;
    }

    .social-links a {
      justify-content: start;
    }

    .bottom-line {
      max-width: 900px;
      margin: auto;
    }
  }
</style>

<footer class="site-footer navigation-theme">
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
          Subscribe {@html rssIcon}
        </a>
        <a href="/feed.json" title="JSON feed" class="json-feed">{@html jsonFeedIcon}</a>
      </section>
    </section>
    <section class="socials">
      <h3>Contact</h3>
      <ul class="social-links">
        <li class="email">
          <a href="mailto: michalvankosk@gmail.com" title="E-mail address">
            {@html emailIcon}
            <span>michalvankosk@gmail.com</span>
          </a>
        </li>
        <li class="twitter">
          <a href="https://twitter.com/michalvankodev" title="Twitter profile">
            {@html twitterLogo}
            <span>Twitter</span>
          </a>
        </li>
        <li class="github">
          <a href="https://github.com/michalvankodev" title="Github profile">
            {@html githubLogo}
            <span>Github</span>
          </a>
        </li>
        <li class="twitch">
          <a href="https://twitch.tv/michalvankodev" title="Twitch profile">
            {@html twitchLogo}
            <span>Twitch</span>
          </a>
        </li>
        <li class="instagram">
          <a
            href="https://www.instagram.com/michalvankodev/"
            title="Instagram profile">
            {@html instagramLogo}
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
