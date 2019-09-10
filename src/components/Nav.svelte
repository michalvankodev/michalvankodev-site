<script>
	export let segment;
</script>

<style>
	/* nav {
		border-bottom: 1px solid rgba(255,62,0,0.1);
		font-weight: 300;
		padding: 0 1em;
	}

	ul {
		margin: 0;
		padding: 0;
	}

	/* clearfix */
	/* ul::after {
		content: '';
		display: block;
		clear: both;
	}

	li {
		display: block;
		float: left;
	}

	.selected {
		position: relative;
		display: inline-block;
	}

	.selected::after {
		position: absolute;
		content: '';
		width: calc(100% - 1em);
		height: 2px;
		background-color: rgb(255,62,0);
		display: block;
		bottom: -1px;
	} */

	/* a {
		text-decoration: none;
		padding: 1em 0.5em;
		display: block;
  } */
  
  nav {
    position: sticky;
    top: 0;
    background: rgb(31, 22, 94);
    z-index: 1;
    padding: 0.5em;
    box-shadow: 0px 2px 4px #090808;
  }

  nav a {
    padding: 0.4em 0.5em;
    text-decoration: none;
  }

  nav a:hover {
    text-decoration: underline
  }
</style>

<nav>
	<!-- <ul> -->
		<!-- <li><a class='{segment === undefined ? "selected" : ""}' href='.'>home</a></li> -->
		<!-- <li><a class='{segment === "portfolio" ? "selected" : ""}' href='portfolio'>portfolio</a></li> -->

		<!-- for the blog link, we're using rel=prefetch so that Sapper prefetches
		     the blog data when we hover over the link or tap it on a touchscreen -->
		<!-- <li><a rel=prefetch class='{segment === "blog" ? "selected" : ""}' href='blog'>blog</a></li> -->
	<!-- </ul> -->

  <div class="page-navigation">
	  <a href="portfolio#personal-information">About</a>
		<a href="portfolio#skills">Skills</a>
		<a href="portfolio#work-history">Work History</a>
		<a href="portfolio#projects">Projects</a>
		<a href="portfolio#education">Education</a>
  </div>
</nav>