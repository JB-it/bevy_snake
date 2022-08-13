use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::snek::{SnekBody, SnekHead};

pub struct FoobPlugin;

#[derive(Component)]
pub struct Foob {
    pub eaten: bool,
}

impl Plugin for FoobPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(foob_setup).add_system(respawn_foob);
    }
}

fn foob_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(90.)),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        })
        .insert(Foob { eaten: false });
}

fn respawn_foob(
    mut foob_query: Query<(&mut Transform, &mut Foob)>,
    snek_body: Query<&Transform, (Or<(With<SnekBody>, With<SnekHead>)>, Without<Foob>)>,
) {
    for (mut trausform, mut foob) in foob_query.iter_mut() {
        if foob.eaten {
            let mut set_successfully = false;
            while !set_successfully {
                let new_position = Vec3::new(
                    (rand::thread_rng().gen_range(-4..4) * 100) as f32,
                    (rand::thread_rng().gen_range(-4..4) * 100) as f32,
                    0.0,
                );

                if snek_body
                    .iter()
                    .filter(|body_transform| {
                        body_transform.translation.x == new_position.x
                            && body_transform.translation.y == new_position.y
                    })
                    .count()
                    > 0
                {
                    continue;
                }

                trausform.translation = new_position;
                foob.eaten = false;
                set_successfully = true;
            }
        }
    }
}
