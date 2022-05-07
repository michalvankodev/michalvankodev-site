<script lang="ts">
  import { format } from 'date-fns'
  import type { PostContent } from 'src/routes/blog/_content'
  import SvgIcon from './SvgIcon.svelte'
  import {
    boldClass,
    bottomLineClass,
    dateClass,
    headerClass,
    hrClass,
    latestPostsClass,
    listLiClass,
    listUlClass,
    nestedListLiClass,
    sectionListsClass,
    sectionListSectionClass,
    siteFooterClass,
    socialLinkClass,
    socialLinkLabelClass,
    strokeSvgClass,
    svgClass,
    licenceText,
  } from './Footer.css'

  export let latestPosts: PostContent[]
</script>

<footer class="site-footer navigation-theme {siteFooterClass}">
  <div class="lists {sectionListsClass}">
    <section class="site-map {sectionListSectionClass}">
      <ul class={listUlClass}>
        <li class={listLiClass}>
          <a href="/">Introduction</a>
        </li>
        <li class={listLiClass}>
          <a href="/portfolio">Portfolio</a>
          <ul class={listUlClass}>
            <li class={nestedListLiClass}>
              <a href="/portfolio#personal-information">About</a>
            </li>
            <li class={nestedListLiClass}>
              <a href="/portfolio#skills">Skills</a>
            </li>
            <li class={nestedListLiClass}>
              <a href="/portfolio#work-history">Work History</a>
            </li>
            <li class={nestedListLiClass}>
              <a href="/portfolio#projects">Projects</a>
            </li>
            <li class={nestedListLiClass}>
              <a href="/portfolio#education">Education</a>
            </li>
          </ul>
        </li>
      </ul>
    </section>
    <section class="latest-posts {sectionListSectionClass} {latestPostsClass}">
      <h3 class={headerClass}>
        <a href="/blog">Latest posts</a>
      </h3>
      <ul class={listUlClass}>
        {#each latestPosts as post}
          <li class={listLiClass}>
            <a rel="prefetch" href="/blog/{post.slug}">
              <span>{post.title}</span>
              <time class="date {dateClass}" datetime={post.date}>
                - {format(new Date(post.date), 'do MMM, yyyy')}
              </time>
            </a>
          </li>
        {/each}
      </ul>
      <hr class={hrClass} />
      <section class="subscribe {boldClass}">
        <a href="/feed.xml" rel="external" title="RSS feed" class="rss">
          Subscribe
          <SvgIcon name="rss" className={svgClass} />
        </a>
        <a
          href="/feed.json"
          rel="external"
          title="JSON feed"
          class="json-feed"
          aria-label="Subscribe with JSON feed"
        >
          <SvgIcon name="json-feed" className={svgClass} />
        </a>
      </section>
    </section>
    <section class="socials {sectionListSectionClass}">
      <h3 class={headerClass}>Contact</h3>
      <ul class="social-links {listUlClass}">
        <li class="email {listLiClass}">
          <a
            class={socialLinkClass}
            href="mailto: michalvankosk@gmail.com"
            title="E-mail address"
          >
            <SvgIcon name="mail" className={svgClass} />
            <span class={socialLinkLabelClass}>michalvankosk@gmail.com</span>
          </a>
        </li>
        <li class="twitter {listLiClass}">
          <a
            class={socialLinkClass}
            href="https://twitter.com/michalvankodev"
            title="Twitter profile"
          >
            <SvgIcon name="twitter" className={strokeSvgClass} />
            <span class={socialLinkLabelClass}>Twitter</span>
          </a>
        </li>
        <li class="github {listLiClass}">
          <a
            class={socialLinkClass}
            href="https://github.com/michalvankodev"
            title="Github profile"
          >
            <SvgIcon name="github" className={strokeSvgClass} />
            <span class={socialLinkLabelClass}>Github</span>
          </a>
        </li>
        <li class="linkedin {listLiClass}">
          <a
            class={socialLinkClass}
            href="https://www.linkedin.com/in/michal-vanko-dev/"
            title="LinkedIn profile"
          >
            <SvgIcon name="linkedin" className={svgClass} />
            <span class={socialLinkLabelClass}>LinkedIn</span>
          </a>
        </li>
        <li class="twitch {listLiClass}">
          <a
            class={socialLinkClass}
            href="https://twitch.tv/michalvankodev"
            title="Twitch profile"
          >
            <SvgIcon name="twitch" className={svgClass} />
            <span class={socialLinkLabelClass}>Twitch</span>
          </a>
        </li>
        <li class="instagram {listLiClass}">
          <a
            class={socialLinkClass}
            href="https://www.instagram.com/michalvankodev/"
            title="Instagram profile"
          >
            <SvgIcon name="instagram" className={svgClass} />
            <span class={socialLinkLabelClass}>Instagram</span>
          </a>
        </li>
      </ul>
    </section>
  </div>
  <footer class={bottomLineClass}>
    <p
      class={licenceText}
      xmlns:cc="http://creativecommons.org/ns#"
      xmlns:dct="http://purl.org/dc/terms/"
    >
      <a
        property="dct:title"
        rel="cc:attributionURL"
        href="https://michalvanko.dev/">michalvanko.dev</a
      >
      by
      <a
        rel="cc:attributionURL dct:creator"
        property="cc:attributionName"
        href="https://michalvanko.dev/">Michal Vanko</a
      >
      is licensed under
      <a
        href="http://creativecommons.org/licenses/by-nc-nd/4.0/?ref=chooser-v1"
        target="_blank"
        rel="license noopener noreferrer"
        style="display:inline-block;"
        >CC BY-NC-ND 4.0<img
          style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
          src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1"
          alt="cc"
        /><img
          style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
          src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1"
          alt="by"
        /><img
          style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
          src="https://mirrors.creativecommons.org/presskit/icons/nc.svg?ref=chooser-v1"
          alt="nc"
        /><img
          style="height:22px!important;margin-left:3px;vertical-align:text-bottom;"
          src="https://mirrors.creativecommons.org/presskit/icons/nd.svg?ref=chooser-v1"
          alt="nd"
        /></a
      >
    </p>
  </footer>
</footer>
