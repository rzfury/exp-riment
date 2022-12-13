use bevy::{prelude::{Component, Bundle, Plugin, App, SystemSet, Commands, ResMut, Assets, Mesh, ParallelSystemDescriptorCoercion}, sprite::{ColorMesh2dBundle, ColorMaterial}};

use crate::{GameState, clear_entities};

struct PlayerPlugin;

#[derive(Component)]
struct Player {
  pub speed: f32,
}

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(
        SystemSet::on_enter(GameState::Playing)
            .with_system(clear_entities::<Player>.before(spawn_player))
            .with_system(spawn_player),
    );
  }
}

#[derive(Component)]
struct Collider(f32);

#[derive(Bundle)]
struct PlayerBundle {
  #[bundle]
  shape_primary: ColorMesh2dBundle,
  #[bundle]
  shape_decoration: ColorMesh2dBundle,
  collider: Collider,
}

pub fn spawn_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  
}
