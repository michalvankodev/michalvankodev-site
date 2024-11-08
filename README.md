# michalvanko.dev site

This is the repository for my own site hosted at https://michalvanko.dev

Feel free to use and ammend to code to your needs.
Respect the [Creative Commons BY-NC-ND 4.0 License](https://creativecommons.org/licenses/by-nc-nd/4.0/?ref=chooser-v1) for the content of the site.

## Architecture

The site is hosted as a static generated HTML files on the server via [Caddy](https://caddyserver.com/) reverse proxy. There is an [example Caddyfile](./Caddyfile-preview) that can be used for deployment on server.
During development the axum web framework serves content as a HTTP server in a classic SSR HTML.

### Development

Look at the [justfile](./justfile) for the available commands that are being used for development and deployment.

Use `just server_dev` or `just dev` for running the server for development purpose.

### Tools and libraries used for generating the content

- [Decap CMS](https://decapcms.org/)
- [Rust](https://www.rust-lang.org/)
- [axum web framework](https://github.com/tokio-rs/axum)
- [tailwind](https://tailwindcss.com/)
- [wget](https://www.gnu.org/software/wget/) for SSG

### Deployment

Deployment requires these steps:

1. Ensure all images are generated
  1.1 Run server in either `dev` or `production` mode `just prod` 
  1.2 Crawl the site with `just ssg` command to ensure all routes are being hit to indicate that all images have to be generated.
  1.3 Wait till the server stops generating images. Monitor the CPU load until it drops. Takes few minutes.
2. `just export` will start the server in `production` mode and use `wget` to recursively crawl the site. Remember, content has to be linked somewhere on the site to be discovered by `wget`.
3. `just deploy` will synchronise the `/dist` folder with the server with `rsync`

### Image generation

I want all images to be served to users optimally. 
All images that are used are generated in several sizes so they are optimized for different displays sizes.
Browsers will pick and download the appropriate size.

I'd love to link some references for this problem, but I haven't found the exact use case that I was trying to solve.
