use bevy::prelude::*;

// TODO:
// - keyboard movement
// - create ball
// - ball movement
// - ball collision with walls
// - implement game_state
// - change score when ball hits wall

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(-920.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 100.0)),
            ..Default::default()
        })
        .id();
}

fn paddle_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.just_pressed(KeyCode::Down) {
            println!("hello");
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            println!("hello");
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.y += time.delta_seconds() * direction;
        // bound the paddle within the walls
        // translation.x = translation.y.min(380.0).max(-380.0);
    }
}
