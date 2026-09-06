//! Game Template — the smallest honest game in the Insiculous layout.
//!
//! The gameplay is `assets/scenes/` plus `assets/scripts/`, not Rust: this
//! crate loads whatever scene it finds, steps the engine's script runner
//! around the physics step, and puts a menu and a coin counter around the
//! result. That is what lets a project exported from the Web Playground be
//! unzipped over a clone of this repository and just run.
//!
//! The library owns the whole game so both entry points stay thin:
//! `main.rs` (native window, filesystem saves, optional editor) and
//! `web_entry.rs` (wasm-bindgen start: fetch assets, then the same
//! `run_game`).

mod achievements;
mod constants;
mod drawing;
mod effects;
mod gameplay;
mod menu;
mod spawning;
mod types;

#[cfg(test)]
mod scene_tests;

#[cfg(target_arch = "wasm32")]
mod web_entry;

use engine_core::prelude::*;

use constants::*;
use types::*;

pub use types::TemplateGame;

/// The shared `GameConfig` for every target. Entry points add their own
/// platform extras on top (native: save paths anchored to the game dir; web:
/// localStorage keys per the engine's `docs/WEB_SAVES.md` contract).
///
/// `asset_base` must be an ANCHORED base: native callers pass an absolute
/// path (`main.rs` derives it from `game_root!()` so the cwd never matters);
/// the web entry passes the deploy URL base. A bare relative path like
/// `"assets"` would silently resolve against the current working directory.
pub fn game_config(asset_base: &str) -> GameConfig {
    GameConfig::new("Game Template")
        .with_size(WIN_W as u32, WIN_H as u32)
        .with_clear_color(0.0, 0.0, 0.0, 1.0)
        .with_fps(60)
        .with_asset_base_path(asset_base)
}

impl Game for TemplateGame {
    fn register_achievements(&self, achievements: &mut AchievementManager, _strings: &Strings) {
        achievements::register_all(achievements);
    }

    fn init(&mut self, ctx: &mut GameContext) {
        // Resolve against the configured asset base so the same relative path
        // works natively (game dir) and on the web (VFS keys).
        let font_path = std::path::Path::new(ctx.assets.base_path()).join("fonts/font.ttf");
        if let Ok(font) = ctx.ui.load_font_file(&font_path.to_string_lossy()) {
            ctx.ui.set_default_font(font);
        }

        if let Ok(white) = ctx.assets.create_solid_color(1, 1, [255, 255, 255, 255]) {
            self.white_texture = white.id;
        }

        let base_path = ctx.assets.base_path().to_string();
        match spawning::spawn_scene(ctx.world, ctx.assets, &base_path) {
            Ok(instance) => {
                self.physics = spawning::physics_for(instance.physics.as_ref());
                self.coin = instance.get_entity("Coin");
                for warning in &instance.load_warnings {
                    log::warn!("scene loaded with a warning: {warning}");
                }
            }
            // A missing or broken scene is not a crash: the menu still draws,
            // and the editor's inspector is how you find out what went wrong.
            Err(error) => log::error!("no scene loaded: {error}"),
        }

        // Compile every `.rhai` the scene bound, once per session. A game that
        // never calls this runs no scripts, silently.
        ctx.scripts.reset(ctx.world, &base_path);

        self.grid = Some(default_playfield_grid(&self.current_theme()));
    }

    fn update(&mut self, ctx: &mut GameContext) {
        self.frame_count = self.frame_count.wrapping_add(1);

        match self.state.clone() {
            GameState::TitleScreen { selection } => self.update_title_input(ctx, selection),
            GameState::ChaosSelect { selection } => self.update_chaos_input(ctx, selection),
            GameState::Achievements => self.update_achievements_input(ctx),
            GameState::Paused => {
                self.update_paused_input(ctx);
                self.update_paused(ctx);
            }
            GameState::Playing => {
                if MenuInput::read(ctx.input).back {
                    self.state = GameState::Paused;
                    self.update_paused(ctx);
                } else {
                    self.update_playing(ctx);
                }
            }
        }

        self.draw_ui(ctx);
    }
}
