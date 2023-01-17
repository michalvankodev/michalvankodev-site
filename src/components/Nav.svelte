<script>
  import classNames from 'classnames'
  import {
    logoImgClass,
    logoLinkClass,
    logoSectionClass,
    navigationClass,
    navigationContentClass,
    navigationLinksClass,
    portfolioPageNavigation,
    portfolioPageNavigationLinkClass,
    portfolioPageNavigationLinksClass,
    selectedClass,
  } from './Nav.css'
  import { page } from "$app/stores"

  $: segment = $page.url.pathname
</script>

<nav class={navigationClass}>
  <section class={navigationContentClass}>
    <ul class={navigationLinksClass}>
      <li>
        <a class={classNames({ [selectedClass]: segment === '/' })} href="/">
          Introduction
        </a>
      </li>
      <!-- for the blog link, we're using rel=prefetch so that Sapper prefetches
		     the blog data when we hover over the link or tap it on a touchscreen -->
      <li>
        <a
          rel="prefetch"
          class={classNames({ [selectedClass]: segment.startsWith('/blog') })}
          href="/blog"
        >
          Blog
        </a>
      </li>
      <li>
        <a
          class={classNames({
            [selectedClass]: segment.startsWith('/portfolio'),
          })}
          href="/portfolio"
        >
          Portfolio
        </a>
      </li>
    </ul>

    <aside class="logo-section {logoSectionClass}">
      <a class="logo {logoLinkClass}" href=".">
        <img
          class={logoImgClass}
          src="/m-logo.svg"
          alt="m logo"
          width="44px"
          height="44px"
        />
      </a>
    </aside>
  </section>
</nav>

{#if segment === '/portfolio'}
  <section class="page-navigation {portfolioPageNavigation}">
    <div class={portfolioPageNavigationLinksClass}>
      <a
        class={portfolioPageNavigationLinkClass}
        href="/portfolio#personal-information">About</a
      >
      <a class={portfolioPageNavigationLinkClass} href="/portfolio#skills"
        >Skills</a
      >
      <a class={portfolioPageNavigationLinkClass} href="/portfolio#work-history"
        >Work History</a
      >
      <a class={portfolioPageNavigationLinkClass} href="/portfolio#projects"
        >Projects</a
      >
      <a class={portfolioPageNavigationLinkClass} href="/portfolio#education"
        >Education</a
      >
    </div>
  </section>
{/if}
