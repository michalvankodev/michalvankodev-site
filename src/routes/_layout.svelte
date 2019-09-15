<script>
	import Nav from '../components/Nav.svelte';

	export let segment;
</script>

<style>
  .app-content {
		background: #f2f6f6;
  }

	main {
		position: relative;
		max-width: 700px;
		padding: 2em;
		margin: 0 auto;
		box-sizing: border-box;
	}

</style>

<div class="app-content">
  <Nav {segment}/>

  <main>
    <slot></slot>
  </main>
</div>
