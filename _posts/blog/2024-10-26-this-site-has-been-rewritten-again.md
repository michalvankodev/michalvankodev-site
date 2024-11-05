---
title: This site has been rewritten again
segments:
  - blog
published: false
date: 2024-10-26T22:30:00.000Z
tags:
  - News
  - Development
  - Rust
notes: Bu
---
Hey world,

After few months of work I can finally present to you new style for this site. The style isn't the only thing that changed.
I've completely rewritten how this site is produced. 

## Motivation

With the front-end tech world spinning too fast I've noticed this cool little library called HTMX that takes us back 10 years and provides completely different look at how web development could be done. 
I've also doing some experiments with Qwik before, but I'm glad that I've abandoned this route as I would like to write less and less _TypeScript_ `||` _Javascript_ in the future.
After doing last years [Advent of Code](https://adventofcode.com/) in _OCaml_ I've figured that I'd like to use tools that I think make me productive and also provide me help on the path of producing the most safe, predictable and performant code.
That's the reason I've chosen _Rust_ for this site and most of my side projects in last couple of years.

## What changed

### Tech stack

I've mentioned HTMX, but it only inspired me and I took it even a next step back and I haven't opted for any _JavaScript_ included by default. You can find some _JavaScript_ on this site but it is just inlined without any build step. However there still has to be a build step. Instead of setting up _vite_ for 5th time. I opted for something classic but still new. I've just find [just](https://github.com/casey/just) command runner. It is very similar to how many _Makefiles_ look like but the API is modern and  **written in Rust BTW**. So if any developer wants to look like what commands are being used during the development or deployment, they are all located in [`.justfile`](https://github.com/michalvankodev/michalvankodev-site/blob/main/justfile). All commands are just `bash` commands and some `bash` scripts with few instructions for `just` to combine commands by their dependencies.

During development the site is being hosted with [axum Rust web framework](https://github.com/tokio-rs/axum). I want to write my templates in HTML so they can be most easily reused in any other web project with any tech stack. For Rust, there is a **type-safe** Jinja like template parser [Askama](https://github.com/djc/askama) and even though I'm not very confident wether I would use it again, I can still easily migrate to any other template parser there is.

I've completely redesigned the site. Dodging _TypeScript_ based solutions for <abbr title="Cascade style sheets">CSS</abbr>. Opting for [tailwind](https://tailwindcss.com/) running _just_ in the background. The experience with _tailwind_ is something that I've never had felt previously with any other <abbr title="Cascade style sheets">CSS</abbr> framework before. I'm satisfied with it altought it has some downsides, the productivity hasn't been matched.

## What stayed the same

### All the content

I haven't even moved the files to a different folder. I've just slightly adjusted the models in the DecapCMS config for [/showcase](/showcase) page. Decap CMS is pretty much just the same Netlify CMS that it has been before the rebranding.
I would like to recommend it as _THE CMS_, but I can't. I like the way how the content is managed, I also like _git_ based workflow for managing content. But there are many struggles to set it up for use with clients that are not programmers. With more changes to the models you also start to miss the capability of running migrations on all files/records. Everyone would be better of just hosting _Strapi_. The ecosystem starts to get healthy. Setting up custom media library is also a thing that has to be concidered.

### Media library hosting

[Netlify has deprecated](https://answers.netlify.com/t/large-media-feature-deprecated-but-not-removed/100804) their [Large Media](https://docs.netlify.com/git/large-media/setup/) service. I don't mind tracking media with `git` LFS plugin. What I do mind is where do I store these assets. [Github's] free quota is pretty low considering sharing photo galleries. For now, I've moved all assets on my "powered by" [selfhosted](https://awesome-selfhosted.net/index.html) server, running my own [Gitea](https://gitea.katelyn.michalvanko.dev/) instance.

### Static site generation

Site is still distributed as <abbr title="Static site generation">SSG</abbr> html files. You could argue that the logic for generating every page is just like any other <abbr title="Server Side Rendered">SSR</abbr> website.

For <abbr title="Server side generation">SSG</abbr> I've came up with a `wget` command that downloads all the necessarry dependencies of all pages. It also is capable of recursively crawling the whole site following all links. It was pretty hard to come up with the correct set of parameters for `wget` to be able to produce same routing capabilities as with the classic `SSR` running server used during the development process. Here I can **praise all the generative AI tools**. You could find multiple prompts asking for an explanation of all the options of `wget` and how I should use them for desired output in my history.

The final `wget` command for generating this site looks like this:

```sh
wget --no-convert-links -r -p -E -P dist --no-host-directories localhost:3080
```
You can also [prompt for the explanation](https://lmgpthat.com/render.html?search=wget%20--no-convert-links%20-r%20-p%20-E%20-P%20dist%20--no-host-directories%20localhost%3A3080).

## Aim for future

I still have to write articles.
Many many articles that I haven't yet wrote down. But I did some but haven't published them, and probably I never will. But so many things happened in the meantime. How do you call an update that is old? _"Outofdate"_?
It would also be handy to have some article recommendations when you finish reading this article, right?

I am still gathering feedback for the new design. And I am considering many of the suggestions that I get. Our [Discord server](https://discord.gg/2cGg7kwZEh) is getting warm.

I was also thinking about what would I use for CMS and having a [_SQLite_](https://www.sqlite.org/docs.html) database saved in the repository, it would still count as **git based management**.

Let your ambition carry you!
