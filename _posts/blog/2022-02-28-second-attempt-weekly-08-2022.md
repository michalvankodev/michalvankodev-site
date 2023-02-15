---
layout: blog
title: "Second attempt @ Weekly #08-2022"
segments:
  - blog
published: true
date: 2022-02-28T11:49:53.914Z
tags:
  - News
  - Weekly
notes: >
  
  I went for a solution very similar to how GitHub is doing it. I've created an `iframe` and set up communication over the `postMessage` API. I was pretty much just debugging when should I fire the rendering process and whatnot. I was not bothered by handling multiple diagrams and I knew that the solution grew on complexity very quickly. On the next day, I just thought of an even simpler solution. I decided to skip the whole `iframe` shenanigans and just import the `mermaid` library dynamically into the page when the article has any diagram. This way I don't have to create any special logic around creating tokens for each diagram and the performance is even better for the client as the parsing doesn't require rendering a whole HTML document in the `iframe`.
---
When I started to write this weekly I've got so deeply into writing about the error handling use case that I basically just dedicated the whole article to it. So this will be my second attempt just to not break the tradition.

## Error handling guide

You can check out the new [article on error handling here](/blog/2022-02-28-error-handling-with-either-type). It is basically a practical continuation of the [first guide for error handling](https://michalvanko.dev/blog/2020-12-09-guide-on-error-handling) I've written a year ago.

## Stream updates

This week we have been discussing the [state of JS](https://2021.stateofjs.com/en-us/) on the Tuesday stream. It went very well and I didn't even manage to get through it to the end. I am planning to continue on the topic when I get back from vacation on the 15th of March.
I will prepare a few advertising materials (probably just a poster) and share them across social media as I want to run the stream officially.

## Mermaid diagram parsing

On the Thursday stream, I've not been able to find a guest to discuss any topics so I just went for coding the diagram support for mermaid diagrams in my posts. I wanted to make render diagrams on the backend.
I was very surprised that the mermaid is directly dependent on the browser to be able to render SVG. I had to rethink all over again how I would implement it. 

I haven't finished it, yet. So **prepare for a new article** where I will present the solution. I will stream it on [my twitch](https://www.twitch.tv/michalvankodev) so if you're interested, **please follow me there**.

Let's keep it short. Thank you!
