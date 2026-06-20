use nightshade_api::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run!(
        |world| {
            spawn_floor(world, 10.0);
            spawn_cube(world, vec3(0.0, 0.5, 0.0))
        },
        |world, cube| {
            rotate(world, *cube, Vec3::y(), delta_time(world));
        },
        |world, cube| {
            let height = 0.7 + (elapsed_seconds(world) * 2.0).sin() * 0.3;
            set_position(world, *cube, vec3(0.0, height, 0.0));
        },
        |world, cube| {
            let pulse = elapsed_seconds(world).sin() * 0.5 + 0.5;
            set_color(world, *cube, [pulse, 0.4, 1.0 - pulse, 1.0]);
        },
    )
}
