<script context="module">
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

<script>
  import Work from '../../components/portfolio/work.svelte'
  import Project from '../../components/portfolio/project.svelte'

  export let content
</script>

<svelte:head>
  <title>{content.title}</title>
</svelte:head>

<h1 class="name-tag">Michal Vanko</h1>

<h2 class="name-tag">Software Architect and Consultant</h2>

<section id="personal-information">
  {@html content.body}
</section>

<section id="work-history">
  <h2>Work experience</h2>
  <section class="work-history-prelude">
    {@html content.workHistoryPrelude}
  </section>
  <ul>
    {#each content.workHistory as work}
      <li>
        <Work {work} />
      </li>
    {/each}
  </ul>
</section>

<section id="projects">
  <h2>Projects</h2>
  <ul>
    {#each content.projects as project}
      <li>
        <Project {project} />
      </li>
    {/each}
  </ul>
</section>

<section id="education">
  <h2>Education</h2>
  <ul>
    {#each content.education as work}
      <li>
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

  section[id] {
    margin-top: 2em;
  }

  h1 {
    text-shadow: 2px 2px 1px #c8c4b7; // TODO
  }

  .name-tag {
    text-align: center;
  }

  ul {
    list-style: none;
    padding: 0;
  }

  li {
    margin: 1.5em 0;
  }
</style>
