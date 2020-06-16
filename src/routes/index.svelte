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
    max-width: 420px;
    margin: 0 0 1em 0;
    border-radius: 5px;
    box-shadow: 0px 0px 8px 1px #2d3935;
  }

  p {
    margin: 1em auto;
  }

  .motto {
    font-size: 1.4em;
  }
</style>

<svelte:head>
  <title>Introduction @michalvankodev</title>
</svelte:head>

<header class="index-header">
  <figure class="profile-pic">
    <img alt="My eye" src="eye.png" />
  </figure>

  <p class="motto">
    <cite>“Let your ambition carry you.”</cite>
    <span>- La Flame</span>
  </p>
</header>

<p>
  Hey, welcome to my personal website. My name is
  <strong>Michal&nbsp;Vanko</strong>
  and I'm a
  <em>
    <a href="https://en.wikipedia.org/wiki/Programmer">programmer</a>
  </em>
  . I'll try to share some stories and opinions about things that I'm interested
  in.
</p>
