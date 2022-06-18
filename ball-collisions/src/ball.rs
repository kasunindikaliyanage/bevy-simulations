use std::option;

use crate::{GameTextures, WinSize, components::{Velocity, Position}};
use bevy::{prelude::*, sprite::{self, collide_aabb::collide}};
use rand::Rng;

const  BALL_SPRITE_SCALE: f32 = 0.05;
const  BALL_RADIUS : f32 = 17.;
const  MAX_NUM_BALLS: u8 = 100;
const  COL_PADDING:f32 = 0.;

pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app: &mut App){
        app
        .add_startup_system_to_stage(StartupStage::PostStartup,setup_system)
        .add_system(ball_movement_system);
    }
}

fn setup_system(mut commands:Commands, win_size:Res<WinSize>, game_textures:Res<GameTextures>){ 
    let win_w_half = win_size.w /2.;
    let win_h_half = win_size.h / 2.;

    let mut ball_count =0;
    while ball_count <= MAX_NUM_BALLS{
        print!("Hello");
        let mut rng = rand::thread_rng();
        let p_x = rng.gen_range((-win_w_half + BALL_RADIUS )..(win_w_half -BALL_RADIUS));
        let p_y = rng.gen_range((-win_h_half + BALL_RADIUS)..(win_h_half - BALL_RADIUS));

        let v_x = rng.gen_range(-2.0..2.0);
        let v_y = rng.gen_range(-2.0..2.0);
        
        commands.spawn_bundle(SpriteBundle{
            texture: game_textures.ball.clone(),
            transform: Transform{
                translation: Vec3::new(p_x, p_y , 10.),
                scale: Vec3::new( BALL_SPRITE_SCALE, BALL_SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Velocity{x:v_x, y:v_y})
        .insert(Position{x:p_x, y:p_y});
        ball_count +=1;
    }
}


fn ball_movement_system(mut commands:Commands, win_size: Res<WinSize>, mut query : Query<( Entity ,&mut Velocity, &mut Transform)>){

    let mut boids = Vec::new();
    
    for (mut e, mut v, mut t) in query.iter_mut() {
        boids.push((e.clone(), v.clone(), t.clone()));
        t.translation.x += v.x;
        t.translation.y += v.y;

        let win_w_half = win_size.w /2.;
        let win_h_half = win_size.h / 2.;

        if t.translation.x <= (-win_w_half + BALL_RADIUS) {
            v.x *= -1.;
            t.translation.x +=COL_PADDING;
        }
        else if t.translation.x >= (win_w_half - BALL_RADIUS){
            v.x *= -1.;
            t.translation.x -=COL_PADDING ;
        }

        if t.translation.y <= (-win_h_half + BALL_RADIUS){
            v.y *= -1.;
            t.translation.y += COL_PADDING;
        }
        else if t.translation.y >= (win_h_half - BALL_RADIUS){
            v.y *= -1.;
            t.translation.y -= COL_PADDING;
        }
    }

    for ( e1, mut v1, mut t1) in query.iter_mut() {
        for (e2, _, t2) in boids.iter() {
            if e1 != *e2 {
                let options = collide(t1.translation, Vec2::new(20.,20.), t2.translation, Vec2::new(20.,20.));
                if let Some(_) = options {
                    // Need to keep a map of already despawn entities and despawn only the ones currently not in the map
                    commands.entity(e1).despawn();
                    commands.entity(*e2).despawn();
                }
            }
        }
    }

}