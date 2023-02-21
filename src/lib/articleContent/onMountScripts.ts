declare global {
  interface Window {
    onMountScripts?: Array<() => void>
  }
}
export function runOnMountScripts() {
  window.onMountScripts?.forEach((fn) => {
    fn()
  })
}

