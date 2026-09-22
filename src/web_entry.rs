//! Browser entry point: fetch all assets into the engine's VFS, then run the
//! exact same game `main.rs` runs natively.
//!
//! The site serves no bundle for the template — it has no game page, and needs
//! none. The constants below exist so `--kind games` and `--kind editor` both
//! build out of the box, and so the version-bump checklist is in front of the
//! reader on the day they rename this crate into their own game.
//!
//! Saves persist to localStorage: on wasm the engine treats `GameConfig`
//! save-path strings as localStorage keys (`engine_core::save_store`), so
//! achievements and input bindings survive reloads under the
//! `beinsiculous.games.game-template.*` keys from `docs/WEB_SAVES.md`.

use engine_core::web::{init_web_logging, preload_assets, set_boot_status};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(feature = "editor"))]
use engine_core::prelude::run_game;

#[cfg(feature = "editor")]
use editor_integration::{run_game_with_editor_opts, EditorRunOptions};
#[cfg(feature = "editor")]
use std::path::PathBuf;

/// Where the deployed build serves its assets from; also the VFS key base.
///
/// VERSION-BUMP CHECKLIST — these must all agree (a mismatch 404s every asset
/// at boot with a "not in vfs" message):
/// 1. this constant (`/games/<slug>/v<N>/assets`),
/// 2. `scripts/build_wasm.sh`'s `--version vN` (its output dir `dist/games/<slug>/v<N>/`),
/// 3. the site's `src/content/games/<slug>.md` `wasm:` path,
/// 4. the deployed dir `insiculous_web/public/games/<slug>/v<N>/`.
#[cfg(not(feature = "editor"))]
const ASSET_BASE: &str = "/games/game-template/v1/assets";

/// Where the editor bundle serves its assets from; also its VFS key base.
///
/// Its version is INDEPENDENT of the game's own: the two bundles deploy
/// separately. VERSION-BUMP CHECKLIST for this one:
/// 1. this constant (`/playground/<slug>/v<N>/assets`),
/// 2. `scripts/build_wasm.sh --kind editor`'s output dir,
/// 3. the site's `src/content/games/<slug>.md` `editor:` path,
/// 4. the deployed dir `insiculous_web/public/playground/<slug>/v<N>/`.
#[cfg(feature = "editor")]
const EDITOR_ASSET_BASE: &str = "/playground/game-template/v1/assets";

/// The editor's preferences slot for this game.
///
/// On wasm a preferences path is a localStorage key, and the editor's native
/// default (`editor_prefs.json`) would sit outside the site's key contract.
/// Each game gets its own slot because the preferences carry the camera.
#[cfg(feature = "editor")]
const EDITOR_PREFS_SLOT: &str = "beinsiculous.playground.game-template.editor_prefs";

/// The base the boot phase preloads — whichever bundle this build is.
#[cfg(not(feature = "editor"))]
const BOOT_ASSET_BASE: &str = ASSET_BASE;
#[cfg(feature = "editor")]
const BOOT_ASSET_BASE: &str = EDITOR_ASSET_BASE;

#[wasm_bindgen(start)]
pub fn start() {
    init_web_logging();
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = preload_assets(BOOT_ASSET_BASE).await {
            log::error!("asset preload failed: {e}");
            set_boot_status(&format!("Failed to load assets: {e}"));
            return;
        }
        if let Err(e) = run_configured_game() {
            log::error!("failed to start game: {e}");
            set_boot_status(&format!("Failed to start: {e}"));
        }
    });
}

#[cfg(not(feature = "editor"))]
fn run_configured_game() -> Result<(), engine_core::EngineError> {
    let config = crate::game_config(ASSET_BASE)
        .with_achievement_save_path("beinsiculous.games.game-template.achievements")
        .with_input_settings_path("beinsiculous.games.game-template.input");
    run_game(crate::TemplateGame::default(), config)
}

/// The editor session passes NO save paths: with none set the engine keeps
/// achievements, scores and bindings in memory and falls back to the default
/// input bindings, so nothing the site's boards read is written.
#[cfg(feature = "editor")]
fn run_configured_game() -> Result<(), engine_core::EngineError> {
    run_game_with_editor_opts(
        crate::TemplateGame::default(),
        crate::game_config(EDITOR_ASSET_BASE),
        EditorRunOptions {
            prefs_slot: Some(PathBuf::from(EDITOR_PREFS_SLOT)),
            ..Default::default()
        },
    )
}
