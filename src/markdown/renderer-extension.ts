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
    const src = isLocal ? `${href}?nf_resize=fit&h=640&w=640` : href

    return `
      <figure>
        <img
          alt="${text}"
          src="${src}"
        />
        ${figcaption}
      </figure>
    `
  },
}
