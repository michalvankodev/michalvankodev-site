<script>
  import twitterLogo from '../svg/iconfinder_Twitter_4070150.svg'
  import githubLogo from '../svg/iconfinder_github_2561442.svg'
  import twitchLogo from '../svg/iconfinder_twitch_306173.svg'
  import instagramLogo from '../svg/iconfinder_38-instagram_1161953.svg'
  import emailIcon from '../svg/iconfinder_mail_5474819.svg'
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
    background: #2d3935;
    color: #eae9be;
    font-size: 0.8em;
    padding: 0.4em;
  }

  .no-wrap {
    white-space: nowrap;
  }

  a {
    /* color: #f2ee12; */
    color: #eae9be;
  }

  .socials {
    /* display: flex;
    align-items: center; */
  }

  .social-links {
    list-style: none;
    padding: 0;
    margin: 0;
    /* display: inline-flex; */
    /* align-items: center; */
  }

  .social-links a span {
    /* display: none; */
    padding: 0 0.4em;
  }

  .social-links li {
    /* display: inline-block; */
    margin-left: 0.5em;
  }

  .email :global(svg path) {
    fill: #eae9be;
  }

  .twitter :global(svg path) {
    /* fill: rgb(29, 161, 242); */
    stroke: #fff;
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

  :global(svg) {
    height: 1em;
    width: 1em;
  }

  .social-links a {
    display: flex;
    align-items: center;
  }

  .bottom-line {
    display: flex;
    justify-content: space-between;
    margin-top: 1em;
  }

  @media only screen and (min-width: 700px) {
    .social-links a span {
      display: inline;
    }
  }
</style>

<footer class="site-footer">
  <div class="no-wrap socials">
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
  </div>
  <footer class="bottom-line">
    <span class="no-wrap">Created by @michalvankodev</span>
    <span class="no-wrap">&copy; 2020</span>
  </footer>
</footer>
