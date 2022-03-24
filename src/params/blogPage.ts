/** @type {import('@sveltejs/kit').ParamMatcher} */
export function match(param: string) {
  console.log('parma', param)
  return !['tags', 'page'].some((keyword) => param.startsWith(keyword))
}
