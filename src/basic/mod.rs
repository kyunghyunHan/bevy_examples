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