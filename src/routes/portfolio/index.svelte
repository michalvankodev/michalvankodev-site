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
  import Workhistory from '../../components/portfolio/workhistory.svelte';
  import Project from '../../components/portfolio/project.svelte';

  export let content
</script>

<svelte:head>
	<title>{content.title}</title>
</svelte:head>

<h1>Michal Vanko</h1>

<h2>Software Developer</h2>

<section class="personal-information">
  {@html content.body}
</section>

<Workhistory />

<section class="projects">
  <h2>Projects</h2>
  <ul>
    {#each content.projects as project}
      <li>
        <Project project={project} />
      </li>
    {/each}
  </ul>
</section>
