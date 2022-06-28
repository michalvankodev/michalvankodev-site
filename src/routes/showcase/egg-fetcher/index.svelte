<h1>Egg-fetcher</h1>

<p>As mentioned in <a href="/blog/2022-06-26-our-attempt-at-rusty-game-jam-weekly-25-2022">Weekly #25-2022</a>, I've attended the <a href="https://itch.io/jam/rusty-jam-2">Rusty game jam #2</a> where we had a week to <strong>create a game with Rust</strong>.</p>

<p>I've teamed up with <a href="https://github.com/silen-z/">@silen-z</a>. We haven't been able to finish the game. We didn't even make the mechanics we were thinking of. But I'd like to show <strong>the incomplete version of Egg-fetcher</strong> anyway. As we built it with Rust and <a href="https://bevyengine.org/">bevy engine</a> we were able to <a href="https://github.com/septum/rusty_jam_bevy_template">reuse a template</a> that had a configured WASM build. Therefore is very easy to just present the game in the browser.</p>

<iframe src="/egg-fetcher/index.html" width="800" height="600"></iframe>

<p><strong>The only functional controls are arrows</strong>. We have built a collision system where the chickens should move out of the way of the player and his pet. The player is not able to move through the fences and so on. We wanted to <strong>create a puzzle</strong> where you would have to play fetch with your pet dog to control the chickens and simultaneously control player movement.</p>

<p>This was only my 3rd attempt at a Rust codebase and therefore I got pretty much always stuck at some problem with borrow-checker or lifetimes.
  I learned many things and I'd like to continue with Rust and use it more in my side-projects.</p>

<style>
 iframe {
  height: 720px;
  width: 1280px !important;
  max-width: 1280px !important;
}
</style>
