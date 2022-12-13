use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};

mod player;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Test".to_owned(),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(5.))
                .with_rotation(Quat::from_rotation_z(to_radians(45.))),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        });
}

pub fn clear_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn to_radians(degree: f32) -> f32 {
    degree * (22./7.) / 180.
}

fn to_degree(degree: f32) -> f32 {
    degree * 180. / (22. / 7.)
}
