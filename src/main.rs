use bevy_lunex::prelude::*;
use bevy::{prelude::*, sprite::Anchor};

//# For visual effects only
use bevy::core_pipeline::bloom::{BloomSettings, BloomPrefilterSettings, BloomCompositeMode};
use bevy::core_pipeline::tonemapping::Tonemapping;

mod interface;

use crate::interface::main_menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Stardawn".into(),
                    mode: bevy::window::WindowMode::Windowed,
                    ..Default::default()
                }),
                ..Default::default()
            }
        ))
        .add_systems(Startup, setup)

        //Debug
        //.add_plugins(LunexDebugPlugin)


        .add_systems(Update, (hierarchy_update, cursor_update).chain())

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //# Spawn the camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1000. },
                ..default()
            },
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings {
            intensity: 0.20,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 0.9,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.25,
                threshold_softness: 0.1,
            },
            composite_mode: BloomCompositeMode::Additive,
        }
    ));

    //# Spawn cursor
    commands.spawn ((
        Cursor::new(10.0),
        SpriteBundle {
            texture: asset_server.load("cursor_mouse.png"),
            transform: Transform { translation: Vec3 { x: 0., y: 0., z: 800. } , scale: Vec3 { x: 0.4, y: 0.4, z: 1. }, ..default() },
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 2.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        },
    ));


    let mut ui_tree = UITree::new();
    build_main_menu(&mut commands, &asset_server, &mut ui_tree).unwrap();
    

    println!("{}", ui_tree.get_map_debug());
    println!("{}", ui_tree.get_map());
    commands.spawn (ui_tree);

}