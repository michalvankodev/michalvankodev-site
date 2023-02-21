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
  import { page } from '$app/stores'

  $: segment = $page.url.pathname

  let links = [
    {
      label: 'Introduction',
      url: '/',
    },
    {
      label: 'Blog',
      url: '/blog',
    },
    {
      label: 'Broadcasts',
      url: '/broadcasts',
    },
    // {
    //   label: "Dev's Cookery",
    //   url: '/cookery',
    // },
    {
      label: 'Portfolio',
      url: '/portfolio',
    },
  ]
</script>

<nav class={navigationClass}>
  <section class={navigationContentClass}>
    <ul class={navigationLinksClass}>
      {#each links as link}
        <li>
          <a
            rel="prefetch"
            class={classNames({ [selectedClass]: segment === link.url })}
            href={link.url}
          >
            {link.label}
          </a>
        </li>
      {/each}
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
