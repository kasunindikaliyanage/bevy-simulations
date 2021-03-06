use bevy::prelude::Component;

#[derive(Component,Clone)]
pub struct Velocity{
    pub x : f32,
    pub y : f32
}
    
#[derive(Component)]
pub struct Position{
    pub x : f32,
    pub y : f32
}