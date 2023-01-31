<script lang="ts">
  import svgSprite from '../svg/build/icons-sprite.svg'
  export let className: string
  export let name: string
</script>

<svg aria-hidden="true" class={className}>
  <use xlink:href={`${svgSprite}#${name}`} />
</svg>
