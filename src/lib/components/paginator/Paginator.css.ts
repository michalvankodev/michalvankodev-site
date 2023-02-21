import { sprinkles } from '$lib/styles/sprinkles.css'

export const listClass = sprinkles({
  listStyle: 'none',
  display: 'flex',
  justifyContent: 'center',
})

export const listItemClass = sprinkles({
  paddingX: '1x',
})

export const activePage = sprinkles({
  //fontStyle: 'italic',
  fontWeight: 'bold',
  paddingX: '2x',
})

export const pageLinkClass = sprinkles({
  paddingX: '1x',
})
