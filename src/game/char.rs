use super::{
    animation::Animator,
    input::{InputFrame, InputState, InputSystem},
};
use crate::{
    cmd::{Direction, DirectionKind},
    spec::state::Physics,
};
use crate::{
    spec::{
        cmd::{CommandList, Key},
        constants::char_constants::*,
        state::common_states,
        state::StateType,
    },
    utils::sprite_sheet::SpriteSheet,
};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Chain;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color, ImageFormat, Text},
    Context, GameResult,
};
pub struct CharSystem {
    sprite_sheet: SpriteSheet,
    pub constants: CharConstants,
    input: InputSystem,
}

impl CharSystem {
    pub fn new(sprite_sheet: SpriteSheet, constants: CharConstants) -> CharSystem {
        CharSystem {
            sprite_sheet,
            constants,
            input: InputSystem::new(),
        }
    }

    pub fn update(&mut self, context: &mut Context) -> InputFrame {
        self.input.update(context)
    }

    pub fn draw(
        &mut self,
        size: (f32, f32),
        canvas: &mut graphics::Canvas,
        draw_position: Vec2,
        position: Vec2,
        group: u16,
        image: u16,
    ) {
        let axis = self.sprite_sheet.get_axis(group, image);
        let sprite_size = self.sprite_sheet.get_size(group, image);
        let size_vec = Vec2::from(size);

        let mut position = position + draw_position;
        position.x += ((size_vec.x - 320.0) / 2.0) - axis.0 as f32;
        position.y += (size_vec.y - 240.0) - axis.1 as f32;

        //position -=  Vec2::new(axis.0 as f32, axis.1 as f32);
        canvas.draw(
            self.sprite_sheet.get(group, image),
            graphics::DrawParam::new().dest(position),
        );
    }

    pub fn constant(&self, name: &str) -> Option<ConstantValue> {
        self.constants.get(name)
    }
}

pub struct CharState {
    pub animator: Animator,
    pub position: Vec2,
    pub direction: Vec2,
    pub velocity: Vec2,
    pub ctrl_flag: i32,
    pub state_no: i32,
    pub state_time: i32,
    pub draw_position: Vec2,
    pub draw_translation: Vec2,
    pub state_physics: Physics,
    alive: i32,
    state_type: StateType,
    sys_var: [i32; 6],
    var: [i32; 60],
    fvar: [f32; 60],
    input: InputState,
    pub command_list: CommandList,
    last_command: Option<String>,
    pub prev_state_no: i32,
    pub persistent: i32,
    pub persistent_counter: i32,
}

impl CharState {
    pub fn update(&mut self, frame_no: i32, frame: InputFrame, constants: &CharConstants) {
        self.input.update(frame_no, frame, &self.command_list);

        self.movement(); 
        self.physics(constants);
        self.position = self.position + self.velocity;
        self.animator.update();
    }
    /*
        if self.state_type == StateType::S && self.input.buffered_key(Direction::U) {
            if self.state_no != 40 {
                self.set_state(40);
            }
        } else if self.state_type == StateType::S
            && self.state_no != common_states::CROUCHING
            && self
                .input
                .buffered(Key::Direction(DirectionKind::Single(Direction::D)))
        {
            if self.state_no == common_states::RUN_FWD {
                self.set_velocity((0, 0));
            }
            // self.set_velocity((0, 25));
            //self.set_draw_translation(0.0, 10.0);
            self.set_state(10);
        } else if self.state_no == common_states::CROUCHING {
            //self.set_velocity((0, 0));
            self.set_draw_translation(0.0, 0.0);

            if !self.input.command("holddown") {
                self.set_state(12);
            }
        } else if self.state_no == common_states::RUN_FWD {
            self.set_velocity((18, 0));
            self.last_command = Some("FF".to_string());
        } else if self.state_no == common_states::HOP_BACKWARDS {
            self.last_command = Some("BB".to_string())
        } else if self.input.buffered_key((Direction::F)) && !self.is_jumping() {
            //&& self.state_no != 100{
            self.set_state(20);
            let vel_x = match constants.get("velocity.walk.fwd.x") {
                Some(ConstantValue::Int(vel_x)) => vel_x,
                Some(ConstantValue::Float(vel_x)) => vel_x as i32,
                _ => 0,
            };
            self.set_velocity((10, 0));
            self.last_command = Some("F".to_string());
        } else if self.input.buffered_key(Direction::B) && !self.is_jumping() {
            //&& self.state_no != 100 {
            self.set_state(20);
            self.set_velocity((-9, 0));
            self.last_command = Some("B".to_string());
        } else {
            if let Some(cmd) = &self.last_command {
                if cmd == "F" || cmd == "B" || cmd == "FF" || cmd == "BB" {
                    //dbg!("resetting!");
                    self.set_state(0);
                    self.set_velocity((0, 0));
                    self.last_command = None;
                }
            }
        }
     */
    fn is_jumping(&self) -> bool {
        dbg!(self.state_no); 
        // !(self.state_type != StateType::A && self.state_physics != Physics::A)
             !(self.state_no < common_states::JUMP_START
                || self.state_no > common_states::JUMP_DOWN)
    }

    fn movement(&mut self) {
        self.vertical_movement();
        self.Horizontal_movement(); 
        self.diagonal_movement(); 
    }

    fn jump(&mut self) {
        if self.state_no != common_states::JUMP_START {
            self.set_state(common_states::JUMP_START);
        }
    }

    fn vertical_movement(&mut self) {
        if self.state_type == StateType::S {
            if self.input.buffered_key(Direction::U) {
                self.jump(); 
            } else if self.state_no != common_states::CROUCHING
                && self.input.buffered_key(Direction::D)
            {
                if self.state_no == common_states::RUN_FWD {
                    self.set_velocity((0, 0));
                }
                self.set_state(common_states::STAND_TO_CROUCH);
            }
        } else if self.state_no == common_states::CROUCHING {
            self.set_draw_translation(0.0, 0.0);

            if !self.input.command("holddown") {
                self.set_state(common_states::CROUCH_TO_STAND);
            }
        }
    }

    fn Horizontal_movement(&mut self) {
        if self.state_no == common_states::RUN_FWD {
            self.set_velocity((18, 0));
            self.last_command = Some("FF".to_string());
        } else if self.state_no == common_states::HOP_BACKWARDS {
            self.last_command = Some("BB".to_string())
        } else if self.input.buffered_key((Direction::F)) && !self.is_jumping() {
            self.set_state(common_states::WALK);
            self.set_velocity((10, 0));
            self.last_command = Some("F".to_string());
        } else if self.input.buffered_key(Direction::B) && !self.is_jumping() {
            self.set_state(common_states::WALK);
            self.set_velocity((-9, 0));
            self.last_command = Some("B".to_string());
        } else {
            self.reset_to_neutral(); 
        }        
    }
    
    fn diagonal_movement(&mut self) {
        if self.input.buffered_key(Direction::UF) || self.input.buffered_key(Direction::UB){
            self.jump(); 
        }
    }

    fn reset_to_neutral(&mut self) {
        if let Some(cmd) = &self.last_command {
            if cmd == "F" || cmd == "B" || cmd == "FF" || cmd == "BB" {
                self.set_state(common_states::STAND);
                self.set_velocity((0, 0));
                self.last_command = None;
            }
        }
    }

    /*
    pub fn update(&mut self, context: &mut Context) {
        if context
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Right)
        {
            if self.animator.current_action != 20 {
                self.animator.set_action(20);
            }
            self.direction = Vec2::new(1.45, 0.0);
            self.position = self.position + self.direction;
        } else if context
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Left)
        {
            if self.animator.current_action != 21 {
                self.animator.set_action(21);
            }
            self.direction = -Vec2::new(1.40, 0.0);
            self.position = self.position + self.direction;
        } else if context
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Down)
        {
            if self.animator.current_action != 11 {
                self.animator.set_action(11);
            }
            self.direction = Vec2::new(0.0, 0.0);
        } else {
            if self.animator.current_action != 0 {
                self.animator.set_action(0);
            }
            self.direction = Vec2::new(0.0, 0.0);
        }
        self.animator.update();
    }
    */

    pub fn get_int_var(&self, idx: usize) -> i32 {
        self.var[idx]
    }

    pub fn set_int_var(&mut self, idx: usize, val: i32) {
        self.var[idx] = val;
    }

    pub fn get_flaot_var(&self, idx: usize) -> f32 {
        self.fvar[idx]
    }

    pub fn set_float_var(&mut self, idx: usize, val: f32) {
        self.fvar[idx] = val;
    }

    pub fn set_state_physics(&mut self, physics: Physics) {
        self.state_physics = physics;
    }

    pub fn physics(&mut self, constants: &CharConstants) {
        if self.state_physics == Physics::A {
            self.gravity(constants);
            self.land();
        }
    }

    pub fn land(&mut self) {
        if self.get_velocity().1 > 28.0 && self.get_state_no() != 105 && self.get_state_no() != 52 {
            self.set_state(52);
        }
    }

    pub fn gravity(&mut self, constants: &CharConstants) {
        self.velocity.y += match constants
            .get("movement.yaccel")
            .unwrap_or(ConstantValue::Float(1.76))
        {
            ConstantValue::Float(f) => f,
            ConstantValue::Int(i) => i as f32,
        };
    }

    pub fn get_persistent_value(&self) -> i32 {
        self.persistent
    }

    pub fn increment_persistent_counter(&mut self) {
        self.persistent_counter += 1;
    }

    pub fn get_persistent_counter(&self) -> i32 {
        self.persistent_counter
    }

    pub fn increment_state_time(&mut self) {
        self.state_time += 1;
    }

    pub fn set_state_type(&mut self, state_type: StateType) {
        self.state_type = state_type;
    }

    pub fn set_velocity(&mut self, vel: (i32, i32)) {
        self.velocity.x = vel.0 as f32;
        self.velocity.y = vel.1 as f32;
    }

    pub fn set_position(&mut self, pos: (i32, i32)) {
        self.position.x = pos.0 as f32;
        self.position.y = pos.1 as f32;
    }

    pub fn set_state(&mut self, state_no: i32) {
        self.persistent_counter = 0;
        self.state_time = 0;
        self.prev_state_no = self.state_no;
        self.state_no = state_no;
    }

    pub fn get_prev_state_no(&self) -> i32 {
        self.prev_state_no
    }

    pub fn set_draw_translation(&mut self, x: f32, y: f32) {
        self.draw_translation.x = x;
        self.draw_translation.y = y;
    }

    pub fn get_draw_position(&self) -> Vec2 {
        self.draw_position
    }

    pub fn set_draw_position(&mut self, x: f32, y: f32) {
        self.draw_position.x = x;
        self.draw_position.y = y;
    }

    pub fn set_animation_no(&mut self, anim_no: i32) {
        self.animator.set_action(anim_no as u64);
    }

    pub fn set_animation_element(&mut self, element_no: i32) {
        self.animator.set_element(element_no);
    }

    pub fn set_ctrl_flag(&mut self, ctrl_flag: i32) {
        self.ctrl_flag = ctrl_flag;
    }

    pub fn draw(&mut self) -> (u16, u16) {
        self.draw_position = self.draw_position + self.draw_translation;
        self.animator.draw()
    }

    pub fn get_anim_no(&self) -> u64 {
        self.animator.get_anim_no()
    }
    pub fn get_anim_element(&self) -> usize {
        self.animator.get_anim_element() 
    }
    pub fn get_anim_time(&self) -> i64 {
        self.animator.get_anim_time()
    }

    pub fn get_state_time(&self) -> i32 {
        self.state_time
    }

    pub fn get_velocity(&self) -> (f32, f32) {
        (self.velocity.x, self.velocity.y)
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    pub fn get_alive(&self) -> i32 {
        self.alive
    }

    pub fn get_state_no(&self) -> i32 {
        self.state_no
    }

    pub fn get_ctrl(&self) -> i32 {
        self.ctrl_flag
    }

    pub fn get_state_type(&self) -> StateType {
        self.state_type
    }

    // this will come from inputstate or wherever
    pub fn command(&self, name: &str) -> bool {
        self.input.command(name)
    }

    pub fn sys_var(&self, idx: usize) -> i32 {
        self.sys_var[idx]
    }

    pub fn set_sys_var(&mut self, idx: usize, val: i32) {
        self.sys_var[idx] = val;
    }

    pub fn get_move_contact(&self) -> i32 {
        0 
    }
}

pub struct CharBuilder {
    animator: Option<Animator>,
    position: Vec2,
    direction: Vec2,
    velocity: Vec2,
    ctrl_flag: i32,
    state_no: i32,
    state_time: i32,
    alive: i32,
    command_list: Option<CommandList>,
}

impl CharBuilder {
    pub fn new() -> Self {
        Self {
            animator: None,
            position: Vec2::new(80.0, 150.0),
            direction: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            ctrl_flag: 1,
            state_no: 0,
            state_time: 0,
            alive: 1,
            command_list: None,
        }
    }
    pub fn animator(mut self, animator: Animator) -> Self {
        self.animator = Some(animator);
        self
    }

    pub fn command_list(mut self, command_list: CommandList) -> Self {
        self.command_list = Some(command_list);
        self
    }

    pub fn build(self) -> CharState {
        CharState {
            animator: self.animator.unwrap(),
            position: Vec2::new(0.0, 0.0),
            direction: self.direction,
            velocity: self.velocity,
            ctrl_flag: self.ctrl_flag,
            state_no: self.state_no,
            prev_state_no: -1, // remember that you did this.
            state_time: self.state_time,
            alive: self.alive,
            state_type: StateType::default(),
            sys_var: [0; 6],
            var: [0; 60],
            fvar: [0.0; 60],
            input: InputState::new(),
            command_list: self.command_list.unwrap(),
            last_command: None,
            draw_position: self.position,
            draw_translation: Vec2::new(0.0, 0.0),
            state_physics: Physics::S,
            persistent: 1,
            persistent_counter: 0,
        }
    }
}
