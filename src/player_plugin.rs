use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};
use bevy::color::palettes::css::BLACK;
use bevy_rapier2d::prelude::*;

const RES_WIDTH: u32 = 1366;
const RES_HEIGHT: u32 = 768;
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// Константы игрока
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
        app.insert_resource(PlayerSettings::default())
            .add_systems(Startup, (setup_camera, spawn_player))
            .add_systems(Update, (player_movement, fit_canvas));
    }
}

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_texture = asset_server.load("player.png");
    
    commands.spawn((
        Sprite {
            image: player_texture,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        ColliderMassProperties::Mass(1.0),
        GravityScale(3.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.0),
        Damping {
            linear_damping: 5.0,
            angular_damping: 0.0
        },
        ExternalImpulse::default(),
        Player,
        PlayerDirection { direction_x: 0.0 },
        PIXEL_PERFECT_LAYERS,
    ));
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };
    
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);
    
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(BLACK.into()),
            ..default()
        },
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));
    
    commands.spawn((
        Sprite::from_image(image_handle),
        Canvas,
        HIGH_RES_LAYERS,
    ));
    
    commands.spawn((
        Camera2d,
        Msaa::Off,
        OuterCamera,
        HIGH_RES_LAYERS,
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

        let world_pos = transform.translation * Vec3::new(
            RES_WIDTH as f32 / 2.0,
            RES_HEIGHT as f32 / 2.0,
            1.0,
        );
        println!("Player position: {:?}", world_pos);
    }
}

fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projection: Single<&mut Projection, With<OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };

    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}