# Game Template

A working game whose gameplay is a scene and two scripts rather than Rust — so a
project exported from the [Insiculous Web Playground](https://beinsiculous.com/playground/)
drops onto a clone of this repository and runs.

## Start here

The engine is a sibling directory, not a dependency you download: every Insiculous
game depends on it by relative path, and the engine is meant to be edited beside the
game you are building (find something missing, fix it in the engine, with tests).

```sh
git clone https://github.com/beinsiculous/insiculous_2d
mkdir games && git clone https://github.com/beinsiculous/game-template games/<your-game>
cd games/<your-game> && cargo run
```

The path dependency is `../../insiculous_2d/crates/engine_core`, so the layout above is
the one the `Cargo.toml` already expects.

## Run

```sh
cargo run                     # the game
cargo run --features editor   # the same game inside the scene editor
```

The web builds run from the **engine** root:

```sh
scripts/build_wasm.sh ../games/<your-game> <slug> --kind games
scripts/build_wasm.sh ../games/<your-game> <slug> --kind editor
```

## The layout

An exported project is a zip in this shape:

```
<slug>.zip
├── project.ron     # the ProjectManifest
├── README.md       # generated: the title and the docs URL
└── assets/         # scenes, .sheet.ron sidecars, scripts, images, sounds, fonts, locales
```

This repository is the same tree with a crate around it:

```
project.ron         # the same manifest, so the clone and the archive match
assets/
├── scenes/main.scene.ron
├── scripts/{player,coin}.rhai
├── images/ball_8px.png                # the Player and the Coin, tinted by the scene
└── fonts/font.ttf
src/                # the crate: menu, chaos modes, achievements, the coin count
```

It goes both ways.

**An export drops on the clone**, from the clone's root:

```sh
rm -rf assets/scenes assets/scripts && unzip -o <slug>.zip -x README.md -d .
```

`project.ron` and `assets/` are replaced and `cargo run` plays the export. Both halves are
load-bearing. The `rm` first: `unzip -o` adds to a directory, it does not empty it, and the
game loads whichever scene sorts first — the template's own `main.scene.ron` sorts ahead of
most names, so without the `rm` you would be playing the coin demo, and a later zip of the
clone would carry it back into the browser. The `-x`: every export carries a generated
`README.md`, and `unzip -o` would replace the one you are reading with it. The font under
`assets/fonts/` stays, so the template's text still draws.

**The clone goes back to the browser**, from the clone's root:

```sh
zip -r <slug>.zip project.ron assets
```

then **Import project** on `/playground/`. Zip those two paths and no more — the importer
refuses any other entry at the root.

Nothing here names a scene file: the crate loads whichever `.ron` sorts first under
`assets/scenes/`, which is why an export that calls its scene `pong.scene.ron` still runs.

## Make it yours

Rename, in this order:

1. `Cargo.toml`'s package name,
2. `game_config`'s title in `src/lib.rs`,
3. the four constants in `src/web_entry.rs` (two asset bases, the editor preferences slot,
   the two save keys),
4. `project.ron`'s `slug` and `title`,
5. the save file names in `src/main.rs`.

The menu, the chaos modes, the achievements and the coin count are the template's **Rust**,
and they are yours to replace: a foreign export's scripts run without any of them, because
the crate only loads the scene, steps the script runner, and reads the blackboard.

**First exercise:** give each chaos mode a meaning. All four modes exist here, but the only
thing that changes between them is the particle multiplier the engine's theme carries.
A per-game meaning is where a template stops and a game starts.

## Conventions

The house rules for a new game — module layout, chaos modes, achievements, the neon look,
headless tests — are in the engine's `.claude/skills/new-game/SKILL.md`.

## Docs

- [Web Playground](https://github.com/beinsiculous/insiculous_2d/blob/main/docs/WEB_PLAYGROUND.md) — the bundle, export and import
- [Scripting](https://github.com/beinsiculous/insiculous_2d/blob/main/docs/SCRIPTING.md) — the hooks, the param headers, and running the runner outside the editor
