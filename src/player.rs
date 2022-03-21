use bevy::prelude::*;

use crate::{Materials, Player, WinSize, Speed, Laser, TIME_STEP, PlayerReadyToFire};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, group: &mut App) {
        group.add_startup_stage("game_setup_player", SystemStage::single(player_spawn.system()))
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .add_system(laser_movement.system());
        
    }
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
    .insert(PlayerReadyToFire(true))
    .insert(Speed::default());   

}

fn laser_movement(
    mut commands: Commands,
    winsize: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>
){
    for(laser_entity, speed, mut transform, _) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y += speed.0 + TIME_STEP;
        
        if translation.y > winsize.h {
            commands.entity(laser_entity).despawn();
        }
    }
}

fn player_fire(
    mut commands: Commands, 
    materials: Res<Materials>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&Transform, &mut PlayerReadyToFire, With<Player>)>,
){
    for (transform, mut ready_to_fire, _) in query.iter_mut() {
        if ready_to_fire.0 && keyboard_input.pressed(KeyCode::Space) {
            let x = transform.translation.x;
            let y = transform.translation.y; 
            // info!("x={:?} y={:?}",x, y);

            let mut spawn_lasers = |x_offset: f32| {
                commands.spawn_bundle(SpriteBundle {
                    texture: materials.laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + 15., 0.),
                        scale: Vec3::new(0.1, 0.1, 1.0),
                        ..Default::default() 
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(Speed::default());   
    
            };

            let x_offset = 180. / 9. - 5.;


            info!("x_offset={:?}",x_offset);

            spawn_lasers(x_offset);
            spawn_lasers(-x_offset);


            ready_to_fire.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) {
            ready_to_fire.0 = true;
        }
        
    }   
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
){

    for (speed, mut transform, _) in query.iter_mut() {

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