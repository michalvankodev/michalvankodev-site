import { generateSrcSet, getNFResize } from '$lib/large-media'
import Prism from 'prismjs'
import loadLanguages from 'prismjs/components/index.js'
import { createMermaidDiv } from './diagrams'

loadLanguages(['bash', 'markdown', 'json', 'yaml', 'typescript'])

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
    const src = isLocal ? getNFResize(href, { height: 800, width: 800 }) : href
    const srcset = isLocal
      ? `srcset="${generateSrcSet(href, { width: 800, height: 800 })}"`
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

    if (lang === 'mermaid') {
      return createMermaidDiv(source)
    }

    const highlightedSource = Prism.highlight(
      source,
      Prism.languages[usedLang],
      usedLang
    )
    return `<pre class='language-${usedLang}'><code class='language-${usedLang}'>${highlightedSource}</code></pre>`
  },
}
