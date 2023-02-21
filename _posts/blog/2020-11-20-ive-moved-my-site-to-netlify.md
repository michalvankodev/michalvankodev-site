---
layout: blog
title: I've moved my site to Netlify
segments:
  - blog
published: true
date: 2020-11-20T20:35:05.230Z
tags:
  - News
  - Development
  - Serverless
---
## Prelude

So I've been trying to set access to my CMS for a long time now. The problem was that in order to have CMS accessible through my domain, it **requires an _OAuth_ server**.
I've figured out that creating a custom **serverless function** which will handle the authentication with _GitHub_ would be sufficient.
So I've took some inspiration from the 2 repositories linked in [_Netlify CMS_ documentation](https://www.netlifycms.org/docs/external-oauth-clients/):

- [netlify-serverless-oauth2-backend](https://github.com/marksteele/netlify-serverless-oauth2-backend)
- [netlify-cms-github-oauth-provider](https://github.com/vencax/netlify-cms-github-oauth-provider)

The reason because I link both of these was that I've used the _almighty AWS Amplify_ CLI to create an API for me, which has bootstrapped an [_express_](http://expressjs.com/) application for me. This turned out to be way easier then to make 4 independent _functions_ as _Amplify_ is not providing a different way of creating simple shared repository of multiple functions like [serverless-framework](https://www.serverless.com/) does.

At one point as I was struggling with the _Amplify_ CLI it **managed to delete** whole API once as I was trying to synchronize my changes with the cloud. But somewhere in the development process of the _Amplify framework_ they made some changes which had made my previously used backend environments unusable. I wasn't able to find any migration information about these changes or anything. I'd found some GitHub issues about similar problems which haven't been resolved in any user friendly manners. So I've decided to just **re-create these environments**. I was lucky that I haven't had anything important published in these. Also my IDE had saved me and I was able to **restore deleted files from cache**.

## Turning point

After several attempts to publish the API on the AWS I figured out that to be able to use this API I have to configure a custom domain for it because _AWS API Gateway_ appends `/${stage}` to every URL that it is published. This wouldn't be so bad for any API but I was trying to create a **OAuth authentication** for _Netlify CMS_ which required the [`base_name` to be equal to the `origin`](https://github.com/netlify/netlify-cms/blob/aded9d7c24f40f6fa6a159ea764c1e4fccbe82c3/packages/netlify-cms-lib-auth/src/netlify-auth.js#L44) of the API. I was trying to configure custom domain name for this API then. Thinking that _Amplify_ will be able to help me, but this isn't a concern for its use case yet. So I've managed to create a certificate for the domain first. Waited half an hour for validation. And then finally I've got to the point that to be able to configure that custom domain name for this API I will have to configure _AWS Route58_, which is their DNS configuration service, to manage my entire domain which I haven't set up yet as I didn't think it would be necessary. My site has been working correctly just with `CNAME` configuration just right.

That was my the moment I've decided to **quit AWS completely**. This was after 2 days configuring and debugging a very simple think such as creating a simple API with 4 routes.

## Alternatives

I've done some research and was looking between 2 main contenders to host my precious website:

- [Netlify](https://www.netlify.com/)
- [Vercel](https://vercel.com/)

I haven't found a big decider feature in any of these so I just went with the one I already had an account set up - _Netlify_.
I was able to **configure whole deployment** of my website **in 8 minutes**.
The instructions where so clear and simple that I've also **transfered my DNS nameserver** too. I didn't have to search in 10 different services and documentations. I've just went with the flow. Didn't had to change a single line of code. I've just removed the `amplify` folder and I don't want to see any of the AWS docs ever again.

It got me so pumped that I've immediately logged in to the CMS (finally) through my own domain and written this article.
