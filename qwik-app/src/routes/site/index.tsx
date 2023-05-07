// import { generateSrcSet, getNFResize } from "$lib/large-media";
import {
  citeOwnerClass,
  mottoClass,
  // profilePicClass,
  twitchAsideClass,
  twitchEmbedClass,
  twitchIframeClass,
  twitchStreamPromoClass,
  welcomeNoteClass,
} from './page.css'
import { component$ } from '@builder.io/qwik'

export default component$(() => {
  return (
    <>
      <header class="index-header">
        {/* <figure class={`profile-pic ${profilePicClass}`}>
          <picture>
            <source
              media="(max-width: 550px)"
              srcset={generateSrcSet('/images/profile-portugal-portrait.jpg', {
                width: 500,
              })}
            ></source>
            <img
              alt="Portrait"
              srcset={generateSrcSet('/images/profile-portugal-landscape.jpg', {
                width: 800,
              })}
              src={getNFResize('/images/profile-portugal-landscape.jpg', {
                width: 800,
              })}
            ></img>
          </picture>
        </figure> */}
        <p class={`motto ${mottoClass}`}>
          <cite>“Let your ambition carry you.”</cite>
          <span class={`cite-owner ${citeOwnerClass}`}>- La Flame</span>
        </p>
      </header>
      <p class={welcomeNoteClass}>
        Hey, welcome to my personal website. My name is{' '}
        <strong>Michal Vanko</strong>
        and I'm a{' '}
        <em>
          <a href="https://en.wikipedia.org/wiki/Programmer">programmer</a>
        </em>
        . I'll try to share some stories and opinions about things that I'm
        interested in.
      </p>
      <section class={`twitch-stream-promo ${twitchStreamPromoClass}`}>
        <h2>Follow my twitch stream</h2>
        <div class={`twitch-embed ${twitchEmbedClass}`}>
          <div class={`twitch-video ${twitchIframeClass}`}>
            <iframe
              title="My twitch channel"
              src="https://player.twitch.tv/?channel=michalvankodev&parent=michalvanko.dev&parent=localhost&autoplay=false"
              loading="lazy"
              frameBorder="0"
              scrolling="no"
              height="100%"
              width="100%"
              class="embed "
              allowFullScreen={true}
            ></iframe>
          </div>
          <aside class={twitchAsideClass}>
            Come hang out and chat with me{' '}
            <strong>every Tuesday and Thursday</strong>
            afternoon central Europe time. I stream working on my side-projects
            and talking anything about the developer lifestyle.
          </aside>
        </div>
      </section>
    </>
  )
})
