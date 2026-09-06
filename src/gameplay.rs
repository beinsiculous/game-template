//! The in-match update: step the script runner around the physics step, then
//! react to what the scripts wrote.
//!
//! The runner block below is `docs/SCRIPTING.md` § "Running scripts outside
//! the editor" verbatim. It is the whole reason a scene authored in the
//! playground behaves the same after it is unzipped over this crate: the
//! editor's project host steps the runner exactly this way.

use engine_core::prelude::*;

use crate::constants::COINS_KEY;
use crate::{achievements, effects};
use crate::types::TemplateGame;

impl TemplateGame {
    pub(crate) fn update_playing(&mut self, ctx: &mut GameContext) {
        // early_update runs BEFORE the step (kinematic targets), update AFTER
        // it with the frame's drained collisions. Swap the order and a script
        // reacts to last frame's contacts.
        ctx.scripts.early_update(
            ctx.world,
            ctx.input,
            ctx.players,
            ctx.delta_time,
            Some(&mut self.physics),
        );
        self.physics.update(ctx.world, ctx.delta_time);
        let collisions = self.physics.take_collision_events();
        ctx.scripts.update(
            ctx.world,
            ctx.input,
            ctx.players,
            ctx.delta_time,
            &collisions,
            Some(&mut self.physics),
        );

        self.react_to_coins(ctx);

        // Step and re-emit the grid after gameplay so it reacts to this
        // frame's movement.
        engine_core::grid::step_and_emit_grid(
            self.grid.as_mut(),
            ctx.world,
            ctx.lines,
            ctx.delta_time,
            false,
        );
    }

    /// Draw the frozen scene without advancing it, so a paused game still
    /// shows its grid under the overlay.
    pub(crate) fn update_paused(&mut self, ctx: &mut GameContext) {
        engine_core::grid::step_and_emit_grid(
            self.grid.as_mut(),
            ctx.world,
            ctx.lines,
            0.0,
            false,
        );
    }

    /// The template's own chrome over whatever scripts are loaded: read the
    /// coin count the scripts keep, and celebrate a change.
    ///
    /// The scripts own the number; Rust only reacts. A foreign export writes
    /// no `coins` key at all, and an absent key reads as zero — so this stays
    /// quiet instead of failing.
    fn react_to_coins(&mut self, ctx: &mut GameContext) {
        let coins = current_coins(ctx.world);
        if coins <= self.coins_seen {
            // A reload or an export without a coin can move the count back;
            // treat only forward movement as banking one.
            self.coins_seen = coins;
            return;
        }
        self.coins_seen = coins;

        achievements::unlock_for_coins(ctx.achievements, coins);

        let theme = self.current_theme();
        let position = self
            .coin
            .and_then(|coin| ctx.world.get::<Transform2D>(coin))
            .map(|transform| transform.position)
            .unwrap_or(Vec2::ZERO);
        let burst = effects::coin_burst(&theme, self.white_texture);
        ctx.particles.spawn_burst(position, &burst);
    }
}

/// What the scripts have banked, or zero when nothing writes the key.
pub(crate) fn current_coins(world: &World) -> i32 {
    match world.resource::<Blackboard>().and_then(|bb| bb.get(COINS_KEY)) {
        Some(ScriptValue::I32(count)) => *count,
        _ => 0,
    }
}
