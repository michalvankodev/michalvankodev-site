import path from 'path'

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
    const file = path.parse(href)
    const figcaption = title ? `<figcaption>${title}</figcaption>` : ''

    return `
      <figure>
        <picture>
          <source srcset="${file.dir}/optimized/${file.name}.webp" type="image/webp" />
          <img alt="${text}" src="${href}" />
        </picture>
        ${figcaption}
      </figure>
    `
  },
}
