use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, setup)
        .add_systems(Update, update_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    });

    commands.spawn((
        ColorMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(200.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(0.),
        Collider::ball(5.),
        LinearVelocity(Vec2::new(500., 0.)),
    ));
}

fn update_camera(
    player_position: Query<&Position>,
    mut camera_position: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let (Ok(player_position), Ok(mut camera_transform)) = (
        player_position.get_single(),
        camera_position.get_single_mut(),
    ) {
        camera_transform.translation = camera_transform
            .translation
            .lerp(player_position.extend(1.), 5. * time.delta_seconds());
    }
}
