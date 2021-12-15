import Prism from 'prismjs'
import loadLanguages from 'prismjs/components/index.js'

loadLanguages(['bash', 'markdown', 'json', 'yaml', 'typescript'])

function getNFResize(href: string, height: number, width: number) {
  return `${href}?nf_resize=fit&h=${height}&w=${width}`
}

export const renderer = {
  heading(text: string, level: string) {
    const escapedText = text.toLowerCase().replace(/[^\w]+/g, '-')

    return `
      <h${level}>
        <a name="${escapedText}" class="anchor" href="#${escapedText}">
          <span class="header-link"></span>
        </a>
        ${text}
      </h${level}>
    `
  },
  image(href: string, title: string, text: string) {
    const figcaption = title ? `<figcaption>${title}</figcaption>` : ''
    const isLocal = !href.startsWith('http')
    const src = isLocal ? getNFResize(href, 800, 800) : href
    const srcset = isLocal
      ? `srcset="${getNFResize(href, 800, 800)}, ${getNFResize(
          href,
          1200,
          1200
        )} 1.5x, ${getNFResize(href, 1600, 1600)} 2x"`
      : ''

    return `
      <figure>
        <img
          alt="${text}"
          ${srcset}
          src="${src}"
        />
        ${figcaption}
      </figure>
    `
  },
  code(source: string, lang?: string) {
    // When lang is not specified it is usually an empty string which has to be handled
    const usedLang = !lang ? 'shell' : lang
    const highlightedSource = Prism.highlight(
      source,
      Prism.languages[usedLang],
      usedLang
    )
    return `<pre class='language-${usedLang}'><code class='language-${usedLang}'>${highlightedSource}</code></pre>`
  },
}
