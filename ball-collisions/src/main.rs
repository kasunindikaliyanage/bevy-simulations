
use bevy::{prelude::*};
use ball::BallPlugin;

mod ball;
mod components;

struct WinSize{
    h:f32,
    w:f32,
}

struct GameTextures{
    ball: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Ball collision".to_string(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .add_startup_system(setup)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows : ResMut<Windows>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    let (WinW, WinH) = (window.width(), window.height());

    let win_size = WinSize{ h: WinH, w: WinW};
    commands.insert_resource(win_size);

    let game_textures= GameTextures {
        ball: asset_server.load("ball.png"),
    };

    commands.insert_resource(game_textures);
}
