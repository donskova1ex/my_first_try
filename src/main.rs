use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

const PLAYER_SPEED: f32 = 1800.0;
const JUMP_IMPULSE: f32 = 800.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerDirection {
    direction_x: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d::default());

    let player_texture = asset_server.load("player.png");

    commands.spawn((
        Sprite {
            image: player_texture,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(8.0, 16.0),
        ColliderMassProperties::Mass(1.0),
        GravityScale(3.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.0),
        Damping { linear_damping: 5.0, angular_damping: 0.0 },
        ExternalForce::default(),
        ExternalImpulse::default(),
        Player,
        PlayerDirection { direction_x: 0.0 },
    ));
    commands.spawn((
        Sprite {
            image: Default::default(),
            texture_atlas: None,
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(1000.0, 20.0)),
            rect: None,
            anchor: Default::default(),
            flip_x: false,
            flip_y: false,
            image_mode: Default::default(),
        },
        Transform::from_xyz(0.0, -350.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
    ));

    commands.spawn((
        Sprite {
            image: Default::default(),
            texture_atlas: None,
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            rect: None,
            anchor: Default::default(),
            flip_x: false,
            flip_y: false,
            image_mode: Default::default(),
        },
        Transform::from_xyz(0.0, -150.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(100.0, 10.0),
    ));
}

fn player_movement(
    mut query: Query<(&mut ExternalImpulse, &Transform), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut impulse, transform)) = query.single_mut() {
        let dt = time.delta_secs();

        let mut direction_x = 0.0;

        if input.pressed(KeyCode::KeyA) {
            direction_x = -1.0;
        } else if input.pressed(KeyCode::KeyD) {
            direction_x = 1.0;
        }

        // Применяем горизонтальный импульс
        impulse.impulse.x = direction_x * PLAYER_SPEED * dt;

        // Прыжок при нажатии пробела
        if input.just_pressed(KeyCode::Space) {
            impulse.impulse.y = JUMP_IMPULSE;
        } else {
            impulse.impulse.y = 0.0; // Не держим прыжок постоянно
        }

        println!("Player position: {:?}", transform.translation);
        println!("Impulse applied: {:?}", impulse.impulse);
    } else {
        println!("Player not found!");
    }
}