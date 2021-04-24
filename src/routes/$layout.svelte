<script context="module" lang="typescript">
  import { take } from 'ramda'

  export function load({ fetch }) {
    return fetch(`/blog.json`)
      .then((r) => r.json())
      .then((posts) => {
        return { props: { latestPosts: take(5, posts) }}
      })
  }
</script>

<script lang="typescript">
  import Nav from '../components/Nav.svelte'
  import Footer from '../components/Footer.svelte'
  export let segment
  export let latestPosts
</script>

<style>
  .app-content {
    display: grid;
    grid-template-rows: auto 1fr auto;
    background: #f2f6f6;
    min-height: 100vh;
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
