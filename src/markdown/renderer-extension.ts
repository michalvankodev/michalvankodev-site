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
    const src = isLocal ? getNFResize(href, 640, 640) : href
    const srcset = isLocal
      ? `srcset="${getNFResize(href, 640, 640)}, ${getNFResize(
          href,
          960,
          960
        )} 1.5x, ${getNFResize(href, 1280, 1280)} 2x"`
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
}
