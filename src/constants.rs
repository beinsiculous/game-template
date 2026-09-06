pub(crate) const WIN_W: f32 = 800.0;
pub(crate) const WIN_H: f32 = 600.0;

/// Where `spawn_scene` looks, relative to the asset base. Whatever `.ron`
/// sorts first in here is the scene that runs — the crate names no file, so a
/// project exported from the playground drops in and runs under its own name.
pub(crate) const SCENES_DIR: &str = "scenes";

/// The blackboard key `assets/scripts/coin.rhai` writes. An export that has no
/// coin never writes it, and an absent key reads as zero.
pub(crate) const COINS_KEY: &str = "coins";
