<script>
  import classNames from 'classnames'

  export let segment
</script>

<nav>
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
        <img src="/m-logo.svg" alt="m logo" width="44px" height="44px" />
      </a>
    </aside>
  </section>
</nav>

{#if segment === '/portfolio'}
  <section class="page-navigation">
    <div class="page-navigation-links">
      <a href="/portfolio#personal-information">About</a>
      <a href="/portfolio#skills">Skills</a>
      <a href="/portfolio#work-history">Work History</a>
      <a href="/portfolio#projects">Projects</a>
      <a href="/portfolio#education">Education</a>
    </div>
  </section>
{/if}

<style>
  @import '../styles/variables.module.less';

  a {
    color: @menu-link-color;
    padding: 0.4em 0.5em;

    &:hover {
      color: @menu-link-hover-color;
    }
  }

  nav {
    padding: 0.5em 0.5em 1em;

    background: radial-gradient(
      120% 100% at 0% 0%,
      @menu-bg-color,
      rgba(@pinky, 0) 100%
    );

    color: @menu-color;
    text-shadow: @menu-link-text-shadow;
  }

  .nav-main {
    display: flex;
    max-width: @media-max;
    margin: 0 auto;
  }

  .logo-section {
    line-height: 0px;
  }

  .logo {
    padding: 0;
    display: block;
    max-height: 1em;
  }

  .logo img {
    height: 1.8em;
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
    text-shadow: @menu-active-link-text-shadow;
  }

  .page-navigation {
    position: sticky;
    top: 0px;
    z-index: 1;
    width: 100%;
    font-size: 0.86em;
    padding: 0.3em;
    background: @bg-color;
    box-shadow: 0px 0.5em 0.5em @bg-color;
  }

  .page-navigation-links {
    max-width: @media-l;
    margin: 0 auto;
  }
</style>
