use crate::{
    components::{Position, Velocity},
    GameTextures, NConnection, WinSize,
};
use bevy::{
    prelude::*,
    sprite::{self, collide_aabb::collide},
};
use rand::Rng;
use std::io::{self, BufRead, BufReader, Write};
use std::str;

use std::collections::HashSet;

const BALL_SPRITE_SCALE: f32 = 0.05;
const BALL_RADIUS: f32 = 17.;
const MAX_NUM_BALLS: u16 = 1;
const COL_PADDING: f32 = 0.;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_system)
            .add_system(ball_movement_system);
    }
}

fn setup_system(mut commands: Commands, win_size: Res<WinSize>, game_textures: Res<GameTextures>) {
    let win_w_half = win_size.w / 2.;
    let win_h_half = win_size.h / 2.;

    let mut ball_count = 1;
    while ball_count <= MAX_NUM_BALLS {
        let mut rng = rand::thread_rng();
        let p_x = rng.gen_range((-win_w_half + BALL_RADIUS)..(win_w_half - BALL_RADIUS));
        let p_y = rng.gen_range((-win_h_half + BALL_RADIUS)..(win_h_half - BALL_RADIUS));

        let v_x = rng.gen_range(-2.0..2.0);
        let v_y = rng.gen_range(-2.0..2.0);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.ball.clone(),
                transform: Transform {
                    translation: Vec3::new(p_x, p_y, 10.),
                    scale: Vec3::new(BALL_SPRITE_SCALE, BALL_SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Velocity { x: v_x, y: v_y })
            .insert(Position { x: p_x, y: p_y });
        ball_count += 1;
    }
}

fn ball_movement_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut balls_map: ResMut<HashSet<Entity>>,
    connection: Res<NConnection>,
    mut query: Query<(Entity, &mut Velocity, &mut Transform)>,
) {
    let mut buffer: Vec<u8> = Vec::new();

    let mut reader = BufReader::new(&connection.stream);
    reader
        .read_until(b'\n', &mut buffer)
        .expect("Could not read into buffer");

    let value = str::from_utf8(&buffer).expect("Could not write buffer as string");

    if buffer.len() >= 2 {
        let deserialized: Point = serde_json::from_str(value).unwrap();

        for (mut e, mut v, mut t) in query.iter_mut() {
            t.translation.x = deserialized.x as f32;
            t.translation.y = deserialized.y as f32;
        }
    }
}
