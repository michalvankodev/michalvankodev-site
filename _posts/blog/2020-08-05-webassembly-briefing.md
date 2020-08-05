---
layout: blog
title: WebAssembly briefing
published: true
date: 2020-08-05T15:38:18.797Z # TODO executable line for generating date
tags:
  - News
  - WebAssembly
  - Development

notes: Dont forget the todos
---

Last week I'd organized a small briefing at work about **WebAssembly**. The goal of the briefing was to give my colleagues an overview about what WebAssembly is and how it can be used to improve our products.

I've used these notes for the discussion.

## What should have been already known

> WebAssembly is a new type of code that can be run in modern web browsers — it is a low-level assembly-like language with a compact binary format that runs with near-native performance and provides languages such as C/C++ and Rust with a compilation target so that they can run on the web. It is also designed to run alongside JavaScript, allowing both to work together.

- _https://developer.mozilla.org/en-US/docs/WebAssembly_

The list of the languages that compile to WebAssembly is getting bigger and bigger. But does it support `_.sample(popularLanguages)`? Most probably: yes, it does. Someone already made a **language designed for WebAssembly** called [Assembly script](https://www.assemblyscript.org/). 
It is heavily inspired by [Typescript](https://www.typescriptlang.org/), and it targets web developers that don't want to learn a new language.

### Motivation behind WebAssembly

> WebAssembly is a safe, portable, low-level code format designed for efficient execution and compact representation. Its main goal is to enable high performance applications on the Web, but it does not make any Web-specific assumptions or provide Web-specific features, so it can be employed in other environments as well.

- _https://webassembly.github.io/spec/core/intro/introduction.html_

As the WebAssembly programs are compiled into portable binary-code format, they are fast to transfer over the wire and also parse by clients.

Use of such technology allows new types of programs to be ran inside the web browser, such as:

- Image / Video editors
- AAA Games
- ML / AI-powered applications
- Scientific visualizations and simulations
- Live video augmentation
- Language interpreters and VMs
- CAD applications

### Brief history

- 2013 - Mozilla designed [asm.js](http://asmjs.org/), **subset of JavaScript** which allowed compiling C / C++ applications to be compiled for web while maintaining performance characteristics. 
  - Compiling is done via [Emscripten](https://emscripten.org), which is still used as of today to port C applications to WASM.
- 2015 - WebAssembly announced with a feature set of asm.js
- 2017 - Initial MVP API designed and agreed upon in between 4 major browser vendors
- 2018 - Core specification released


## Specification

Standardization process of WebAssembly is done in public at [Github](https://github.com/WebAssembly/meetings/blob/master/process/phases.md) and is very similar to standardization process of JavaScript language.

**Current version of the API is 1.1**. It is intended that it will be superseded by new incremental releases with additional features in the future.

Here is a list of latest standartized features which are being implemented in browsers: [Roadmap](https://webassembly.org/roadmap/)

## List of applications making use of WebAssembly

- [Figma](https://medium.com/figma-design/webassembly-cut-figmas-load-time-by-3x-76f3f2395164)
  - [Figma has improved load times 3x with WASM](https://medium.com/figma-design/webassembly-cut-figmas-load-time-by-3x-76f3f2395164)
- [Google Earth](https://medium.com/google-earth/google-earth-comes-to-more-browsers-thanks-to-webassembly-1877d95810d6)
- [Photon Image processing](https://silvia-odwyer.github.io/photon/)
- [Baldur's Gate II](https://personal-1094.web.app/gemrb.html)
- [Doom 3](http://wasm.continuation-labs.com/d3demo/)
- [Flash virtualization](https://medium.com/leaningtech/running-flash-in-webassembly-using-cheerpx-an-update-d500b6fbc44e)
- [AutoCAD](https://www.autodesk.com/products/autocad-web-app/overview?linkId=68719474&plc=ACDIST&term=1-YEAR&support=ADVANCED&quantity=1)
- [ebay using WASM for barcode scanning](https://tech.ebayinc.com/engineering/webassembly-at-ebay-a-real-world-use-case/?utm_source=twitter&utm_medium=Social&utm_campaign=AddThisWidget#.XOhbJgZV6Nw.twitter)
- [Dropbox compression algorithm](https://blogs.dropbox.com/tech/2018/06/building-better-compression-together-with-divans/)


## Endless possibilities

### What it could be used for

- Cross platform application development
- Hardware made directly for consuming WebAssembly instructions
  - If WASM is emergint to become the standard format for executables, tailor-made hardware chips migth become more efficient in the future. No more competing architectures.
- Transformation of the web to more efficient platform would have ecological impact on the environment. If the bundles are smaller and easier to parse that means it will significantly lower need for resources (less power consumption, cheaper hardware).


## Web application frameworks

### Blazor

> Blazor is an open source and cross-platform web UI framework for building single-page apps using .NET and C# instead of JavaScript.
> Blazor is based on a powerful and flexible component model for building rich interactive web UI.
> You implement Blazor UI components using a combination of .NET code and Razor syntax: an elegant melding of HTML and C#. Blazor components can seamlessly handle UI events, bind to user input, and efficiently render UI updates.
> 
> Blazor WebAssembly is now the second supported way to host your Blazor components: client-side in the browser using a WebAssembly-based .NET runtime. Blazor WebAssembly includes a proper .NET runtime implemented in WebAssembly, a standardized bytecode for the web. This .NET runtime is downloaded with your Blazor WebAssembly app and enables running normal .NET code directly in the browser. No plugins or code transpilation are required. Blazor WebAssembly works with all modern web browsers, both desktop and mobile. Similar to JavaScript, Blazor WebAssembly apps run securely on the user’s device from within the browser’s security sandbox. These apps can be deployed as completely standalone static sites without any .NET server component at all, or they can be paired with ASP.NET Core to enable full stack web development with .NET, where code can be effortlessly shared with the client and server.

- _https://devblogs.microsoft.com/aspnet/blazor-webassembly-3-2-0-now-available/_

![Blazor overview](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fimage.slidesharecdn.com%2Fskackovwasm-net-fwdaysimproved-180418134149%2F95%2Foleksandr-skachkov-running-in-your-web-browser-with-webassembly-44-638.jpg%3Fcb%3D1524058963&f=1&nofb=1)

### Yew

Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.

- Features a macro for declaring interactive HTML with Rust expressions. Developers who have experience using JSX in React should feel quite at home when using Yew.
- Achieves high performance by minimizing DOM API calls for each page render and by making it easy to offload processing to background web workers.
- Supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.
- [github repository](https://github.com/yewstack/yew)
- [Sample app](https://yew.rs/docs/getting-started/build-a-sample-app)

### Oak

Really simple framework for Golang [Github](https://github.com/elliotforbes/go-webassembly-framework)

## Conclusion

The discussion went for about 2 hours and it sparked many ideas. I am really glad that I've found time to do prepare and organize it.
We talked a lot about how WebAssembly could be used in front-end frameworks for computational parts to increase the speed of calculations of DOM changes. In virtual-dom frameworks for example it could calculate the diffs more efficiently.
I think, the frameworks presented are doing something similar, but with use of different languages. Which I think is for better. This sparked another discussion about programming languages. Especially how _JavaScript_ is useful enough but is also ergonomic and friendly for programmers to use?

Discussions lasted for few days. I am really excited about the feedback I've received. I'll definitely organize something similar in the near future.
