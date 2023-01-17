<script lang="ts">
  import type { LayoutData } from "./$types"
  import Nav from '../components/Nav.svelte'
  import Footer from '../components/Footer.svelte'
  import 'modern-normalize/modern-normalize.css'
  import '$lib/styles/global.css'
  import { mainContentClass } from './layout.css'

  export let data: LayoutData
  export let latestPosts = data.latestPosts
</script>

<div class="app-content">
  <Nav />

  <main class={mainContentClass}>
    <slot />
  </main>
  <Footer {latestPosts} />
</div>
