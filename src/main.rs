//#![allow(unused)] // silence unused warning while learning

mod player;
mod enemy;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use player::PlayerPlugin;
use enemy::EnemyPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1.0 / 60.0;


// Resources
pub struct Materials {
    player: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
    enemy: Handle<ColorMaterial>,
    explosion: Handle<TextureAtlas>,
}
struct WinSize {
    #[allow(unused)]
    w: f32,
    h: f32,
}

struct ActiveEnemies(u32);

// Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;

struct Enemy;

struct Explosion;
struct ExplosionToSpawn(Vec3);
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self (500.0)
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup.system())
        .add_system(laser_hit_enemy.system())
        .add_system(explosion_to_spawn.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlasas: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);

    // create the main resources
    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_SPRITE).into()),
        enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
        explosion: texture_atlasas.add(texture_atlas),
    });
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // position window
    window.set_position(IVec2::new(1314, 0));
}

fn laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>
) {
    for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
            let laser_scale = Vec2::from(laser_tf.scale);
            let enemy_scale = Vec2::from(enemy_tf.scale);

            let collision = collide(
                laser_tf.translation,
                laser_sprite.size * laser_scale,
                enemy_tf.translation,
                enemy_sprite.size * enemy_scale,
            );

            if let Some(_) = collision {
                // remove the enemy
                commands.entity(enemy_entity).despawn();
                active_enemies.0 -= 1;

                // remove the laser
                commands.entity(laser_entity).despawn();

                // spawn explosion 
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn(
    mut commands: Commands,
    mut query: Query<(Entity, &ExplosionToSpawn)>,
    materials: Res<Materials>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter_mut() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: materials.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion);

        commands.entity(explosion_spawn_entity).despawn();
    }
}
