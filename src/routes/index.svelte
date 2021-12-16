<script lang="ts">
  import { generateSrcSet, getNFResize } from '$lib/large-media'
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
        srcset={generateSrcSet('images/profile-portugal-portrait.jpg', {
          width: 500,
        })}
      />
      <img
        alt="Portrait"
        srcset={generateSrcSet('images/profile-portugal-landscape.jpg', {
          width: 800,
        })}
        src={getNFResize('images/profile-portugal-landscape.jpg', {
          width: 800,
        })}
      />
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
