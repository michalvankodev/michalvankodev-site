---
layout: blog
title: I've made a RSS feed
published: true
date: 2020-06-15T15:48:18.797Z # TODO executable line for generating date
tags:
  - News
  - Svelte
  - RSS
notes: Dont forget the todos
---

I was thinking of the ways how could people subscribe to the blog without checking it periodically or me having to post on social media any time that I post a new article.
The first thing that came to my mind was an RSS feed. I thought that it should be very easy to create one (and it is) so I started to work on it last weekend.
RSS is pretty old specification (talking in software years) and there are few alternatives that are trying to make online syndication more generic or better said adaptable for broader / modern use.

## What is RSS feed

RSS stands for _Rich Site Summary_. It is a specification for a `XML` formatted file which should provide short summary of the site content. First version was built in 1999 and the standard is still _de facto_ standard across the web.

The feed should be short so the consuming applications can download these feeds on regular basis without too much data being transferred.

As I was looking into how could I implement RSS feed for this blog I was pretty unsatisfied with the information on the internet as there weren't that many recipes or tutorials. The web was full of plugins or suggestions of dedicated software applications. Nothing really straight forward.

## Alternatives

As the use of the internet changed through out the years there came some alternatives to the RSS feed specifications. First one that is also `XML` based is called **Atom**.

_Atom_ was created as a response to the frozen *RSS 2.0.1* specification which was not satisfying to use anymore. It requires feed to specify some required fields so its easier for feed readers to parse and display.

On the other side there is **JSON Feed** specification which takes the modern approach of minimalism and even more broader use. It only specifies a little information as required and supports a case of **microblogging**.
The reason for `JSON` is that it is much commonly used in this information era than the `XML` format. I personally feel that way as well but the downside of this specification is that it is not very well known and used in consumer applications.

I am sure you can find much more information about [_Atom_](https://en.wikipedia.org/wiki/Atom_(Web_standard)) and [_JSON Feed_](https://jsonfeed.org/) on the internet.

## How to create a feed for sapper site

This blog is created with [**sapper**](https://sapper.svelte.dev/) so to generate a feed I needed to create following file structure:

```
  - src
    + components
    - routes
      + blog
      - feed
        _feed.js
        index.json.js
        index.xml.js
```

`_feed.js` contains logic for creating the feed and the `index.json.js` and `index.xml.js` are there to provide desired path for the feeds.
As you can tell I've built RSS feed as well as _JSON_ one. For feed construction I've used node library https://github.com/jpmonette/feed which provides a way to export constructed feed into multiple formats. That's why there is a `_feed.js` file which contains this logic that can be easily reused. Speaking of reusing logic. For the content I just refactored the blog listing logic and put the parser of the blog files into separate function and that was it.

I use `export` functionality of **sapper** so constructing the feed was not enough. As the `export` uses a crawler to find linked content on your site from the entry point, I had 2 options. Either add the path of the feeds to the export command to the AWS console or just link the feeds somewhere accessible in the site. Therefore I went for the generally better option and I've linked these feeds in the `footer` so now you can **subscribe** and receive updates with your feed consuming application of choice.

## Discoverability

One thing that I'd struggled with was finding out information about how to let know these consumer applications where they can find my feeds.
It took me some time to find but in the end it is just one `<meta>` tag in the `<head>` for each feed I offer.

```html
<link rel="alternate" type="application/rss+xml" title="RSS feed for latest posts" href="https://michalvanko.dev/feed.xml" />
<link rel="alternate" title="JSON feed for latest posts" type="application/json" href="https://michalvanko.dev/feed.json" />
```

I'm pretty happy about implementing these feeds as I've learned something new about the possibilities of subscribing to content that I can filter myself and is not controlled by anyone else. I am trying out some applications that I've found and perhaps make a review of those at some point.
