use bevy::{prelude::*, window::PrimaryWindow};
use rand::{seq::SliceRandom, Rng};

mod utils;

pub const PLAYER_SIZE: f32 = 64.0; // this is the size of the player sprite
pub const ENEMY_SIZE: f32 = 64.0; // this is the size of the enemy sprite
pub const STAR_SIZE: f32 = 30.0; // this is the size of the star sprite
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_TIMESTEP: f32 = 1.0;
pub const COLLISION_REBOUND_STRENGTH: f32 = 50.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const PLAYER_START_HEALTH: u32 = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_enemies, spawn_stars),
        )
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement.after(player_movement),
                enemy_movement,
                confine_enemy_movement.after(enemy_movement),
                player_hit_enemy,
                player_hit_star,
                update_score,
                tick_star_spawn_timer,
                spawn_stars_over_time,
                kill_player_without_health,
            ),
        )
        .add_systems(
            FixedUpdate,
            enemy_redirection.before(confine_enemy_movement),
        )
        .insert_resource(FixedTime::new_from_secs(ENEMY_TIMESTEP))
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Health {
    current: u32,
}

#[derive(Component)]
pub struct Enemy {
    direction: Vec3,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
        Health {
            current: PLAYER_START_HEALTH,
        },
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, ENEMY_SIZE);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x_position: f32 = rng.gen_range(x_min..=x_max);
        let y_position: f32 = rng.gen_range(y_min..=y_max);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_position, y_position, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::ZERO,
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, STAR_SIZE);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for _ in 0..NUMBER_OF_STARS {
        let x_position: f32 = rng.gen_range(x_min..=x_max);
        let y_position: f32 = rng.gen_range(y_min..=y_max);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_position, y_position, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn enemy_redirection(mut enemy_query: Query<&mut Enemy>) {
    let sample_directions: [f32; 3] = [-1.0, 0.0, 1.0];
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for mut enemy in &mut enemy_query {
        let mut direction = Vec3::ZERO;
        let x_random: &f32 = sample_directions
            .choose(&mut rng)
            .expect("Random x direction should have been generated.");
        let y_random: &f32 = sample_directions
            .choose(&mut rng)
            .expect("Random y direction should have been generated.");
        direction += Vec3::new(*x_random, *y_random, 0.0);
        enemy.direction = direction.normalize_or_zero();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in &mut enemy_query {
        enemy_transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, ENEMY_SIZE);

    for (mut enemy_transform, mut enemy) in &mut enemy_query {
        let mut changed_direction: bool = false;

        if enemy_transform.translation.x < x_min {
            enemy_transform.translation.x = x_min;
            enemy.direction.x = -enemy.direction.x;
            changed_direction = true;
        } else if enemy_transform.translation.x > x_max {
            enemy_transform.translation.x = x_max;
            enemy.direction.x = -enemy.direction.x;
            changed_direction = true;
        }
        if enemy_transform.translation.y < y_min {
            enemy_transform.translation.y = y_min;
            enemy.direction.y = -enemy.direction.y;
            changed_direction = true;
        } else if enemy_transform.translation.y > y_max {
            enemy_transform.translation.y = y_max;
            enemy.direction.y = -enemy.direction.y;
            changed_direction = true;
        }

        if changed_direction {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/pluck_001.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }

        direction = direction.normalize_or_zero();

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();
        let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, PLAYER_SIZE);

        if player_transform.translation.x < x_min {
            player_transform.translation.x = x_min;
        } else if player_transform.translation.x > x_max {
            player_transform.translation.x = x_max;
        }
        if player_transform.translation.y < y_min {
            player_transform.translation.y = y_min;
        } else if player_transform.translation.y > y_max {
            player_transform.translation.y = y_max;
        }
    }
}

pub fn player_hit_enemy(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut player_transform, mut player_health)) = player_query.get_single_mut() {
        let collision_distance = (PLAYER_SIZE + ENEMY_SIZE) / 2.0;
        for mut enemy_transform in &mut enemy_query {
            let mut relative_vector_in_plane = Vec3 {
                x: player_transform.translation.x - enemy_transform.translation.x,
                y: player_transform.translation.y - enemy_transform.translation.y,
                z: 0.0,
            };

            if relative_vector_in_plane.length() > collision_distance {
                continue;
            }

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/explosionCrunch_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });

            relative_vector_in_plane = relative_vector_in_plane.normalize_or_zero();
            enemy_transform.translation -= COLLISION_REBOUND_STRENGTH * relative_vector_in_plane;
            player_transform.translation += COLLISION_REBOUND_STRENGTH * relative_vector_in_plane;

            player_health.current -= 1;
            println!("You lost a health point ({} left)!", player_health.current)
        }
    }
}

pub fn kill_player_without_health(
    mut commands: Commands,
    player_query: Query<(Entity, &Health), With<Player>>,
) {
    if let Ok((player_entity, player_health)) = player_query.get_single() {
        if player_health.current > 0 {
            return;
        }

        commands.entity(player_entity).despawn();
        println!("You died!")
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&mut Transform, (With<Player>, Without<Star>)>,
    star_query: Query<(Entity, &mut Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let collision_distance = (PLAYER_SIZE + ENEMY_SIZE) / 2.0;
        for (star_entity, star_transform) in &star_query {
            let relative_vector_in_plane = Vec3 {
                x: player_transform.translation.x - star_transform.translation.x,
                y: player_transform.translation.y - star_transform.translation.y,
                z: 0.0,
            };

            if relative_vector_in_plane.length() > collision_distance {
                continue;
            }

            commands.spawn(AudioBundle {
                source: asset_server.load("audio/laserLarge_000.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });

            commands.entity(star_entity).despawn();
            score.value += 1;
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    star_spawn_timer: ResMut<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if !star_spawn_timer.timer.finished() {
        return;
    }

    let window: &Window = window_query.get_single().unwrap();
    let [x_min, x_max, y_min, y_max] = utils::get_confinement(window, STAR_SIZE);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let x_position: f32 = rng.gen_range(x_min..=x_max);
    let y_position: f32 = rng.gen_range(y_min..=y_max);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x_position, y_position, 0.0),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {},
    ));
}
