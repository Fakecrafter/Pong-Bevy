#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

// TODO:
// - ball collision with walls
// - ball collision with paddles
// - implement game_state
// - change score when ball hits wall
// - put everything in modules
// TODO's for later:
// - Window Descriptor
// - make values responsive to size of window

struct GameState {
    left: u32,
    right: u32,
}

struct Ball;
struct Paddle;

enum PaddleSize {
    Right,
    Left,
}

struct BallDirection(i32, i32);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement.system())
        .add_system(ballmovement.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // paddle 1
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(-920.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 100.0)),
            ..Default::default()
        })
        .insert(PaddleSize::Left)
        .insert(Paddle)
        .id();

    // paddle 2
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(920.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 100.0)),
            ..Default::default()
        })
        .insert(PaddleSize::Right)
        .insert(Paddle)
        .id();

    // ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(BallDirection(1, 1))
        .id();
}

fn paddle_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &PaddleSize)>,
) {
    for (mut transform, paddle) in query.iter_mut() {
        let mut dy = 0.0;

        match paddle {
            PaddleSize::Left => {
                if keyboard_input.pressed(KeyCode::S) {
                    dy -= 1.0;
                }

                if keyboard_input.pressed(KeyCode::W) {
                    dy += 1.0;
                }

                let translation = &mut transform.translation;

                // move the paddle horizontally
                translation.y += time.delta_seconds() * dy * 1200.0;
                // bound the paddle within the walls
                translation.y = translation.y.min(520.0 - 50.0).max(-520.0 + 50.0);
            }
            PaddleSize::Right => {
                if keyboard_input.pressed(KeyCode::Down) {
                    dy -= 1.0;
                }

                if keyboard_input.pressed(KeyCode::Up) {
                    dy += 1.0;
                }

                let translation = &mut transform.translation;

                // move the paddle horizontally
                translation.y += time.delta_seconds() * dy * 1200.0;
                // bound the paddle within the walls
                translation.y = translation.y.min(520.0 - 50.0).max(-520.0 + 50.0);
            }
        }
    }
}

fn ballmovement(time: Res<Time>, mut query: Query<(&mut Transform, &Ball, &BallDirection)>) {
    for (mut transform, ball, direction) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y += 200.0 * time.delta_seconds() * direction.0 as f32;
        translation.x += 200.0 * time.delta_seconds() * direction.1 as f32;
    }
}
