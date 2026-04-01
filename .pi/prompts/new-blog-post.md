---
description: Create a new blog post with today's date and proper front matter. Takes a title/topic as argument.
---
Create a new blog post file for this project.

**Input:** $@ - The title/topic for the blog post

---

## Instructions

1. Generate a URL-friendly slug from the title:
   - Lowercase
   - Replace spaces and special characters with hyphens
   - Remove consecutive hyphens
   - Strip leading/trailing hyphens

2. Create the file at `_posts/blog/YYYY-MM-DD-{slug}.md` where YYYY-MM-DD is today's date

3. Use this front matter template:

```yaml
---
layout: blog
title: {Original Title}
segments:
  - blog
published: false
date: {YYYY-MM-DD}T12:00:00.000Z
thumbnail:
tags:
  - News
---

## {First section heading}

{Leave space for content}
```

4. **Important:** The date in the filename (YYYY-MM-DD) MUST match the date in the front matter's `date` field

5. Confirm the file was created with the full path

## Available Segments
- `blog` (default)
- `featured`
- `broadcasts`
- `cookbook`

## Common Tags
- News
- Development
- Programming
- Rust
- JavaScript
- TypeScript
- Keyboards
- Tutorial
- Personal
