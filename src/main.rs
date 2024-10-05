use bevy::{prelude::*, app::AppExit};

const ANGULAR_SPEED: f32 = 0.03;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct AngularVelocity(Quat);

fn print_system(query: Query<&Velocity>) {
    for velocity in &query {
        println!("velocity: {} {}", velocity.0.x, velocity.0.y);
    }
}

fn add_components(mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((Velocity(Vec3 {x: 0.1, y: 0.1, z: 0.0}),
                    AngularVelocity(Quat::from_axis_angle(
                        Vec3::new(1.0, 0.0, 0.0),
                        ANGULAR_SPEED)),
                    PbrBundle {
                        mesh: meshes.add(shape::Torus { radius: 6.0,
                                                        ..default()}
                                         .into()),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                        ..default()
                    }));

    commands.spawn((Velocity(Vec3 {x: 0.1, y: 0.1, z: 0.0}),
                    AngularVelocity(Quat::from_axis_angle(
                        Vec3::new(0.7, 0.3, 0.0),
                        -ANGULAR_SPEED)),
                    PbrBundle {
                        mesh: meshes.add(shape::Torus { radius: 4.0,
                                                        ..default()}
                                         .into()),
                        material: materials.add(Color::GRAY.into()),
                        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                        ..default()
                    }));

    commands.spawn((Velocity(Vec3 {x: 0.1, y: 0.1, z: 0.0}),
                    AngularVelocity(Quat::from_axis_angle(
                        Vec3::new(0.85, 0.15, 0.0),
                        ANGULAR_SPEED)),
                    PbrBundle {
                        mesh: meshes.add(shape::Torus { radius: 2.0,
                                                        ..default()}
                                         .into()),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                        ..default()
                    }));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            color: Color::Hsla { hue: 0.5, saturation: 0.5, lightness: 0.7, alpha: 1.0 },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_positions_system(mut query: Query<(&mut Transform, &AngularVelocity)>) {
    for (mut transform, velocity) in &mut query {
        // transform.translation.add_assign(velocity.v);
        transform.rotate(velocity.0);

    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_components)
        .add_systems(Update, update_positions_system)
        .add_systems(Update, print_system)
        .run();
}
