---
title: Auto-commit & push git repositories
segments:
  - blog
published: true
date: 2024-11-23T15:23:00.000Z
thumbnail: /images/uploads/git-icon-logo-svg-vector.svg
tags:
  - News
  - Development
  - Rust
notes: ""
---
## Motivation

I've been **writing my notes** for over 4 years in **markdown**. I write my notes in similar style of a [Zettelkasten method](https://zettelkasten.de/introduction/). I was **inspired by [foambubble](https://github.com/foambubble/foam) project** and was using it for quite some time. The goal of foambubble was to re-use as much **free technology** for writing and **keeping second brain**. The author [Jani Eväkallio](https://jevakallio.dev/) has chosen to write **a plugin for VSCode** and **utilize git workflow**.
This approach as complicated for average human was very appealing to me as I was already user of certain tools.

Since then, I stopped using VSCode as my code editor and therefore, as there wasn't support for *foam* outside of VSCode I've **migrated to a simpler workflow** using [zk](https://github.com/zk-org/zk).

Since then, I have struggled a lot with **forgetting to commit** my changes as I often write updates, mark completed tasks, or write new TODOs throughout the day.
I use a computer and a laptop when on the road and it is miserable when you forget to push your changes. I'm still trying to find a perfect workflow for **synchronizing notes with my mobile phone**. I've used many different applications, but they **all failed** me as soon as there was a **git conflict**. I think I have a solution for this thought through.

A very easy solution would be to run a [script generated with GPT](https://letmegpt.com/?q=I'd%20like%20to%20automatically%20commit%20and%20push%20to%20my%20git%20repository%20after%205%20minutes%20of%20inactivity%20on%20my%20changes.) to **auto-commit**. It has the downsides of always having to remember to start it and it would not be easy to **share and apply to other repositories**. So, I came up with another **Rust crate**. 

## Welcome `git_afk`

`git_afk` might stand for *get always forgotten knowledge*. It doesn't. I've **chosen Rust** again. As I use it more frequently, my appreciation for it grows. I feel like Rust just makes you manage the state in the place where the state belongs. Working on this project taught me how to keep and share asynchronous callbacks in a way that makes sense but isn't so obvious at first glance. The **compiler is always very helpful**. While in other languages you would need to keep `unsubscribe` handles all over the place to **avoid memory leaks**. Rust makes you think differently about how to structure the code and data around it.

The application is a **CLI app** that has a basic ability to add repositories to the watch list and then run `watch` command.
Anytime there is a change in the configuration file it will reload watchers and start watching all files in the repositories for changes. It uses [`inotify`](https://www.man7.org/linux/man-pages/man7/inotify.7.html) a kernel interface for watching. You can find the [source code on GitHub](https://github.com/michalvankodev/git_afk).

As the Rust ecosystem is so simple you can install `git_afk` with [`cargo`](https://crates.io/) `cargo install git_afk`. Submitting packages is easy. Installing them as well. **Automatic documentation generation** is awesome. Every package has the same style of documentation. Very easy to navigate. Rust is growing up on me. I'd very much like to use it as my main tool for my job.

Now I can safely *git away from keyboard*.
