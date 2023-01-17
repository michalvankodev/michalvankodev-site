import type { PageLoad } from './$types'

export const load = (async () => {
  const res = await fetch('/portfolio.json')
  const content = await res.json()
  return {
    content,
  }
}) satisfies PageLoad
