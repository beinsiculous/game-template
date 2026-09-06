//! All on-screen text: the menu screens and the in-game status line.

use engine_core::prelude::*;

use crate::achievements::DISPLAY_SECTIONS;
use crate::menu::{achievements_panel, chaos_panel, title_panel, TITLE_ITEMS};
use crate::types::{GameState, TemplateGame};

impl TemplateGame {
    fn menu_style(&self) -> MenuStyle {
        MenuStyle::from_theme(&self.current_theme())
    }

    pub(crate) fn draw_ui(&self, ctx: &mut GameContext) {
        match &self.state {
            GameState::TitleScreen { selection } => self.draw_title(ctx, *selection),
            GameState::ChaosSelect { selection } => self.draw_chaos(ctx, *selection),
            GameState::Achievements => self.draw_achievements(ctx),
            _ => self.draw_status(ctx),
        }
    }

    fn draw_title(&self, ctx: &mut GameContext, selection: u8) {
        let style = self.menu_style();
        let panel = title_panel("Game Template", ctx.window_size);
        let mut y = panel.begin(ctx.ui, &style);
        for (row, item) in TITLE_ITEMS.iter().enumerate() {
            y = panel.item(ctx.ui, y, item, row as u8 == selection, &style);
        }
        panel.hint(ctx.ui, "Arrows/WASD move  ·  Enter confirms", &style);
    }

    fn draw_chaos(&self, ctx: &mut GameContext, selection: u8) {
        let style = self.menu_style();
        let panel = chaos_panel("Chaos Mode", ctx.window_size);
        let mut y = panel.begin(ctx.ui, &style);
        for (row, mode) in ChaosMode::ALL.iter().enumerate() {
            y = panel.item(ctx.ui, y, mode.label(), row as u8 == selection, &style);
        }
        panel.hint(ctx.ui, "Escape goes back", &style);
    }

    fn draw_achievements(&self, ctx: &mut GameContext) {
        let style = self.menu_style();
        let panel = achievements_panel("Achievements", ctx.window_size);
        let mut y = panel.begin(ctx.ui, &style);
        for (header, ids) in DISPLAY_SECTIONS {
            y = panel.line(ctx.ui, y, header, &style);
            for id in *ids {
                let Some(achievement) = ctx.achievements.get(id) else { continue };
                let mark = if ctx.achievements.is_unlocked(id) { "[x]" } else { "[ ]" };
                let row = format!("{mark} {} — {}", achievement.name, achievement.description);
                y = panel.line(ctx.ui, y, &row, &style);
            }
        }
        panel.hint(ctx.ui, "Escape goes back", &style);
    }

    fn draw_status(&self, ctx: &mut GameContext) {
        let center_x = ctx.window_size.x / 2.0;
        let coins = crate::gameplay::current_coins(ctx.world);
        // y is the text BASELINE: the top wall spans the first 20 screen
        // pixels, so keep the glyphs clear of it.
        ctx.ui.label_centered(&format!("Coins: {coins}"), Vec2::new(center_x, 42.0));

        let theme = self.current_theme();
        if let Some(banner) = theme.banner_text {
            let color = Color::new(
                theme.banner_color.x,
                theme.banner_color.y,
                theme.banner_color.z,
                theme.banner_color.w,
            );
            ctx.ui.label_centered_styled(
                banner,
                Vec2::new(center_x, ctx.window_size.y - 32.0),
                color,
                16.0,
            );
        }

        if self.state == GameState::Paused {
            let style = self.menu_style();
            let panel = MenuPanel::new("Paused", Vec2::new(center_x, ctx.window_size.y / 2.0), 320.0, 1);
            let y = panel.begin(ctx.ui, &style);
            panel.line(ctx.ui, y, "Escape resumes", &style);
        }
    }
}
