use crate::game::{char::CharState, state_manager::StateManager}; 
use ggez::{
    event,
    glam::*,
    graphics::{self, Color, ImageFormat, Text},
    Context, GameResult,
};

pub fn char_debug(char: &CharState) -> Text {
    let state_no_string = format!("State No: {}\n", char.get_state_no()); 
    let state_time_string = format!("State Time: {}\n", char.get_state_time()); 
    let state_type_string = format!{"State Type: {:?}\n", char.get_state_type()}; 
    let prev_state_string = format!("Previous State: {}\n", char.get_prev_state_no()); 
    let ctrl_string = format!("Ctrl: {:?}\n", char.get_ctrl() != 0); 

    let anim_time_string = format!("Animation Time: {}\n", char.get_anim_time());
    let anim_no_string = format!("Animation Number: {}\n", char.get_anim_no()); 

    let mut  full_string = String::new();
    full_string.push_str(&state_no_string); 
    full_string.push_str(&state_time_string); 
    full_string.push_str(&state_type_string); 
    full_string.push_str(&prev_state_string); 
    full_string.push_str(&ctrl_string); 
    full_string.push_str(&anim_time_string); 
    full_string.push_str(&anim_no_string); 

    Text::new(full_string) 
}

pub struct DebugSystem {

}

impl DebugSystem {
    pub fn draw(text: Text, canvas: &mut graphics::Canvas, draw_position: Vec2) {
        canvas.draw(&text, graphics::DrawParam::new().dest(draw_position)) 
    }
}