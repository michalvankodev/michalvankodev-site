<svelte:head>
  <title>Introduction @michalvankodev</title>
</svelte:head>

<header class="index-header">
  <figure class="profile-pic">
    <picture>
      <source
        media="(max-width: 550px)"
        srcset="images/profile-picture-portrait.jpg"
      />
      <img alt="My profile" src="images/profile-picture2.jpg" />
    </picture>
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
  <em> <a href="https://en.wikipedia.org/wiki/Programmer">programmer</a> </em>
  . I'll try to share some stories and opinions about things that I'm interested
  in.
</p>

<style>
  figure,
  p {
    text-align: center;
    margin: 0 auto;
  }

  p {
    margin: 1em auto;
  }

  .motto {
    font-size: 1.4em;
  }
</style>
