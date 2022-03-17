<script lang="ts">
  import type { ProjectAttributes } from '../../routes/portfolio/index.json'
  import { projectScopeClass } from './project.css'

  export let project: ProjectAttributes
</script>

<article class="project {projectScopeClass}">
  <h3>{project.name}</h3>
  <section class="description">
    {#if project.image}
      <img
        src={project.image.source}
        class="project-image"
        alt={project.image.image_description}
      />
    {/if}
    {@html project.description.parsed}
  </section>
  <aside />
</article>
