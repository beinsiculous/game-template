# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo run                       # play the game
cargo run --features editor     # run the game inside the engine's scene editor
cargo test                      # the headless suite
cargo test --features editor    # adds the four tests that instantiate the scene
cargo clippy --all-targets      # must be clean, with and without the feature
```

The game depends on the `insiculous_2d` engine by relative path
(`../../insiculous_2d/crates/engine_core`); both checkouts must sit side by side or
nothing builds. Engine crates used: `engine_core` (always) and `editor_integration`
(only behind the `editor` feature, and only for its headless texture resolver in tests).

## Architecture

**The gameplay is data, not Rust.** `assets/scenes/main.scene.ron` spawns everything and
`assets/scripts/{player,coin}.rhai` moves it; `src/` only loads the scene, steps the
engine's script runner, and puts a menu and a coin counter around the result. That is the
whole design: a project exported from the Web Playground unzips over this tree and runs,
because nothing in the crate knows a scene's name or an entity's job.

**Nothing names a scene file.** `spawning::spawn_scene` loads whatever
`SceneLoader::first_scene_in(&base.join("scenes"))` returns — the first `.ron` in sorted
order. An export carries its own scene name, so a hard-coded `main.scene.ron` would crash
at `init` on the very drop-in the README promises.

**The runner is stepped by the game, around the game's own physics step.**
`gameplay::update_playing` is `docs/SCRIPTING.md` § "Running scripts outside the editor"
verbatim: `early_update` before `physics.update`, then `update` after it with the frame's
drained collision events. Swap that order and a script reacts to last frame's contacts; skip
it entirely and the scripts never run, silently.

**Scripts own the state; Rust reads it.** The coin count lives on the engine's `Blackboard`
under `"coins"`, written by `coin.rhai`. `gameplay::current_coins` reads it and treats an
absent key as zero, so an exported project that has no coin at all runs without complaint.
`Blackboard` and `ScriptValue` come from `engine_core::prelude` — a game talks to
`engine_core` and nothing else.

**The coin's body type is load-bearing.** Rapier reports no intersection between two
non-dynamic bodies, so the Coin is a `Dynamic` body with `gravity_scale: 0.0` carrying a
sensor collider, touched by the `Kinematic` Player. A kinematic sensor would fire nothing
and the count would never move; `a_player_sitting_on_the_coin_banks_it` in
`src/scene_tests.rs` is the guard.

**Coordinate and scale conventions (the main trap):** world origin is screen center, the
window is 800×600, the renderer multiplies `Transform2D.scale` by `RENDER_UNIT = 80.0` to
get pixel size, and **collider shapes use absolute pixels and ignore scale entirely**.
Sprites and colliders are sized through different paths and can silently diverge.

**Chaos modes** (Normal / Insane / Ridiculous / Insiculous) all exist, and the only thing
that differs between them is the particle multiplier the engine's `ChaosTheme` carries.
Giving each mode a meaning of its own is the README's first exercise — and the line where a
template stops being a template.

## What to keep when you fork this

Rename the package, the config title, the four constants in `src/web_entry.rs`,
`project.ron`'s slug and title, and the save file names. Keep the scene discovery and the
runner block: those are what make the browser and the desktop the same game.
