/** @type {import('@sveltejs/kit').ParamMatcher} */
export function match(param: string) {
  console.debug('parma', param)
  return !['tags', 'page'].some((keyword) => param.startsWith(keyword))
}
