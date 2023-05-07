import { Slot, component$ } from '@builder.io/qwik'
import { mainContentClass } from './layout.css'

export default component$(() => {
  return (
    <div class="blog-site">
      {
        // <Navigation />
      }
      <main class={mainContentClass}>
        <Slot />
      </main>
      {
        // <Footer />
      }
    </div>
  )
})
