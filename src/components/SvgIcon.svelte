<script>
  import svgSprite from '../svg/build/icons-sprite.svg'
  export let className
  export let name
</script>

<svg aria-hidden="true" role="img" class={className}>
  <use xlink:href={`${svgSprite}#${name}`} />
</svg>
