#![allow(unused)] //silence unsed warning for learning 

use bevy::prelude::*;

const WINDOW_TITLE: &str = "Space shooter game on Rust"; 
const PLAYER_SPRITE: &str =  "Texture2D/shiptt_0012_Layer-3-copy-9.png";
const BACKGROUND: &str = "Texture2D/bg1.png";
const TIME_STEP: f32 = 1. / 60.;
pub struct Materials {
    background_materials: Handle<Image>,
    player_materials : Handle<Image>
}   
pub struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerSpeed(f32);    //tuple struct
impl Default for PlayerSpeed {
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
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_background", SystemStage::single(background_spawn.system()))
        .add_startup_stage("game_setup_player", SystemStage::single(player_spawn.system()))
        .add_system(player_movement.system())
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
    })
    .insert(Player)
    .insert(PlayerSpeed::default());   

}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
){

    for (speed, mut transform, _) in query.iter_mut() {

        const MAX_VELOCITY: f32 = 16.0;

        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            1.
        }
        else{
            0.
        };


        transform.translation.x += dir * speed.0 + TIME_STEP;
        // transform.translation.x += dir + TIME_STEP;
        transform.translation.x = transform.translation.x.clamp(-320.0, 320.0);
        
    }
  
}   