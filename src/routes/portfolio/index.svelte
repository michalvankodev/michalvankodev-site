<script lang="ts" context="module">
  /**
   * @type {import('@sveltejs/kit').Load}
   */
  export async function load({ fetch }) {
    const res = await fetch('portfolio.json')
    const content = await res.json()
    return {
      props: {
        content,
      },
    }
  }
</script>

<script lang="ts">
  import Work from '../../components/portfolio/work.svelte'
  import Project from '../../components/portfolio/project.svelte'
  import type { PortfolioContent } from './index.json'
  import { listClass, listItemClass, nameTagClass } from './index.css'

  export let content: PortfolioContent
</script>

<svelte:head>
  <title>{content.title}</title>
</svelte:head>

<h1 class="name-tag {nameTagClass}">Michal Vanko</h1>

<h2 class="name-tag {nameTagClass}">Software Architect and Consultant</h2>

<section id="personal-information">
  {@html content.body}
</section>

<section id="work-history">
  <h2>Work experience</h2>
  <section class="work-history-prelude">
    {@html content.workHistoryPrelude}
  </section>
  <ul class={listClass}>
    {#each content.workHistory as work}
      <li class={listItemClass}>
        <Work {work} />
      </li>
    {/each}
  </ul>
</section>

<section id="projects">
  <h2>Projects</h2>
  <ul class={listClass}>
    {#each content.projects as project}
      <li class={listItemClass}>
        <Project {project} />
      </li>
    {/each}
  </ul>
</section>

<section id="education">
  <h2>Education</h2>
  <ul class={listClass}>
    {#each content.education as work}
      <li class={listItemClass}>
        <Work {work} />
      </li>
    {/each}
  </ul>
</section>

<style>
  :global([id])::before {
    content: '';
    display: block;
    height: 5em;
    margin-top: -5em;
    visibility: hidden;
  }
</style>
