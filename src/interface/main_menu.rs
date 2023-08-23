use bevy_lunex::prelude::*;
use bevy::prelude::*;

use super::style::FastFlickerEffect;

/// Main menu constructor function
pub fn build_main_menu (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    //Create temporary UI tree
    let mut temporary_tree = UiTree::new();

    //Create the widgets and handle errors
    let main_menu = Widget::create(&mut temporary_tree, "main_menu", RelativeLayout::default().pack())?;
    let background = Widget::create(&mut temporary_tree, &main_menu.end("background"), SolidLayout::default().with_width(1920.0).with_height(1080.0).with_scaling(SolidScale::Fill).pack())?;


    let boundary = Widget::create(&mut temporary_tree, &main_menu.end("boundary"), SolidLayout::default().with_width(16.0).with_height(9.0).with_scaling(SolidScale::Fit).pack())?;

    //Merge the temporary tree to main tree if nothing failed so far
    ui_tree.merge(temporary_tree)?;

    //Spawn the Background
    commands.spawn(ImageElementBundle::new(background.clone(), &ImageParams::default().with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("images/interface/main_menu/background_0.png"), Vec2::new(1920.0,1080.0)));
    commands.spawn((
        ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.1), asset_server.load("images/interface/main_menu/background_1.png"), Vec2::new(1920.0,1080.0)),
        FastFlickerEffect::new(1.0, 0.02, 0.0, 0.8),
    ));
    commands.spawn((
        ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.2), asset_server.load("images/interface/main_menu/background_2.png"), Vec2::new(1920.0,1080.0)),
        FastFlickerEffect::new(3.0, 0.02, 0.3, 1.0),
    ));
    commands.spawn((
        ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.3), asset_server.load("images/interface/main_menu/background_3.png"), Vec2::new(1920.0,1080.0)),
        FastFlickerEffect::new(5.0, 0.01, 0.0, 1.5),
    ));
    commands.spawn((
        ImageElementBundle::new(background.clone(), &ImageParams::default().with_depth(0.4), asset_server.load("images/interface/main_menu/background_4.png"), Vec2::new(1920.0,1080.0)),
        FastFlickerEffect::new(7.0, 0.02, 0.1, 1.2),
    ));


    //# Spawn the planet
    commands.spawn((
        ImageElementBundle::new(boundary.clone(), &ImageParams::center().with_depth(0.5).at(85.0, 75.0), asset_server.load("images/interface/main_menu/planet.png"), Vec2::new(1230.0,1341.0)),
        SlowRotation::new(f32::to_radians(-0.02)),
    ));

    //# Spawn the ring
    commands.spawn((
        ImageElementBundle::new(boundary.clone(), &ImageParams::center().with_depth(0.5).at(85.0, 75.0).scaled(200.0), asset_server.load("images/interface/main_menu/planet_ring_lowres.png"), Vec2::new(1077.0,1065.0)),
        SlowRotation::new(f32::to_radians(0.02)),
    ));

    Result::Ok(())
}




/// # Slow Rotation Effect
#[derive(Component, Default)]
pub struct SlowRotation {
    ang_speed: f32,
}
impl SlowRotation {
    pub fn new (ang_speed: f32) -> SlowRotation {
        SlowRotation {
            ang_speed,
        }
    }
}
pub fn slow_rotation_update (mut query: Query<(&mut Transform, &SlowRotation)>) {
    for (mut transform, rotation) in &mut query {
        transform.rotate_z(rotation.ang_speed);
    }
}



pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, slow_rotation_update);
    }
}