//! Headless tests over the assets, which is where this game's behaviour lives.
//!
//! The base is `CARGO_MANIFEST_DIR`-anchored because the engine's games gate
//! runs `cargo test` from the engine root, where a relative base finds
//! nothing.
//!
//! The two tests that instantiate the scene need a texture resolver, and the
//! headless one lives in `editor_integration` — so they run behind the
//! `editor` feature, which the games gate exercises.

use engine_core::prelude::*;

fn asset_base() -> String {
    format!("{}/assets", env!("CARGO_MANIFEST_DIR"))
}

fn scene_path() -> std::path::PathBuf {
    let scenes = std::path::Path::new(&asset_base()).join("scenes");
    SceneLoader::first_scene_in(&scenes).expect("the crate ships a scene")
}

#[test]
fn the_shipped_scene_is_found_by_discovery_and_parses() {
    // Discovery, not a hard-coded name: this is the call an exported project
    // with its own scene name relies on.
    let path = scene_path();
    let data = SceneLoader::load_from_file(&path).expect("the scene parses");
    assert_eq!(data.name, "Game Template");
}

#[test]
fn every_shipped_script_passes_the_engine_check() {
    let scripts_dir = std::path::Path::new(&asset_base()).join("scripts");
    let mut checked = 0;
    for entry in std::fs::read_dir(&scripts_dir).expect("scripts dir exists") {
        let path = entry.expect("readable entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rhai") {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("script is readable");
        engine_core::scripting::check_source(&source)
            .unwrap_or_else(|error| panic!("{} is rejected: {error}", path.display()));
        checked += 1;
    }
    assert_eq!(checked, 2, "player.rhai and coin.rhai are the shipped scripts");
}

#[cfg(feature = "editor")]
#[test]
fn the_scene_instantiates_the_entities_the_readme_lists() {
    use editor_integration::HeadlessAssets;
    use engine_core::prelude::World;

    let data = SceneLoader::load_from_file(scene_path()).expect("the scene parses");
    let mut world = World::new();
    let mut assets = HeadlessAssets::new();
    let instance =
        SceneLoader::instantiate(&data, &mut world, &mut assets).expect("the scene instantiates");

    let mut names: Vec<&str> = instance.named_entities.keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "Background",
            "Bottom Wall",
            "Camera",
            "Coin",
            "Left Wall",
            "Player",
            "Right Wall",
            "Top Wall",
        ]
    );
    assert!(instance.load_warnings.is_empty(), "{:?}", instance.load_warnings);
}

#[cfg(feature = "editor")]
#[test]
fn ten_frames_of_the_runner_leave_the_player_in_bounds_and_the_scripts_quiet() {
    use editor_integration::HeadlessAssets;
    use engine_core::prelude::World;
    use engine_core::ScriptRunner;

    let data = SceneLoader::load_from_file(scene_path()).expect("the scene parses");
    let mut world = World::new();
    let mut assets = HeadlessAssets::new();
    let instance =
        SceneLoader::instantiate(&data, &mut world, &mut assets).expect("the scene instantiates");
    let player = instance.named_entities["Player"];

    let mut runner = ScriptRunner::new();
    runner.reset(&mut world, &asset_base());

    let mut physics = crate::spawning::physics_for(instance.physics.as_ref());
    // `InputHandler` is not in the prelude and the template depends on
    // `engine_core` alone; inference from the runner's signature names it.
    let input = Default::default();
    let players = InputSettings::default();
    let delta_time = 1.0 / 60.0;

    for _ in 0..10 {
        runner.early_update(&mut world, &input, &players, delta_time, Some(&mut physics));
        physics.update(&mut world, delta_time);
        let collisions = physics.take_collision_events();
        runner.update(
            &mut world,
            &input,
            &players,
            delta_time,
            &collisions,
            Some(&mut physics),
        );
    }

    let position = world.get::<Transform2D>(player).expect("the player has a transform").position;
    assert!(position.x.abs() <= 370.0, "player left its x bound: {position:?}");
    assert!(position.y.abs() <= 270.0, "player left its y bound: {position:?}");
    assert!(runner.errors().is_empty(), "{:?}", runner.errors());
}

/// The coin's body type is load-bearing, not decoration: rapier reports no
/// intersection between two non-dynamic bodies, so a kinematic sensor coin
/// touched by the kinematic Player would fire nothing and the count would
/// never move. This is the pair the whole template rests on.
#[cfg(feature = "editor")]
#[test]
fn a_player_sitting_on_the_coin_banks_it() {
    use editor_integration::HeadlessAssets;
    use engine_core::prelude::World;
    use engine_core::ScriptRunner;

    let data = SceneLoader::load_from_file(scene_path()).expect("the scene parses");
    let mut world = World::new();
    let mut assets = HeadlessAssets::new();
    let instance =
        SceneLoader::instantiate(&data, &mut world, &mut assets).expect("the scene instantiates");
    let player = instance.named_entities["Player"];
    let coin = instance.named_entities["Coin"];

    let coin_position = world.get::<Transform2D>(coin).expect("the coin has a transform").position;
    world
        .get_mut::<Transform2D>(player)
        .expect("the player has a transform")
        .position = coin_position;

    let mut runner = ScriptRunner::new();
    runner.reset(&mut world, &asset_base());

    let mut physics = crate::spawning::physics_for(instance.physics.as_ref());
    let input = Default::default();
    let players = InputSettings::default();
    let delta_time = 1.0 / 60.0;

    for _ in 0..5 {
        runner.early_update(&mut world, &input, &players, delta_time, Some(&mut physics));
        physics.update(&mut world, delta_time);
        let collisions = physics.take_collision_events();
        runner.update(
            &mut world,
            &input,
            &players,
            delta_time,
            &collisions,
            Some(&mut physics),
        );
    }

    let blackboard = world.resource::<Blackboard>().expect("the runner installs a blackboard");
    assert_eq!(blackboard.get("coins"), Some(&ScriptValue::I32(1)));
    // Banked coin number 1 hops to spot 1, the opposite corner in x.
    let moved = world.get::<Transform2D>(coin).expect("the coin has a transform").position;
    assert_eq!(moved, Vec2::new(-200.0, 150.0));
    assert!(runner.errors().is_empty(), "{:?}", runner.errors());
}

/// An exported scene brings its own physics block, and the desktop must obey
/// it as the playground does: a scene that declares gravity falls here too.
/// The shipped scene declares a zero-gravity block, which is simulation-
/// equivalent to the top-down preset, so the sims above would stay green if
/// `init` kept a hard-coded preset; this test guards the helper, and the line
/// in `init` that installs its result is covered only by running the game.
#[test]
fn a_scene_that_declares_gravity_falls_on_the_desktop_too() {
    let declared = PhysicsSettings { gravity: (0.0, -980.0), pixels_per_meter: 100.0, timestep: 1.0 / 60.0 };
    let mut physics = crate::spawning::physics_for(Some(&declared));
    let mut world = World::new();
    let body = world.create_entity();
    world.add_component(&body, Transform2D::new(Vec2::ZERO)).expect("transform");
    world.add_component(&body, RigidBody::new_dynamic()).expect("body");
    world
        .add_component(&body, Collider::new(ColliderShape::Circle { radius: 5.0 }))
        .expect("collider");

    for _ in 0..30 {
        physics.update(&mut world, 1.0 / 60.0);
    }

    let fallen = world.get::<Transform2D>(body).expect("transform").position;
    assert!(fallen.y < -1.0, "a dynamic body under declared gravity fell: {fallen:?}");

    // And with no block at all, the preset is top-down: nothing falls.
    let mut still = crate::spawning::physics_for(None);
    let mut world = World::new();
    let body = world.create_entity();
    world.add_component(&body, Transform2D::new(Vec2::ZERO)).expect("transform");
    world.add_component(&body, RigidBody::new_dynamic()).expect("body");
    for _ in 0..30 {
        still.update(&mut world, 1.0 / 60.0);
    }
    let at_rest = world.get::<Transform2D>(body).expect("transform").position;
    assert_eq!(at_rest, Vec2::ZERO);
}
