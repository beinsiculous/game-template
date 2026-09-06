//! The template's two achievements.
//!
//! Registered through `Game::register_achievements`, which the engine calls
//! before the window opens — that is what lets `--achievements-manifest`
//! export the list with no GPU. Names are literals rather than locale keys:
//! the template ships no locale table, and one string is not worth a second
//! system.

use engine_core::prelude::*;

pub(crate) const FIRST_COIN: &str = "first_coin";
pub(crate) const TEN_COINS: &str = "ten_coins";

/// Grouped display order for the achievements screen: the section header and
/// the ids under it.
pub(crate) const DISPLAY_SECTIONS: &[(&str, &[&str])] =
    &[("Coins", &[FIRST_COIN, TEN_COINS])];

pub(crate) fn register_all(mgr: &mut AchievementManager) {
    mgr.register(Achievement::new(
        FIRST_COIN,
        "First Coin".to_string(),
        "Bank a coin.".to_string(),
    ));
    mgr.register(Achievement::new(
        TEN_COINS,
        "Ten Coins".to_string(),
        "Bank ten coins in one session.".to_string(),
    ));
}

/// Unlock whatever `coins` has reached. Called from the coin count in
/// `gameplay`, so a session that banks ten in one go earns both.
pub(crate) fn unlock_for_coins(achievements: &mut AchievementManager, coins: i32) {
    if coins >= 1 {
        achievements.unlock(FIRST_COIN);
    }
    if coins >= 10 {
        achievements.unlock(TEN_COINS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_sections_cover_every_registered_achievement() {
        let mut mgr = AchievementManager::in_memory();
        register_all(&mut mgr);

        let shown: std::collections::HashSet<&str> = DISPLAY_SECTIONS
            .iter()
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect();

        for ach in mgr.all() {
            assert!(
                shown.contains(ach.id.as_str()),
                "{} registered but not in DISPLAY_SECTIONS",
                ach.id
            );
        }
        assert_eq!(shown.len(), mgr.total(), "DISPLAY_SECTIONS has duplicates or extras");
    }

    #[test]
    fn coins_unlock_at_one_and_at_ten() {
        let mut mgr = AchievementManager::in_memory();
        register_all(&mut mgr);

        unlock_for_coins(&mut mgr, 0);
        assert!(!mgr.is_unlocked(FIRST_COIN));

        unlock_for_coins(&mut mgr, 1);
        assert!(mgr.is_unlocked(FIRST_COIN));
        assert!(!mgr.is_unlocked(TEN_COINS));

        unlock_for_coins(&mut mgr, 10);
        assert!(mgr.is_unlocked(TEN_COINS));
    }
}
