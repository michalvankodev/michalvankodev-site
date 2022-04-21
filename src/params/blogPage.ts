/** @type {import('@sveltejs/kit').ParamMatcher} */
export function match(param: string) {
  return !['tags', 'page'].some((keyword) => param.startsWith(keyword))
}
