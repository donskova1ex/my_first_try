use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

// ====== Ресурс для конфигурации ======
#[derive(Resource)]
pub struct PlatformSettings {
    pub platforms: Vec<Vec3>,
}

impl Default for PlatformSettings {
    fn default() -> Self {
        Self {
            platforms: vec![
                Vec3::new(0.0, -350.0, 0.0),   // Земля
                Vec3::new(0.0, -150.0, 0.0),   // Платформа выше
                Vec3::new(300.0, -250.0, 0.0), // Платформа справа
                Vec3::new(-300.0, -200.0, 0.0), // Платформа слева
            ],
        }
    }
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlatformSettings::default())
            .add_systems(Startup, spawn_platforms);
    }
}

fn spawn_platforms(
    mut commands: Commands,
    settings: Res<PlatformSettings>,
) {
    for &pos in &settings.platforms {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(200.0, 20.0)),
                ..default()
            },
            Transform::from_translation(pos),
            RigidBody::Fixed,
            Collider::cuboid(100.0, 10.0),
        ));
    }
}