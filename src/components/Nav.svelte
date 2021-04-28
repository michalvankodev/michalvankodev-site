<script>
  import classNames from 'classnames'

  export let segment
</script>

<nav class="navigation-theme">
  <section class="nav-main">
    <ul>
      <li>
        <a class={classNames({ selected: segment === '/' })} href="/">
          Introduction
        </a>
      </li>
      <!-- for the blog link, we're using rel=prefetch so that Sapper prefetches
		     the blog data when we hover over the link or tap it on a touchscreen -->
      <li>
        <a
          rel="prefetch"
          class={classNames({ selected: segment.startsWith('/blog') })}
          href="/blog"
        >
          Blog
        </a>
      </li>
      <li>
        <a
          class={classNames({ selected: segment.startsWith('/portfolio') })}
          href="/portfolio"
        >
          Portfolio
        </a>
      </li>
    </ul>

    <aside class="logo-section">
      <a class="logo" href=".">
        <img src="/images/m-logo.png" alt="m logo" />
      </a>
    </aside>
  </section>

  {#if segment === 'portfolio'}
    <!-- Move to portfolio layout -->
    <section class="page-navigation">
      <a href="portfolio#personal-information">About</a>
      <a href="portfolio#skills">Skills</a>
      <a href="portfolio#work-history">Work History</a>
      <a href="portfolio#projects">Projects</a>
      <a href="portfolio#education">Education</a>
    </section>
  {/if}
</nav>

<style>
  @import '../styles/variables.module.less';

  nav {
    position: sticky;
    top: -2.75em;
    z-index: 1;
    padding: 0.5em 0.5em 2em;
    background: linear-gradient(
      177.43deg,
      @pinky -6.98%,
      rgba(@pinky, 0) 54.88%
    );
    color: @menu-color;
  }

  a {
    color: @menu-link-color;

    &:hover {
      color: @menu-link-hover-color;
    }
  }

  nav a {
    padding: 0.4em 0.5em;
  }

  .nav-main {
    display: flex;
    max-width: 900px;
    margin: 0 auto;
  }

  .logo-section,
  .logo-section img {
    height: 1em;
  }

  .logo-section img {
    filter: invert(1);
  }

  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
  }

  .nav-main > ul {
    flex: 1;
  }

  a.selected {
    text-shadow: 0px 0px 1px;
  }

  .page-navigation {
    font-size: 0.86em;
    border-top: 1px solid #65624f;
    padding-top: 0.4em;
    max-width: 900px;
    margin: 0.3em auto 0;
  }
</style>
