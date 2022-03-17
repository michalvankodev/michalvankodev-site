<script lang="ts">
  import type { WorkAttributes } from '../../routes/portfolio/index.json'
  import { workAddressNameClass, workFooterClass } from './work.css'
  import { horizontalBorderTopClass } from '$lib/styles/scoops.css'
  export let work: WorkAttributes
</script>

<article>
  <h3>{work.name}</h3>
  <section class="description">
    {@html work.description.parsed}
  </section>

  {#if work.address}
    <footer class="{workFooterClass} {horizontalBorderTopClass}">
      <h4 class={workAddressNameClass}>{work.address.name}</h4>
      <address>
        <div>
          {work.address.location},
          {work.address.zipcode}
          {work.address.city},
          {work.address.country}
        </div>
      </address>
    </footer>
  {/if}
</article>
