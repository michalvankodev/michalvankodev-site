<script lang="ts">
  import type { RecordAttributes } from '../../routes/portfolio/index.json'
  export let work: RecordAttributes
</script>

<article>
  <h3>{work.name}</h3>
  <section class="description">
    {@html work.description}
  </section>
</article>
