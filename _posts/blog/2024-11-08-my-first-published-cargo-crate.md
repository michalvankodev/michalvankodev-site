---
title: My first published Cargo crate
segments:
  - blog
  - featured
published: true
date: 2024-11-08T17:09:00.000Z
thumbnail: /images/uploads/aperture-icon.svg
tags:
  - News
  - Development
  - Rust
---
I've become obsessed with cameras, during our Tenerife trip, where I got to use a borrowed mirrorless camera for the first time. After that, I researched which camera I'd buy for myself for five months. While watching all the reviews, I was interested in the **exposure settings** for each picture presented. I've displayed many photographs in our [Tenerife vlog with my wife](https://www.youtube.com/watch?v=tEpoVHQW4Qs&list=PLjUl8tFKyR8rCsckLn93PAwQg6tf0cyBl). I've decided that I'd like to show those exposure settings in my other videos as they might **inspire other photographers**.

I've tried many image editors and tools, but not one was able to **fulfill my demands** on how I'd like to display those settings underneath each picture. Many **tools supported** some kind of **framing**, however it was always **in a limited way**.

## Hello `metaframer`

I knew straight away that this would be a cool and **small side-project** that I could create. I could use my **favorite technologies** and have good **content for my streams**. 

At first, I thought, that I would have to **render the text** right **into the image**. It will however be in some aspect a **deformation to the quality** of the original image. There was also a question of **how should the text be displayed** in a way that for **any image size**, the text will be of a similar or determinable size.

Instead of **rendering pixels** into every different format I've opted for creating an **SVG file** that sits **alongside the original image**. This way the text also **scales with the image** itself. The complexity was lowered instantly. Rendering SVG is just like rendering HTML. I've used [handlebars template parser](https://handlebarsjs.com/). There are more favorable template parsers in Rust like [Askama](https://djc.github.io/askama/askama.html). But I wanted to allow users to **create their own templates** so the templates are not part of the executables and they can be stored in the user's `.config` folder. I had to test the **support of my video editing software** (I use [Kdenlive](https://kdenlive.org/) BTW) and the decision for SVG was set in stone (a.k.a. [README](https://github.com/michalvankodev/metaframer)).

There were still complications along the way. **SVG** is still a much **less capable format than HTML** and **positioning elements** in a `space-around` fashion **has to be calculated**. There are also many different options and resolutions that I had to consider and I put all of them into the CLI arguments with [clap](https://docs.rs/clap/latest/clap/). [Clap](https://docs.rs/clap/latest/clap/) is a fantastic **command line argument parser** for Rust. I had to do many calculations on the resolutions and whether the image was a portrait or not. In the end, It wasn't as small of a side-project as I thought. I'm still very proud of it.

I got to experience the ease of publishing crates on [crates.io](https://crates.io/) with [cargo](https://doc.rust-lang.org/cargo/guide/).

This week, I've **automatized the [release process](https://github.com/michalvankodev/metaframer/blob/main/.github/workflows/release.yml)** and published binaries for multiple platforms.

## On to the next one

I can't wait to use [`metaframer`](https://github.com/michalvankodev/metaframer) in my new videos. I've already **ordered my first mirrorless camera** and it is on the way. **Be ready for new content**. What should be my next project?
