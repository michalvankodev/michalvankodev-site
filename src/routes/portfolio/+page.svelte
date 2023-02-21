<script lang="ts">
  import Work from './components/work.svelte'
  import Project from './components/project.svelte'
  import Presentation from './components/presentation.svelte'
  import type { PageData } from './$types'

  import { listClass, listItemClass, nameTagClass } from './page.css'

  export let data: PageData
</script>

<svelte:head>
  <title>{data.title}</title>
</svelte:head>

<h1 class="name-tag {nameTagClass}">Michal Vanko</h1>

<h2 class="name-tag {nameTagClass}">
  Software Architect and Engineering Manager
</h2>

<section id="personal-information">
  {@html data.body}
</section>

<section id="work-history">
  <h2>Work experience</h2>
  <section class="work-history-prelude">
    {@html data.workHistoryPrelude}
  </section>
  <ul class={listClass}>
    {#each data.workHistory as work}
      <li class={listItemClass}>
        <Work {work} />
      </li>
    {/each}
  </ul>
</section>

<section id="projects">
  <h2>Projects</h2>
  <ul class={listClass}>
    {#each data.projects as project}
      <li class={listItemClass}>
        <Project {project} />
      </li>
    {/each}
  </ul>
</section>

<section id="presentations">
  <h2>Presentations</h2>
  <ul class="">
    {#each data.presentations as presentation}
      <li class="">
        <Presentation {presentation} />
      </li>
    {/each}
  </ul>
</section>

<section id="education">
  <h2>Education</h2>
  <ul class={listClass}>
    {#each data.education as work}
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
