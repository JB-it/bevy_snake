use bevy::{math::vec3, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{foob::Foob, game_data::GameData, game_state::GameState, settings::Settings};

const MOVEMENT_STEP: f64 = 100.0;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct DeadSnek;

#[derive(Component)]
struct SnekMovement {
    direction: Direction,
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct SnekHead;

#[derive(Component)]
pub struct SnekBody {
    pub snek_head: SnekHead,
    pub lifetime: usize,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AI;

#[derive(Component)]
struct SnekLen(usize);

#[derive(Component)]
struct MovementTimer(Timer);

pub struct SnekPlugin;

impl Plugin for SnekPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_snek)
            .add_system(update_snek_len)
            .add_system(player_controller)
            .add_system(eating_system)
            .add_system(collission_system)
            .add_system(move_snek)
            .add_system(slowly_kill_snek)
            .add_system(update_score)
            .add_system(color_snek);
    }
}

fn setup_snek(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&SnekHead, With<Player>>,
    dead_snek_query: Query<(Entity, &SnekHead), (With<DeadSnek>, Without<Player>)>,
    body_query: Query<(&SnekBody, Entity)>,
    game_data: Res<GameData>,
    mut foob_query: Query<&mut Transform, With<Foob>>,
) {
    if game_data.game_state == GameState::Menu {
        dead_snek_query.for_each(|(snek_entity, snek_head)| {
            for (body, body_entity) in body_query.iter() {
                if body.snek_head == *snek_head {
                    commands.entity(body_entity).despawn();
                }
            }

            commands.entity(snek_entity).despawn_recursive();
        });

        for mut foob_transform in foob_query.iter_mut() {
            foob_transform.translation = vec3(0., 0., 1.);
        }

        if player_query.iter().len() == 0 {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(90.))
                        .with_translation(Vec3::new(100., 0., 2.)),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    ..default()
                })
                .insert(SnekHead)
                .insert(SnekLen(1))
                .insert(MovementTimer(Timer::from_seconds(0.3, true)))
                .insert(SnekMovement {
                    direction: Direction::Right,
                })
                .insert(Player);
        }
    }
}

fn update_snek_len(
    snek_query: Query<(&Transform, &SnekLen, &SnekHead)>,
    body_query: Query<&SnekBody>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (head_pos, snek_len, snek_head) in snek_query.iter() {
        if body_query
            .iter()
            .filter(|b| b.snek_head == *snek_head)
            .count()
            < snek_len.0
        {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform {
                        translation: head_pos.translation,
                        ..Default::default()
                    }
                    .with_scale(Vec3::splat(90.))
                    .with_translation(Vec3::new(-1000., -1000., 1.)),
                    material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
                    ..default()
                })
                .insert(SnekBody {
                    snek_head: *snek_head,
                    lifetime: 1,
                });
        }
    }
}

fn move_snek(
    time: Res<Time>,
    mut snek_query: Query<(
        &mut Transform,
        &SnekMovement,
        &SnekLen,
        &SnekHead,
        &mut MovementTimer,
    )>,
    settings: Res<Settings>,
    mut body_query: Query<(&mut Transform, &mut SnekBody), Without<SnekHead>>,
) {
    for (mut transform, snek_mov, snek_len, head, mut timer) in snek_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for (mut body_transform, mut body_part) in &mut body_query.iter_mut() {
                if body_part.snek_head != *head {
                    continue;
                }
                body_part.lifetime -= 1;
                if body_part.lifetime == 0 {
                    body_transform.translation = transform.translation;
                    body_transform.translation.z = 1.;
                    body_part.lifetime = snek_len.0
                }
            }

            let mut movement = transform.translation;

            match snek_mov.direction {
                Direction::Up => movement.y += MOVEMENT_STEP as f32,
                Direction::Down => movement.y -= MOVEMENT_STEP as f32,
                Direction::Left => movement.x -= MOVEMENT_STEP as f32,
                Direction::Right => movement.x += MOVEMENT_STEP as f32,
            }

            if settings.wraparound {
                movement.x = (movement.x + 450.0 + 900.0) % 900.0 - 450.0;
                movement.y = (movement.y + 450.0 + 900.0) % 900.0 - 450.0;
            } else {
                movement.x = movement.x.min(400.0).max(-400.0);
                movement.y = movement.y.min(400.0).max(-400.0);
            }

            transform.translation = movement;
        }
    }
}

fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut snek_mov_query: Query<(&mut SnekMovement, &Transform), With<Player>>,
    game_data: Res<GameData>,
) {
    for (mut snek_mov, snek_transform) in snek_mov_query.iter_mut() {
        match game_data.game_state {
            GameState::Playing => {
                if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                    snek_mov.direction = Direction::Up;
                }
                if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                    snek_mov.direction = Direction::Down;
                }
                if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                    snek_mov.direction = Direction::Left;
                }
                if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                    snek_mov.direction = Direction::Right;
                }
            }
            GameState::Menu => {
                let mut head_position = snek_transform.translation;
                head_position.z = 0.;
                if head_position == vec3(100f32, 0f32, 0f32) {
                    snek_mov.direction = Direction::Up;
                } else if head_position == vec3(100f32, 100f32, 0f32) {
                    snek_mov.direction = Direction::Left;
                } else if head_position == vec3(-100f32, 100f32, 0f32) {
                    snek_mov.direction = Direction::Down;
                } else if head_position == vec3(-100f32, -100f32, 0f32) {
                    snek_mov.direction = Direction::Right;
                } else if head_position == vec3(100f32, -100f32, 0f32) {
                    snek_mov.direction = Direction::Up;
                }
            }
            _ => {}
        }
    }
}

fn collission_system(
    mut head_query: Query<(&Transform, Entity), With<Player>>,
    body_transform_query: Query<&Transform, (With<SnekBody>, Without<SnekHead>)>,
    mut game_data: ResMut<GameData>,
    mut commands: Commands,
) {
    if game_data.game_state == GameState::Playing {
        for (head_pos, head_entity) in head_query.iter_mut() {
            for body_pos in body_transform_query.iter() {
                if head_pos.translation.x == body_pos.translation.x
                    && head_pos.translation.y == body_pos.translation.y
                {
                    game_data.game_state = GameState::GameOver;

                    commands
                        .entity(head_entity)
                        .remove::<SnekMovement>()
                        .remove::<Player>()
                        .insert(DeadSnek);
                }
            }
        }
    }
}

fn slowly_kill_snek(
    time: Res<Time>,
    mut commands: Commands,
    mut head_query: Query<(&mut MovementTimer, Entity, &SnekHead, &mut SnekLen), With<DeadSnek>>,
    mut body_query: Query<(&mut SnekBody, Entity), Without<SnekHead>>,
) {
    for (mut timer, head_entity, snek_head, mut snek_len) in head_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if snek_len.0 == 0 {
                commands.entity(head_entity).despawn();
                return;
            }

            for (mut body_part, body_entity) in &mut body_query.iter_mut() {
                if body_part.snek_head == *snek_head {
                    body_part.lifetime -= 1;
                    if body_part.lifetime == 0 {
                        commands.entity(body_entity).despawn();
                        snek_len.0 -= 1;
                    }
                }
            }
        }
    }
}

fn eating_system(
    mut head_query: Query<(&Transform, &mut SnekLen, &SnekHead)>,
    mut foob_query: Query<(&Transform, &mut Foob), With<Foob>>,
    mut snek_body_query: Query<&mut SnekBody>,
) {
    for (transform, mut snek_len, snek_head) in head_query.iter_mut() {
        for (foob_transform, mut foob) in foob_query.iter_mut() {
            if transform.translation.x == foob_transform.translation.x
                && transform.translation.y == foob_transform.translation.y
            {
                foob.eaten = true;

                for mut snek_body in snek_body_query.iter_mut() {
                    if snek_body.snek_head != *snek_head {
                        continue;
                    }
                    snek_body.lifetime += 1;
                }

                snek_len.0 += 1;
            }
        }
    }
}

fn update_score(snek_len_query: Query<&SnekLen, With<Player>>, mut game_data: ResMut<GameData>) {
    if let Some(snek_len) = snek_len_query.iter().next() {
        game_data.score = snek_len.0 as i32;
    }
}

fn color_snek(
    mut head_query: Query<&mut Handle<ColorMaterial>, With<SnekHead>>,
    mut body_query: Query<&mut Handle<ColorMaterial>, (With<SnekBody>, Without<SnekHead>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
) {
    for color in head_query.iter_mut() {
        let mut color_mat = materials.get_mut(&color).unwrap();
        color_mat.color = Color::rgba_u8(
            settings.snek_head_color.r(),
            settings.snek_head_color.g(),
            settings.snek_head_color.b(),
            settings.snek_head_color.a(),
        );
    }

    for color in body_query.iter_mut() {
        let mut color_mat = materials.get_mut(&color).unwrap();
        color_mat.color = Color::rgba_u8(
            settings.snek_body_color.r(),
            settings.snek_body_color.g(),
            settings.snek_body_color.b(),
            settings.snek_body_color.a(),
        );
    }
}
