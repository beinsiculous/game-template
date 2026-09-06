//! Menu screens: navigation and selection. Drawing lives in `drawing`.

use engine_core::prelude::*;

use crate::types::{GameState, TemplateGame};

/// The title menu's rows, in order. Both halves of the menu derive from this
/// list — row count, navigation bound, confirm dispatch, drawn labels — so
/// adding a row is a one-list change.
pub(crate) const TITLE_ITEMS: &[&str] = &["Play", "Chaos Mode", "Achievements", "Exit"];

/// Panel layouts shared by the input half (mouse hit-testing here) and the
/// drawing half — the geometry must match or clicks land beside the rows.
pub(crate) fn title_panel(title: &str, window_size: Vec2) -> MenuPanel {
    MenuPanel::new(title, window_size / 2.0, 360.0, TITLE_ITEMS.len())
}

pub(crate) fn chaos_panel(title: &str, window_size: Vec2) -> MenuPanel {
    MenuPanel::new(title, window_size / 2.0, 400.0, ChaosMode::ALL.len())
}

pub(crate) fn achievements_panel(title: &str, window_size: Vec2) -> MenuPanel {
    MenuPanel::new(title, window_size / 2.0, window_size.x - 160.0, 4)
}

impl TemplateGame {
    pub(crate) fn update_title_input(&mut self, ctx: &mut GameContext, selection: u8) {
        let input = MenuInput::read(ctx.input);
        let mouse = title_panel("", ctx.window_size).mouse_select(ctx.input);
        let selection = mouse.hovered.unwrap_or(selection);
        let mut selection = input.navigate(selection, TITLE_ITEMS.len() as u8);
        if let Some(row) = mouse.clicked {
            selection = row;
        }
        self.state = GameState::TitleScreen { selection };

        if input.confirm || mouse.clicked.is_some() {
            match selection {
                0 => self.state = GameState::Playing,
                1 => self.state = GameState::ChaosSelect { selection: 0 },
                2 => self.state = GameState::Achievements,
                _ => ctx.request_exit(),
            }
        }
    }

    pub(crate) fn update_chaos_input(&mut self, ctx: &mut GameContext, selection: u8) {
        let input = MenuInput::read(ctx.input);
        let mouse = chaos_panel("", ctx.window_size).mouse_select(ctx.input);
        let count = ChaosMode::ALL.len() as u8;
        let selection = mouse.hovered.unwrap_or(selection);
        let mut selection = input.navigate(selection, count);
        if let Some(row) = mouse.clicked {
            selection = row;
        }
        self.state = GameState::ChaosSelect { selection };

        if input.back {
            self.state = GameState::TitleScreen { selection: 1 };
        } else if input.confirm || mouse.clicked.is_some() {
            self.chaos_mode = ChaosMode::ALL[selection as usize];
            // Mirror the selection into the engine context so anything reading
            // ctx.chaos_mode agrees with ours; the engine persists it.
            ctx.chaos_mode = self.chaos_mode;
            self.grid = Some(default_playfield_grid(&self.current_theme()));
            self.state = GameState::TitleScreen { selection: 1 };
        }
    }

    pub(crate) fn update_achievements_input(&mut self, ctx: &mut GameContext) {
        let input = MenuInput::read(ctx.input);
        let dismissed = achievements_panel("", ctx.window_size).clicked_inside(ctx.input);
        if input.back || input.confirm || dismissed {
            self.state = GameState::TitleScreen { selection: 2 };
        }
    }

    /// Escape pauses a running game and resumes a paused one. The pause is a
    /// state, not an engine `PauseMenu`: the template has no surface to hang
    /// one on, and the state match is the thing a reader is here to copy.
    pub(crate) fn update_paused_input(&mut self, ctx: &mut GameContext) {
        let input = MenuInput::read(ctx.input);
        if input.back {
            self.state = GameState::Playing;
        }
    }
}
