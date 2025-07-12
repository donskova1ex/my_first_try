use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 1800.0;
const JUMP_IMPULSE: f32 = 800.0;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
struct PlayerDirection {
    direction_x: f32,
}
#[derive(Resource)]
pub struct PlayerSettings {
    pub speed: f32,
    pub jump_impulse: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            speed: PLAYER_SPEED,
            jump_impulse: JUMP_IMPULSE,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerSettings::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
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
}

fn player_movement(
    mut query: Query<(&mut ExternalImpulse, &Transform), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player_settings: Res<PlayerSettings>,
) {
    if let Ok((mut impulse, transform)) = query.single_mut() {
        let dt = time.delta_secs();

        let mut direction_x = 0.0;

        if input.pressed(KeyCode::KeyA) {
            direction_x = -1.0;
        } else if input.pressed(KeyCode::KeyD) {
            direction_x = 1.0;
        }

        impulse.impulse.x = direction_x * player_settings.speed * dt;


        if input.just_pressed(KeyCode::Space) {
            impulse.impulse.y = player_settings.jump_impulse;
        } else {
            impulse.impulse.y = 0.0;
        }

        println!("Player position: {:?}", transform.translation);
        println!("Impulse applied: {:?}", impulse.impulse);
    } else {
        println!("Player not found!");
    }
}