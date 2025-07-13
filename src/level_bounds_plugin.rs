use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct LevelBoundary;

pub struct LevelBoundsPlugin;

impl Plugin for LevelBoundsPlugin {
    fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_level_bounds);
    }
}

fn setup_level_bounds(mut commands: Commands) {
    let screen_width = 1366.0;
    let screen_height = 768.0;
    let boundary_thickness = 10.0;

    commands.spawn((
        Sprite {
           color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(screen_width, boundary_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -screen_height / 2. - boundary_thickness / 2., 0.0),
        RigidBody::Fixed,
        Collider:: cuboid(screen_width/2., boundary_thickness /2.),
        LevelBoundary,
        ));
    commands.spawn((
        Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(screen_width, boundary_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, screen_height / 2.0 + boundary_thickness / 2.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(screen_width / 2.0, boundary_thickness / 2.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(boundary_thickness, screen_height)),
            ..default()
        },
        Transform::from_xyz(-screen_width / 2.0 - boundary_thickness / 2.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(boundary_thickness / 2.0, screen_height / 2.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(boundary_thickness, screen_height)),
            ..default()
        },
        Transform::from_xyz(screen_width / 2.0 + boundary_thickness / 2.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(boundary_thickness / 2.0, screen_height / 2.0),
    ));
}