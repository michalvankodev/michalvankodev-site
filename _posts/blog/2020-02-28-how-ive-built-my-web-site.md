---
layout: blog
title: How I've built my website
segments:
  - blog
published: true
date: 2020-02-28T16:00:55.791Z
thumbnail: /images/uploads/DSC01202.jpg
tags:
  - News
  - Development
  - Svelte
  - Programming
---

## Motivation

Creating my own website with blog was something I had in my mind from start of my professional career after I left school.
I had a lot of new experience with development which I wanted to elaborate on and save into a small library so I can take a look back on my thoughts how they evolve over time.
This was like 6 years ago. I had a successful first attempt at doing so.

I created a WordPress with the simplest theme I've found and wrote some articles. I've published it under a domain of one of the first startups I've been part of. I still have a backup of the _Wordpress_ database somewhere so I can export those articles here when I will feel like doing so. The blog haven't lived for long as the domain once expired and I was not satisfied with it enough to deploy it somewhere else.

For all those years I was trying to create it in my spare time (of which wasn't that much apparently). There were several attempts. One with _Angular_ when it was "the cool kid on the block". Another one with _cycle.js_ which was not that far from being done. I regret it now as it would be really satisfying to finish that one. I had created neat <abbr title="Server side rendering">SSR</abbr> layer which was not really difficult to accomplish with _cycle.js_ as it is reactive and it only required skipping first client render of _virtual-dom_ after page load. I'm still in love with _cycle.js_ but after _sapper_ was released I've found out of its ability to create a nice **static site** I wanted to try it out. I think that the approach of **compiling the source code** as classic client applications have been doing for many years makes a lot of sense on the internet as well. This is the one thing I'd really like to be able to accomplish with reactive frameworks like _cycle.js_.

## How I got my domain

There was the big boom of new available generic top-level domains released around 2013. More than 1000 new gTLDs have been made available. After that I didn't want a local _(.sk)_ or any common _(.com)_ domain for my personal site. I wanted **.dev** from the first time I saw it. It was very unfortunate decision for me, because the _.dev_ domain was bought entirely by _Google_ and was not available for public until 28th of February 2019. When I found out that this day will come, I was so excited that I'd set up an alarm for it. I was among the first who bought the domain right away after few minutes it was available.

The **michalvanko.dev** domain celebrates 1 year with the release of this blog post.

## Build setup

So I had a domain with no content published on it for few months. But I knew that I'd use it at least for my **portfolio** page. I was not rushing myself into building it right away. But the time came when had to do it as I wanted to change my employment as I was not satisfied with the one at that time.

While I was choosing technology stack I was making a decision between static site generators as I wanted to make it as less resource dependent as possible. This lead me to the _sapper_ framework as it supports building (exporting) static assets with **efficiency**. At first I've wrote the portfolio just in _HTML_ without need of any <abbr title="Content management system">CMS</abbr>. I wanted to build it really fast and deploy it with some automated pipeline just like <abbr title="Javascript, APIs, Markup">JAM</abbr> applications should be.

I've chosen to deploy it on _Amazon's S3_ as I have some experience with it and I knew that it won't cost me anything. I had a 1 year free tier still available for my account even that it was 6 years old as I haven't set up any payments yet. _Amazon_ made this neat tool called [_amplify_](https://aws.amazon.com/amplify/) which suits very well for my needs. It supports **git based deployment** so I can setup my site to build whenever I create or edit any article and it will be published right away with purged cache. I just had to setup configuration of build step with _sapper_ so the exported site is correctly uploaded to _S3_ and that was it.

Build configuration:

```yaml
version: 0.1
frontend:
  phases:
    preBuild:
      commands:
        - npm install
    build:
      commands:
        - npm run export
  artifacts:
    # IMPORTANT - Please verify your build output directory
    baseDirectory: __sapper__/export
    files:
      - '**/*'
  cache:
    paths:
      - node_modules/**/*
```

Mapping _amplified_ app to my domain was easy as well. I didn't even needed to leave _amplify_ console. They've built the setup of the domain right into it so I just had to copy the settings for verification into my domain name registrar and in few minutes it was all synced and done.

## Development experience

So when I had deployed a basic portfolio site I was able to find a new job with completely different terms which made me happier as I am much more independent. I've started to build the blog with the features that _sapper_ offers. I was doing some research around some <abbr title="Content management system">CMS</abbr> systems and tried some of them out. One that deserves the mention is [_strapi_](https://strapi.io/). It was really cool as I could create models for anything and if I was building an application which was not static I'd use it. Now I can only recommend you to look at it. I was looking for something like it for 5 years and it finally looks like there are now more options. While I am writing this article I have to mention [_Keystone.js_](https://www.keystonejs.com/) as one of those libraries that I was looking at 5 years ago and it looked promising but just wasn't 100% compatible with my requirements. At the end I just grabbed the most simple tool I've found and that is [_Netlify CMS_](https://www.netlifycms.org/). I was skeptic about it as one colleague had some experience with it and he was not enjoying it. But when I've tried it out and set it up for my needs it just met all of the requirements I have and it integrates with the deploy pipeline very well.

I've created some articles for development with [Lorem Markdownum](https://jaspervdj.be/lorem-markdownum/) so I can debug styles for every type of content without hassle.
Then I had to finally meet the power of the _sapper_ framework. I had to adapt my thoughts (find better word) about how to create an <abbr title="Application Programmer Interface">API</abbr> so it can be exported into static assets. One thing that makes me mad about it to this day is [_Typescript_](https://www.typescriptlang.org/) support. As I was writing back-end there was no help for node <abbr title="Application Programmer Interface">API</abbr> which I got so used to from my other projects I work on. I've definitely lost some unnecessary minutes debugging why some variables are not something I thought they should be.

I also had some trouble with finding library for parsing article content. I was looking at the [_netlify-gatsby_ starter template](https://github.com/netlify-templates/gatsby-starter-netlify-cms) and the name for the package is just not clearly searchable on the internet. At least I was struggling with it because search results for markdown parsing didn't contain or support anything for files of the type that _netlify CMS_ produces. But after some searching I found the library as [_front-matter_](https://github.com/jxson/front-matter). It turned out that I didn't know what I was searching for. When you read the documentation of it then it will make sense to you why you couldn't find it out. I needed an _YAML_ parser. After that when I got the files parsed I just parsed markdown body with the with the most popular [markdown parser _marked_](https://marked.js.org) which has really nice options.

## Netlify CMS

I didn't have any experience with [_Netlify CMS_](https://www.netlifycms.org/). Setup was very straight forward.

1. Create a static _HTML_ page with one script tag.
2. Setup _YAML_ configuration file with git repository mapping, models and input types.

That's it. Sometimes I've changed some properties in my models, but I haven't struggled with it.

For example this is the model of this blog post:

```yaml
- name: 'blog' # Used in routes, e.g., /admin/collections/blog
  label: 'Blog' # Used in the UI
  folder: '_posts/blog' # The path to the folder where the documents are stored
  create: true # Allow users to create new documents in this collection
  slug: '{{year}}-{{month}}-{{day}}-{{slug}}' # Filename template, e.g., YYYY-MM-DD-title.md
  fields: # The fields for each document, usually in front matter
    - { label: 'Layout', name: 'layout', widget: 'hidden', default: 'blog' }
    - { label: 'Title', name: 'title', widget: 'string' }
    - {
        label: 'Published',
        name: 'published',
        widget: 'boolean',
        default: true,
      }
    - { label: 'Publish Date', name: 'date', widget: 'datetime' }
    - {
        label: 'Featured Image',
        name: 'thumbnail',
        widget: 'image',
        required: false,
      }
    - {
        label: 'Tags',
        name: 'tags',
        widget: 'list',
        default: ['News'],
        required: false,
      }
    - { label: 'Body', name: 'body', widget: 'markdown' }
    - { label: 'Writers notes', name: 'notes', widget: 'markdown' }
```

Neat part of the _CMS_ are those widgets. In editor they will be presented by appropriate component as well as in the editor preview.
I am very satisfied with it and I recommend it.

## What's next

I've decided not to wait for perfect product and I want to release this blog as soon as possible. I will have same approach as with other products. Make a <abbr title="Minimum Viable Product">**MVP**</abbr> and then release features as they are done.
I've put some features into _Github Projects Board_. I will very likely make a redesign with experimental layout changes. I'd like to experiment with colors and make the blog look distinguishable and personal while maintaining accessibility.

There are also some things I want to do with [_Svelte_](https://svelte.dev). I haven't tried it's transition and animation modules. I'd definitely like to add some page transitions and animate menus.
