use bevy::prelude::*;

pub fn example() {
    App::new().add_plugins(DefaultPlugins).run();
}
/* */

fn my_data(){

}
#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

fn level_up(
    // We want to access the Xp and Health data:
    mut query: Query<(&mut Xp, &mut Health)>,
) {
    // process all relevant entities
    for (mut xp, mut health) in query.iter_mut() {
        if xp.0 > 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
        }
    }
}
/*
엔터티 구성 요소


*/

/// Marker for the player
#[derive(Component)]
struct Player;

/// Bundle to make it easy to spawn the player entity
/// with all the correct components:
#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    health: Health,
    xp: Xp,
    // including all the components from another bundle
    sprite: SpriteBundle,
}

fn spawn_player(
    // needed for safely creating/removing data in the ECS World
    // (anything done via Commands will be applied later)
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
) {
    // create a new entity with whatever components we want
    commands.spawn(PlayerBundle {
        marker: Player,
        health: Health {
            current: 100,
            max: 125,
        },
        xp: Xp(0),
        sprite: SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    });

    // Call .id() if you want to store the Entity ID of your new entity
    let my_entity = commands.spawn((/* ... */)).id();
}