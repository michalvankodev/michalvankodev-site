<script>
  export let project
</script>

<style>
  .project-image {
    float: right;
    width: 25%;
  }

</style>

<article class="project">
  <h3>{project.name}</h3>
  <section class="description">
    {#if project.image}
        <img src="{project.image.source}" class="project-image" alt="{project.image.image_description}" />
    {/if}
    {@html project.description}
  </section>
  <aside>
  </aside>
</article>
