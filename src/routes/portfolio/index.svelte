<script context="module">
  export async function preload() {
    const res = await this.fetch('portfolio.json')
    const content = await res.json()
    return {
      content,
    }
  }
</script>

<script>
  import Work from '../../components/portfolio/work.svelte';
  import Project from '../../components/portfolio/project.svelte';

  export let content
</script>

<style>
  :global([id])::before {
    content: '';
    display: block;
    height:      3em;
    margin-top: -3em;
    visibility: hidden;
  }

  section[id] {
    margin-top: 2em;
  }

  h1 {
    text-shadow: 2px 2px 1px #c8c4b7;
  }

  #personal-information :global(h3), #personal-information :global(h2) {
    margin: 1em 0 0.5em;
  }

  ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  li {
    margin: 1.5em 0;
  }
</style>

<svelte:head>
	<title>{content.title}</title>
</svelte:head>

<h1>Michal Vanko</h1>

<h2>Software Developer</h2>

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
        <Work work={work} />
      </li>
    {/each}
  </ul>
</section>

<section id="projects">
  <h2>Projects</h2>
  <ul>
    {#each content.projects as project}
      <li>
        <Project project={project} />
      </li>
    {/each}
  </ul>
</section>

<section id="education">
  <h2>Education</h2>
  <ul>
    {#each content.education as work}
      <li>
        <Work work={work} />
      </li>
    {/each}
  </ul>
</section>
