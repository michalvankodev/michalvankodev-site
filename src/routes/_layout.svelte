<script>
	import Nav from '../components/Nav.svelte';

	export let segment;
</script>

<style>
  .app-content {
		background: linear-gradient(176deg, rgb(31, 22, 94) 0%,rgb(67, 117, 236) 100%);
    background-attachment: fixed;
		color:#f2f0f0;
  }

	main {
		position: relative;
		max-width: 56em;
		padding: 2em;
		margin: 0 auto;
		box-sizing: border-box;
	}

  :global(::selection) {
    background: #0dd0d0;
  }

</style>

<div class="app-content">
  <Nav {segment}/>

  <main>
    <slot></slot>
  </main>
</div>
