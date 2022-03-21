// #![allow(unused)] //silence unsed warning for learning 

mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

const WINDOW_TITLE: &str = "Space shooter game on Rust"; 
const PLAYER_SPRITE: &str =  "Texture2D/shiptt_0012_Layer-3-copy-9.png";
const BACKGROUND: &str = "Texture2D/bg1.png";
const LASER_SPRITE: &str = "Texture2D/fireRay_copy.png";
// const LASER_SPRITE: &str = "Texture2D/2_top.png";
const TIME_STEP: f32 = 1. / 60.;
pub struct Materials {
    background_materials: Handle<Image>,
    player_materials : Handle<Image>,
    laser: Handle<Image>
}   
pub struct WinSize {
    #[allow(unused)]
    w: f32,
    h: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Laser;

#[derive(Component)]
struct PlayerReadyToFire(bool);

#[derive(Component)]
struct Speed(f32);    //tuple struct
impl Default for Speed {
    fn default() -> Self {
        Self(5.)
    }
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
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_background", SystemStage::single(background_spawn.system()))
        .run()

}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut windows: ResMut<Windows>
 ) {

    let window = windows.get_primary_mut().unwrap();

    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        background_materials: asset_server.load(BACKGROUND),
        player_materials: asset_server.load(PLAYER_SPRITE),
        laser: asset_server.load(LASER_SPRITE),
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
