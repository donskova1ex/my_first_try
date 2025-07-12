use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// ====== Типы платформ ======
#[derive(Debug, Clone, Copy)]
pub enum PlatformType {
    Static { color: Color },
    Deadly { color: Color },
    MovingHorizontal { color: Color, range: f32, speed: f32 },
    MovingVertical { color: Color, range: f32, speed: f32 },
}

// ====== Данные о платформе ======
#[derive(Debug, Clone)]
pub struct PlatformSpawnEvent {
    pub position: Vec3,
    pub platform_type: PlatformType,
    pub size: Vec2,
}

// ====== Настройки по умолчанию ======
#[derive(Resource)]
pub struct PlatformSettings {
    pub platforms: Vec<PlatformSpawnEvent>,
}

impl Default for PlatformSettings {
    fn default() -> Self {
        Self {
            platforms: vec![
                // Земля
                PlatformSpawnEvent {
                    position: Vec3::new(0.0, -350.0, 0.0),
                    platform_type: PlatformType::Static { color: Color::srgb(0.3, 0.3, 0.3) },
                    size: Vec2::new(1000.0, 20.0),
                },
                // Обычная платформа
                PlatformSpawnEvent {
                    position: Vec3::new(0.0, -150.0, 0.0),
                    platform_type: PlatformType::Static { color: Color::srgb(0.5, 0.5, 0.5) },
                    size: Vec2::new(200.0, 20.0),
                },
                // Смертельная платформа
                PlatformSpawnEvent {
                    position: Vec3::new(300.0, -250.0, 0.0),
                    platform_type: PlatformType::Deadly { color: Color::srgb(1.0, 0.0, 0.0) },
                    size: Vec2::new(100.0, 20.0),
                },
                // Движущаяся платформа
                PlatformSpawnEvent {
                    position: Vec3::new(-300.0, -200.0, 0.0),
                    platform_type: PlatformType::MovingHorizontal { color: Color::srgb(0.0, 0.5, 1.0), range: 100.0, speed: 50.0 },
                    size: Vec2::new(100.0, 20.0),
                },
            ],
        }
    }
}

// ====== Компонент для движущихся платформ ======
#[derive(Component)]
struct MovingPlatform {
    start_x: f32,
    range: f32,
    speed: f32,
    direction: f32,
}

// ====== Плагин ======
pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlatformSettings::default())
            .add_systems(Startup, spawn_platforms)
            .add_systems(Update, moving_platform_system);
    }
}

// ====== Спавн всех типов платформ ======
fn spawn_platforms(
    mut commands: Commands,
    settings: Res<PlatformSettings>,
) {
    for event in &settings.platforms {
        let half_size = Vec2::new(event.size.x / 2.0, event.size.y / 2.0);

        match event.platform_type {
            PlatformType::Static { color } => {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(event.size),
                        ..default()
                    },
                    Transform::from_translation(event.position),
                    RigidBody::Fixed,
                    Collider::cuboid(half_size.x, half_size.y),
                ));
            }
            PlatformType::Deadly { color } => {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(event.size),
                        ..default()
                    },
                    Transform::from_translation(event.position),
                    RigidBody::Fixed,
                    Collider::cuboid(half_size.x, half_size.y),
                    DeadlyPlatform,
                ));
            }
            PlatformType::MovingHorizontal { color, range, speed } => {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(event.size),
                        ..default()
                    },
                    Transform::from_translation(event.position),
                    RigidBody::KinematicVelocityBased,
                    Collider::cuboid(half_size.x, half_size.y),
                    MovingPlatform {
                        start_x: event.position.x,
                        range,
                        speed,
                        direction: 1.0,
                    },
                ));
            }
            PlatformType::MovingVertical { color, range, speed } => {
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(event.size),
                        ..default()
                    },
                    Transform::from_translation(event.position),
                    RigidBody::KinematicVelocityBased,
                    Collider::cuboid(half_size.x, half_size.y),
                    MovingPlatform {
                        start_x: event.position.x,
                        range,
                        speed,
                        direction: 1.0,
                    },
                ));
            }
        }
    }
}

// ====== Компонент для триггеров ======
#[derive(Component)]
pub struct DeadlyPlatform;

// ====== Логика движущихся платформ ======
fn moving_platform_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovingPlatform)>,
) {
    for (mut transform, mut platform) in query.iter_mut() {
        let x = transform.translation.x;
        let min_x = platform.start_x - platform.range;
        let max_x = platform.start_x + platform.range;

        if x >= max_x {
            platform.direction = -1.0;
        } else if x <= min_x {
            platform.direction = 1.0;
        }

        let movement = platform.direction * platform.speed * time.delta_secs();
        transform.translation.x += movement;
    }
}