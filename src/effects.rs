//! Visual effect presets — particle configs.
//!
//! Centralizes the look of each event so tuning happens in one place. The
//! deforming grid uses the engine's `default_playfield_grid` preset directly.

use engine_core::prelude::*;

/// The burst thrown off a banked coin. Chaos reaches the template's look
/// through `particle_count_mult` and nothing else: giving each mode a meaning
/// of its own is the README's first exercise.
pub(crate) fn coin_burst(theme: &ChaosTheme, texture: u32) -> ParticleConfig {
    let count = (48.0 * theme.particle_count_mult).round() as usize;
    ParticleConfig::burst(count)
        .with_lifetime(0.3, 0.8)
        .with_speed(120.0, 380.0)
        .with_direction(Vec2::Y, std::f32::consts::PI) // full circle
        .with_color(
            Vec4::new(1.0, 0.85, 0.2, 1.0),
            Vec4::new(1.0, 0.85, 0.2, 0.0),
        )
        .with_scale(8.0, 0.5)
        .with_drag(1.8)
        .with_emissive(2.5)
        .with_texture(texture)
}
