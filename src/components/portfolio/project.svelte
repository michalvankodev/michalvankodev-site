<script>
  export let project
</script>

<article class="project">
  <h3>{project.name}</h3>
  <section class="description">
    {@html project.description}
  </section>
  {#if project.image}
    <aside>
      <img src="{project.image}" class="project-image" alt="{project.image.description}" />
    </aside>
  {/if}
  <aside>
  </aside>
</article>
