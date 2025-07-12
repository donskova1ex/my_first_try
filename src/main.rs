use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, spawn_ground))
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity {
    y: f32,
}

#[derive(Component)]
struct Collider {
    size: Vec2,
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Collision {
    Top,
    Bottom,
    Left,
    Right,
}
const PLAYER_SPEED: f32 = 200.0;
const GRAVITY: f32 = -500.0;
const JUMP_STRENGTH: f32 = 300.0;
const PLAYER_HEIGHT: f32 = 32.0;
const MAX_FALL_SPEED: f32 = -400.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Камера
    commands.spawn(Camera2dBundle::default());

    // Спрайт игрока
    let player_texture = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Velocity { y: 0.0 },
        Collider{ size: Vec2::new(16.0, 32.0)},
    ));
}

fn spawn_ground(mut commands: Commands) {
    let ground_width = 1000.0;
    let ground_height = 20.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(ground_width, ground_height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -250.0, 0.0),
            ..default()
        },
        Collider { size: Vec2::new(ground_width, ground_height) },

    ));
}

fn player_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const GROUND_LEVEL: f32 = -250.0 + 10.0;

    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let dt = time.delta_seconds();

        let mut direction_x = 0.0;
        if input.pressed(KeyCode::KeyA) {
            direction_x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction_x += 1.0;
        }

        transform.translation.x += direction_x * PLAYER_SPEED * dt;

        if input.just_pressed(KeyCode::Space) && is_on_ground(transform.translation.y, PLAYER_HEIGHT) {
            velocity.y = JUMP_STRENGTH;
        }

        velocity.y += GRAVITY * dt;
        velocity.y = velocity.y.clamp(MAX_FALL_SPEED, f32::INFINITY); // ограничение скорости падения

        transform.translation.y += velocity.y * dt;

        if transform.translation.y - PLAYER_HEIGHT / 2.0 <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL + PLAYER_HEIGHT / 2.0;
            velocity.y = 0.0;
        }
    }
}

fn is_on_ground(y_position: f32, player_height: f32) -> bool {
    let ground_level = -250.0 + 10.0;
    let player_bottom = y_position - player_height / 2.0;
    (player_bottom - ground_level).abs() < 0.1
}
