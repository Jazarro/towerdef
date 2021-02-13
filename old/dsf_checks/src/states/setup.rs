use amethyst::ecs::{prelude::World, Entities, Join, ReadStorage};

use crate::components::*;
use dsf_core::levels::load_level;
use dsf_core::utility::files::get_assets_dir;

pub fn setup_test(test: MovementTest, world: &mut World) {
    clear_previous_test(world);
    load_level_from_file(&test, world);
}

fn clear_previous_test(world: &mut World) {
    world.exec(
        |(entities, tags): (Entities, ReadStorage<MovementTestScopeTag>)| {
            for (entity, _) in (&entities, &tags).join() {
                entities
                    .delete(entity)
                    .expect("Failed to clear entities belonging to the previous test.");
            }
        },
    );
}

fn load_level_from_file(test: &MovementTest, world: &mut World) {
    let file_name = match test {
        MovementTest::Jump2Wide => "jump_2_wide.ron",
        MovementTest::Jump4Wide => "jump_4_wide.ron",
    };
    let level_file = get_assets_dir().join("tests/").join(file_name);
    load_level(&level_file, world).expect("Failed to load level!");
}
