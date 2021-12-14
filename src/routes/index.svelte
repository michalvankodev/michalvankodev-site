<script lang="ts">
  import {
    citeOwnerClass,
    mottoClass,
    profilePicClass,
    welcomeNoteClass,
  } from './index.css'
</script>

<svelte:head>
  <title>Introduction @michalvankodev</title>
</svelte:head>

<header class="index-header">
  <figure class="profile-pic {profilePicClass}">
    <picture>
      <source
        media="(max-width: 550px)"
        srcset="images/profile-picture-portrait.jpg"
      />
      <img alt="My profile" src="images/profile-picture2.jpg" />
    </picture>
  </figure>

  <p class="motto {mottoClass}">
    <cite>“Let your ambition carry you.”</cite>
    <span class="cite-owner {citeOwnerClass}">- La Flame</span>
  </p>
</header>

<p class={welcomeNoteClass}>
  Hey, welcome to my personal website. My name is
  <strong>Michal&nbsp;Vanko</strong>
  and I'm a
  <em> <a href="https://en.wikipedia.org/wiki/Programmer">programmer</a> </em>
  . I'll try to share some stories and opinions about things that I'm interested
  in.
</p>
