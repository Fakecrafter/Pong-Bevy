#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::sprite::collide_aabb::*;


// TODO:
// - ball collision with paddles
// - speeeed up ball at every touch with paddle
// - scoreboard
// - put everything in modules
// TODO's for later:
// - make values responsive to size of window

struct BallSpeed(i32);

struct PointUpEvent(PaddleSide);

struct GameState {
    left: u32,
    right: u32,
}

struct Ball;
struct Paddle;

enum PaddleSide {
    Left,
    Right,
}

struct BallDirection(i32, i32);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<PointUpEvent>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(GameState {left: 0, right: 0})
        .add_startup_system(setup.system())
        .add_system(paddle_movement.system())
        .add_system(ball_movement.system())
        .add_system(ball_collision.system())
        .add_system(check_point.system())
        .add_system(reset.system())
        .add_system(collision.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    windows.get_primary_mut().unwrap().set_mode(WindowMode::Fullscreen {use_size: false});

    // paddle 1
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(-920.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 100.0)),
            ..Default::default()
        })
        .insert(PaddleSide::Left)
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
        .insert(PaddleSide::Right)
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
        .insert(BallDirection(1, -1))
        .insert(BallSpeed(400))
        .id();
}

fn paddle_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &PaddleSide)>,
) {
    for (mut transform, paddle) in query.iter_mut() {
        let mut dy = 0.0;

        match paddle {
            PaddleSide::Left => {
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
                // translation.y = translation.y.min(520.0 - 50.0).max(-520.0 + 50.0);
                translation.y = translation.y.min(540.0 - 50.0).max(-540.0 + 50.0);
            }
            PaddleSide::Right => {
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
                translation.y = translation.y.min(540.0 - 50.0).max(-540.0 + 50.0);
            }
        }
    }
}

fn ball_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Ball, &BallDirection, &BallSpeed)>) {
    for (mut transform, ball, direction, ballspeed) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += ballspeed.0 as f32 * time.delta_seconds() * direction.0 as f32;
        translation.y += ballspeed.0 as f32 * time.delta_seconds() * direction.1 as f32;
    }
}


fn ball_collision(mut query: Query<(&Transform, &mut BallDirection, &Ball)>) {
    for (transform, mut direction, ball) in query.iter_mut() {
        let translation = &transform.translation;
        if translation.y.abs() >= 525.0 {
            direction.1 *= -1;
        }
    }
}


fn check_point(mut state: ResMut<GameState>, query: Query<(&Transform, &Ball)>, mut ev_pointup: EventWriter<PointUpEvent>) {
    for (transform, ball) in query.iter() {
        if transform.translation.x >= 975.0 {
            ev_pointup.send(PointUpEvent(PaddleSide::Right));
            state.right += 1;
        }
        if transform.translation.x <= -975.0 {
            ev_pointup.send(PointUpEvent(PaddleSide::Left));
            state.left += 1;
        }
    }
}


fn reset(mut ev_pointup: EventReader<PointUpEvent>,
         mut q: QuerySet<(
         Query<(&mut Transform, &Ball, &mut BallSpeed)>,
         Query<(&mut Transform, &Paddle)>
        )>
    ) {
    for ev in ev_pointup.iter() {
        for (mut transform, ball, mut ballspeed) in q.q0_mut().iter_mut() {
            transform.translation = Vec3::zero();
            ballspeed.0 = 400;
        }
        for (mut transform, paddle) in q.q1_mut().iter_mut() {
            transform.translation.y = 0.0;
        }
    }
}

fn collision(
     mut ball_query: Query<(&Transform, &mut BallDirection, &Sprite, &mut BallSpeed)>,
     paddle_query:   Query<(&Transform, &Sprite, &Paddle)>)
{
    for (transform, mut direction, sprite, mut ballspeed) in ball_query.iter_mut() {
        for (ptransform, psprite, paddle) in paddle_query.iter() {
            match collide(transform.translation, sprite.size, ptransform.translation, psprite.size) {
                Some(Collision::Left) => {
                    direction.0 *= -1;
                    ballspeed.0 += 20;
                }
                Some(Collision::Right) => {
                    direction.0 *= -1;
                    ballspeed.0 += 20;
                }
                Some(Collision::Top) => {
                    direction.1 *= -1;
                    ballspeed.0 += 10;
                }
                Some(Collision::Bottom) => {
                    direction.1 *= -1;
                    ballspeed.0 += 10;
                }
                None => {
                }
            }
        }
    }
}
