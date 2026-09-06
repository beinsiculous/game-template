use engine_core::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum GameState {
    TitleScreen { selection: u8 },
    ChaosSelect { selection: u8 },
    Achievements,
    Playing,
    Paused,
}

pub struct TemplateGame {
    pub(crate) state: GameState,
    pub(crate) chaos_mode: ChaosMode,
    pub(crate) physics: PhysicsSystem,
    /// Deforming spring-mass grid drawn under the scene's sprites. Built in
    /// `init()`, once the chaos mode is known (the grid color is themed).
    pub(crate) grid: Option<GridMesh>,
    pub(crate) frame_count: u32,
    /// The scene's `Coin`, when it has one — a foreign export usually does
    /// not, and then the burst falls back to the origin.
    pub(crate) coin: Option<EntityId>,
    /// White 1x1 texture, used by the particle bursts.
    pub(crate) white_texture: u32,
    /// Last coin total read off the blackboard. The scripts own the count;
    /// this is only what the Rust side has already reacted to.
    pub(crate) coins_seen: i32,
}

impl TemplateGame {
    /// Presentation tokens for the currently selected chaos mode.
    pub(crate) fn current_theme(&self) -> ChaosTheme {
        ChaosTheme::for_mode(self.chaos_mode)
    }
}

impl Default for TemplateGame {
    fn default() -> Self {
        Self {
            physics: PhysicsSystem::with_config(PhysicsConfig::top_down()),
            state: GameState::TitleScreen { selection: 0 },
            chaos_mode: ChaosMode::Normal,
            grid: None,
            frame_count: 0,
            coin: None,
            white_texture: 0,
            coins_seen: 0,
        }
    }
}
