<script context="module">
  // export async function preload() {
  //   return this.redirect(302, 'portfolio')
  // }
</script>

<style>
  figure,
  p {
    text-align: center;
    margin: 0 auto;
  }

  figure {
    margin: 0 0 1em 0;
  }

  img {
    width: 100%;
    max-width: 400px;
    margin: 0 0 1em 0;
  }

  p {
    margin: 1em auto;
  }
</style>

<svelte:head>
  <title>michalvanko.dev index page</title>
</svelte:head>

<header class="index-header">
  <figure class="profile-pic">
    <img alt="Borat" src="great-success.png" />
  </figure>

  <p class="motto">
    <cite>“Let your ambition carry you.”</cite>
    <span>- La Flame</span>
  </p>
</header>

<p>
  Hey, welcome to my personal website. My name is
  <strong>Michal Vanko</strong>
  and I'm a
  <em>
    <a href="https://en.wikipedia.org/wiki/Programmer">programmer</a>
  </em>
  . I'll try to share some stories and opinions about things that I'm interested
  in.
</p>
