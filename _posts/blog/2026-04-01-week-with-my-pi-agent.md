---
layout: blog
title: Week with my-pi-agent
segments:
  - blog
  - featured
published: true
date: 2026-04-01T12:00:00.000Z
thumbnail: /images/uploads/pi-logo.svg
tags:
  - News
  - AI
  - Agents
  - Development
---

Last week I've set a goal for myself to go 100% without writing a single line of code manually. I've been trying agentic workflows for a year now. My first agentic tool I've tried was [aider](https://aider.chat/) and it was pretty cool experience at that time. For the last few months I've been using [OpenCode](https://opencode.ai/) but I haven't been able to let it control 100% of my results. Mainly because it hasn't been able to earn my trust. I've been watching different tools for a long time and decided that it was time to try out something minimal. Something that I can make work for me. Make it work with my workflows and tools I use during work.

## Meet Pi

<!-- ![Pi agent logo](/images/uploads/pi-logo.svg 'Pi agent logo') -->
<figure>
  <img src="/images/uploads/pi-logo.svg" alt="Pi agent logo" class="max-w-[280px] h-[280px] bg-black">
</figure>

[Pi](https://pi.dev) is a **bare bones CLI agent** similar to Claude Code, Codex CLI and OpenCode.
Pi itself is very simple. It might be a turnoff for anyone who wants to just download and start using it. You have to spend some time tinkering it.
There is a lot of [packages](https://pi.dev/packages), that can bring the functionality of other CLIs to Pi.
The community around Pi is very fresh and thriving. We are still in the early adopters era of agentic programming. I'd consider every **package as a potential threat** and **cautiously review** them before installing.

I discovered Pi when [OpenClaw](https://openclaw.ai/) was released. I was curious about what is powering _OpenClaw_. Back then I wasn't really interested in Pi.
But after few weeks with [OpenCode](https://opencode.ai/), I wasn't feeling like the tool is working with my workflow.
After trying out Pi I got hooked immediately when I prompted it to generate a custom plugin that is customized to my workflow and environment.
I use a nerdish Desktop environment:
- [Fedora Linux](https://fedoraproject.org/)
- [Noctalia shell](https://noctalia.dev/)
- [Niri Window manager](https://niri-wm.github.io/niri/) (special kind of tiling window manager)
- [fish](https://fishshell.com/) as a shell
- [ZelliJ](https://zellij.dev/) as a terminal multiplexer

I don't know anyone who uses the same tools I do.

Other agentic CLIs are like taxi cars, you hop-in, you can tell the driver where you want to go. But it is out of your hands how you get there. You are often restricted in the capabilities of the CLI.
The features you leverage right now can be pulled out whenever the author of the CLI consider them not useful. With Pi you can **inspect every single line of code**. Why, and what gets executed and when.
With Pi, you build your own ride. Do you need to just get some mail from post office? You build a bicycle. Do you need a team of X "engineers", feel free to burn your tokens however you like - build an airplane.
Want to go the to moon? Build yourself a rocketship. There are no limits.

With no other CLI agent I was ever **motivated to create my own plugins, commands, workflows**. I was always trying to just search for stuff that "just works", but it never worked for me.

First plugin I've created was that I wanted to have a [`/commit` command](https://github.com/michalvankodev/my-pi-agent/blob/main/agent/extensions/commit.ts).

- This command should not do `git commit` for me.
- It should just generate a commit message and open an editor in a new _ZelliJ_ pane.
- I can not only review but edit if I am not satisfied with the output. I can even cancel the commit if I opt to.

One, two prompts and voilà - I have my first custom plugin. Nothing ground-breaking but it is working just how I like it.

![My Pi says Hi!](/images/uploads/pi-screenshot.png 'My Pi says Hi!')

## What works for me

In my experience a simple approach turned out to have the best results considering getting stuff done, cheaper and reliable.
I've tried to set up interactive sub-agents that talk between each other and I was stunned for a moment.
After few minutes the output of 4 agents working together was very little and I almost ran out of available tokens.
Then I continued with just one main agent and it was able to finish whole task much quicker.
The main workflow for me (for backend tasks) is:
1. Prompt agent with what are we going to do. "Gather information about existing code". Write a specification file and split work into tasks.
We are going to update the specification after each task gets done.
2. [`/clarify`](https://github.com/michalvankodev/my-pi-agent/blob/main/agent/prompts/clarify.md) This is the **most powerful prompt** in my workflow. It asks agent to ask clarifying questions. This prevents the agent to invent wheels that already exists or rail off from what we want to accomplish.
3. Start working on tasks one by one

When the context-window gets too big, there is a decision to make. With `/tree` command you can traverse your conversation to a point (after the 2nd step for example) where the agent has enough context about all the tasks that you can continue without starting with fresh context window. Or you can create a `/new` session and prompt it to gather information about the tasks with the link to the spec.

`/tree` is very useful feature. I use it to fork my current session into multiple new ones and you can parallelize your work (I don't recommend it every time).

Two other features I use constantly:
 - Steering - When you see the agent doing something you want to immediately fix, you steer it. Your message
 arrives as soon as the current tool call finishes.
 - Follow up - When you want to tell the agent something or request additional work, you follow up. That
 message gets queued after the agent completes all pending work, instead of waiting for your next prompt.

There's still more to learn. This is just the experience I've had with Pi during the last week.

For front-end task I'd choose a different workflow. I'd incorporate `agent-browser` into the loop. Having constant feedback loop is very powerful and allows the agent to run for longer time (do more without constant supervision)

Another very good use experience I had was with debugging memory issues. This is a task that if you don't do very often during development, but from time to time it happens that unexpected errors happen due to different resource availability on production systems. I've said to the agent what happens and what I think was the issue (I was way way off).
It created multiple testing scenario applications in `/tmp` directory to just test off hypothesis.
He dug out every single call and investigated deep hierarchy of code deep down to every single dependency in the chain.
We haven't been able to find the issue there. So I've used [`heaptrack`](https://github.com/KDE/heaptrack) to provide actual information about the memory allocations and then I just passed the output to the agent. It just came out with the actual issue and provided fix in few seconds.  

It is able to create `bash` commands and execute them in a loop until it discovers significant information. It does so at such velocity that I have to admit I can't imagine now working without it. Once you experience this kind of boost it's unthinkable to go back.
