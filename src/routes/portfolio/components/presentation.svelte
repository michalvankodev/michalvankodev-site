<script lang="ts">
  import type { PresentationAttributes } from 'src/routes/portfolio/index.json'
  import {
    presentationDescriptionClass,
    presentationFrameClass,
    presentationNameClass,
    presentationPreviewLinksClass,
  } from './presentation.css'

  export let presentation: PresentationAttributes
  export let previewVisible = false

  function togglePreviewVisible() {
    previewVisible = !previewVisible
  }
</script>

<article>
  <a href={presentation.link} target="_blank">
    <h3 class={presentationNameClass}>{presentation.name}</h3>
  </a>

  <section class="description {presentationDescriptionClass}">
    {@html presentation.description}
  </section>

  <section class="preview">
    <div class={presentationPreviewLinksClass}>
      <a href="#presentations" on:click|preventDefault={togglePreviewVisible}
        >{previewVisible ? 'Close' : 'Open'} preview</a
      >
    </div>
    {#if previewVisible}
      <iframe
        class={presentationFrameClass}
        src={presentation.link}
        title="Presentation of {presentation.name}"
      />
    {/if}
  </section>
</article>
