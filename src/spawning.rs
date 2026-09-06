//! The only entity creation this crate does: it loads the scene.
//!
//! A game whose gameplay is a scene and its scripts has nothing else to
//! spawn — which is the point. Everything on screen is authored in
//! `assets/scenes/`, so the editor and the playground can both edit it.

use engine_core::prelude::*;

use crate::constants::SCENES_DIR;

/// Load and instantiate the first scene under `<base_path>/scenes`.
///
/// The file is discovered, never named: an exported project carries its own
/// scene name (`pong.scene.ron`, not `main.scene.ron`), and a crate that
/// hard-coded one would fail at `init` on the very drop-in its README
/// promises.
pub(crate) fn spawn_scene(
    world: &mut World,
    assets: &mut AssetManager,
    base_path: &str,
) -> Result<SceneInstance, SceneLoadError> {
    let scenes_dir = std::path::Path::new(base_path).join(SCENES_DIR);
    let scene_path = SceneLoader::first_scene_in(&scenes_dir)
        .ok_or_else(|| {
            SceneLoadError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("no .ron scene under {}", scenes_dir.display()),
            ))
        })?;
    let data = SceneLoader::load_from_file(&scene_path)?;
    SceneLoader::instantiate(&data, world, assets)
}

/// The physics the scene declares, built the way the editor's project host
/// builds it for Play, so a scene that says `gravity: (0.0, -980.0)` falls on
/// the desktop as it did in the browser.
///
/// A scene with no physics block is the one deliberate divergence: the
/// playground runs no physics at all for it (no bodies, no collision events),
/// while this game runs the top-down preset so the menu-and-grid shell still
/// has a world to step. A collision script needs a declared block to behave
/// the same in both.
pub(crate) fn physics_for(settings: Option<&PhysicsSettings>) -> PhysicsSystem {
    let config = match settings {
        Some(settings) => PhysicsConfig::new(Vec2::new(settings.gravity.0, settings.gravity.1))
            .with_scale(settings.pixels_per_meter),
        None => PhysicsConfig::top_down(),
    };
    PhysicsSystem::with_config(config)
}
