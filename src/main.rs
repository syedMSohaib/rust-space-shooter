#![allow(unused)] //silence unsed warning for learning 

use bevy::prelude::*;

const WINDOW_TITLE: &str = "Space shooter game on Rust"; 
const PLAYER_SPRITE: &str =  "Texture2D/shiptt_0012_Layer-3-copy-9.png";
const BACKGROUND: &str = "Texture2D/bg1.png";
pub struct Materials {
    background_materials: Handle<Image>,
    player_materials : Handle<Image>
}   
pub struct WinSize {
    w: f32,
    h: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: WINDOW_TITLE.to_string(),
            width: 600.0,
            height: 700.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_background", SystemStage::single(background_spawn.system()))
        .add_startup_stage("game_setup_player", SystemStage::single(player_spawn.system()))
        .run()

}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut windows: ResMut<Windows>
 ) {

    let mut window = windows.get_primary_mut().unwrap();

    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        background_materials: asset_server.load(BACKGROUND),
        player_materials: asset_server.load(PLAYER_SPRITE),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height()
    })
}

fn background_spawn(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        texture: materials.background_materials.clone(),
        ..Default::default()
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, winsize: Res<WinSize>) {

    let bottom = - winsize.h / 2.;    
    commands.spawn_bundle(SpriteBundle {
        texture: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75.0 / 4. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default() 
        },
        ..Default::default()
    });   

}