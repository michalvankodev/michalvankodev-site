<script context="module">
  import { take } from 'ramda'

  export function preload({ params, query }) {
    return this.fetch(`blog.json`)
      .then(r => r.json())
      .then(posts => {
        return { latestPosts: take(5, posts) }
      })
  }
</script>

<script>
  import Nav from '../components/Nav.svelte'
  import Footer from '../components/Footer.svelte'
  export let segment
  export let latestPosts
</script>

<style>
  .app-content {
    background: #f2f6f6;
  }

  main {
    position: relative;
    max-width: 34em;
    padding: 2em;
    margin: 0 auto;
    box-sizing: border-box;
  }
</style>

<div class="app-content">
  <Nav {segment} />

  <main>
    <slot />
  </main>
  <Footer {latestPosts} />
</div>
