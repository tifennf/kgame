use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Node;

pub fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            println!("click en ({}, {})", position.x, position.y);
        }
    }
}
