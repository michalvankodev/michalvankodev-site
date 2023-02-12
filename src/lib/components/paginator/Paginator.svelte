<script lang="ts">
  import {
    activePage,
    listClass,
    listItemClass,
    pageLinkClass,
  } from './Paginator.css'

  import { getPaginatorPages, createHref } from './paginatorUtils'

  export const Divider = 'divider'

  export let segment: string
  export let page: number
  export let pageSize: number
  export let totalCount: number
  export let filters: Record<string, string>

  $: paginatorPages = getPaginatorPages({ page, pageSize, totalCount })
</script>

<ul class={listClass}>
  {#if page !== 1}
    <li class="{listItemClass} ">
      <a class={pageLinkClass} href={createHref(segment, filters, page - 1)}
        >&lt;</a
      >
    </li>
  {/if}
  {#each paginatorPages as pageNumber}
    {#if pageNumber === Divider}
      <li class={listItemClass}>...</li>
    {:else if page === pageNumber}
      <li class="{listItemClass} {activePage}">{pageNumber}</li>
    {:else}
      <li class="{listItemClass} ">
        <a class={pageLinkClass} href={createHref(segment, filters, pageNumber)}
          >{pageNumber}</a
        >
      </li>
    {/if}
  {/each}
  {#if page !== paginatorPages.length}
    <li class="{listItemClass} ">
      <a class={pageLinkClass} href={createHref(segment, filters, page + 1)}
        >&gt;</a
      >
    </li>
  {/if}
</ul>
