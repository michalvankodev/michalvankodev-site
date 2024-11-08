---
title: This site has been rewritten again
segments:
  - blog
  - featured
published: true
date: 2024-11-05T11:26:00.000Z
thumbnail: /images/uploads/m-logo.svg
tags:
  - News
  - Development
  - Rust
notes: Bu
---
Hey world,

After a few months of work, I can finally present to you a new style for this site. The style isn't the only thing that changed.
I've **completely rewritten** how this site is produced. 

## Motivation

With the front-end tech world spinning too fast I've noticed this cool little library called *[HTMX](https://htmx.org/)* that takes front-end development back 10 years and provides a completely different look at how web development could've been done. 
I've also done some experiments with [Qwik](https://qwik.dev/) before, but I'm glad that I've abandoned this route as I would like to write less and less *TypeScript* `||` *Javascript* in the future.
After doing last year's [Advent of Code](https://adventofcode.com/) in *OCaml* I've figured that I'd like to use tools that I think make me productive and also provide me help on the path of producing the most safe, predictable and performant code.
That's the reason **I've chosen *Rust*** for this site and most of my side projects in the last couple of years.

## What has changed

### Tech stack

I've mentioned *HTMX*, but it only inspired me and I took it a few steps back and I haven't opted for any *JavaScript* included by default. You can find some *JavaScript* on this site but it is only inlined without any build step. However, there still has to be a build step. Instead of setting up *[Vite](https://vite.dev/)* for 5th time. I opted for something classic but still new. I've just found the [just](https://github.com/casey/just) command runner. It is very similar to how many *Makefiles* look, but the API is modern and **written in Rust BTW**. So if any developer wants to look at what commands are being used during the development or deployment, they are all located in [`.justfile`](https://github.com/michalvankodev/michalvankodev-site/blob/main/justfile). All commands are just `bash` commands and some `bash` scripts with few instructions for `just` to combine commands by their dependencies.

### Templates

During development, the site is being hosted with [axum Rust web framework](https://github.com/tokio-rs/axum). I want to write my templates in HTML so they can be as easily reused in any other web project with any tech stack. For Rust, there is a **type-safe** Jinja like template parser [Askama](https://github.com/djc/askama) and even though I'm not very confident whether I would use it again, I can still easily migrate to any other template parser there is.

### Styling

I've completely redesigned the site. Dodging *TypeScript-based* solutions for <abbr title="Cascade style sheets">CSS</abbr>. Opting for [tailwind](https://tailwindcss.com/) running *just* in the background. The experience with *tailwind* is something that I've never felt previously with any other <abbr title="Cascade style sheets">CSS</abbr> framework before. I'm satisfied with it although it has some downsides, **the productivity hasn't been matched**.

## What stayed the same

### All the content

I haven't even moved the files to a different folder. I've just slightly adjusted the models in the [DecapCMS](https://decapcms.org/) config for [/showcase](/showcase) page. DecapCMS is pretty much just the same Netlify CMS that it has been before the rebranding.
I'd like to recommend it as *THE CMS*, but I can't. I like the way how the content is managed, I also like *git-based* workflow for managing content. However, there are many struggles to set it up for use with clients that are not programmers. With more changes to the models you also start to **miss the capability of running migrations** on all files/records. Everyone would be better off just hosting *[Strapi](https://strapi.io/)*. The ecosystem has started to get healthy. Setting up a custom media library is also a thing that has to be considered.

### Media library hosting

[Netlify has deprecated](https://answers.netlify.com/t/large-media-feature-deprecated-but-not-removed/100804) its [Large Media](https://docs.netlify.com/git/large-media/setup/) service. I don't mind tracking media with `git` LFS plugin. What I do mind is where I store these assets. \[Github's] free quota is pretty low considering sharing photo galleries. For now, I've moved all assets on my *"powered by"* [selfhosted](https://awesome-selfhosted.net/index.html) server, running my own [Gitea](https://gitea.katelyn.michalvanko.dev/) instance.

### Static site generation

The site is still distributed as <abbr title="Static site generation">SSG</abbr> HTML files. You could argue that the logic for generating every page is just like any other <abbr title="Server Side Rendered">SSR</abbr> website.

For <abbr title="Server side generation">SSG</abbr> I've come up with a `wget` command that downloads all the necessary dependencies for all pages. It is capable of recursively crawling the whole site following all links. It was pretty hard to come up with the correct set of parameters for the `wget` command to be able to produce the same routing capabilities as with the `SSR` running server used during the development. Here I can **praise all the generative AI tools**. You could find multiple prompts asking for an explanation of all the options for `wget` and how I should use them for the desired output in my history.

The final `wget` command for generating this site looks like this:

```sh
wget --no-convert-links -r -p -E -P dist --no-host-directories localhost:3080
```

You can also [prompt for the explanation](https://lmgpthat.com/render.html?search=wget%20--no-convert-links%20-r%20-p%20-E%20-P%20dist%20--no-host-directories%20localhost%3A3080).

## Aim for future

I still have to **write articles**.
Many many articles that I haven't yet written down. I did some but haven't published them, and probably I never will. But so many things happened in the meantime. How do you call an update that is old? *"Outofdate"*?
It would also be handy to have some article recommendations when you finish reading this article, right?

I am still gathering feedback for the new design. I am considering many of the suggestions that I get. Our [Discord server](https://discord.gg/2cGg7kwZEh) is getting warm.

I was also thinking about what would I use for CMS and having a *[SQLite](https://www.sqlite.org/docs.html)* database saved in the repository, it would still count as **git-based management**.

*'Let your ambition carry you!"*
