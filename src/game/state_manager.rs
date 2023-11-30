use std::collections::HashMap;

use crate::spec::state::StateDef;
use crate::spec::triggers::ExpressionContext;
use crate::CharState;

pub struct StateManager {
    state_map: HashMap<i32, StateDef>,
    current_state: i32,
    last_state: i32,
}

impl StateManager {
    pub fn new(state_map: HashMap<i32, StateDef>) -> Self {
        Self {
            state_map,
            current_state: 0,
            last_state: 0,
        }
    }

    pub fn add_state(&mut self, state_no: i32, state: StateDef) {
        self.state_map.insert(state_no, state);
    }

    pub fn update(&mut self, char: &mut CharState, ctx: &mut ExpressionContext) {
        self.current_state = char.state_no;
        self.run_state(char, -1, ctx);
        self.run_state(char, self.current_state, ctx);
        self.last_state = self.current_state;
    }

    pub fn get_current_state(&self) -> i32 {
        self.current_state
    }

    pub fn get_last_state(&self) -> i32 {
        self.last_state 
    }

    fn run_state(&self, char: &mut CharState, state_no: i32, ctx: &mut ExpressionContext) {
        let state_container = self.state_map.get(&state_no).unwrap();

        // only do this stuff to initialize a new state
        if self.last_state != self.current_state {
            if let Some(anim_no) = state_container.anim {
                char.set_animation_no(anim_no);
            }

            if let Some(ctrl_flag) = state_container.ctrl {
                char.set_ctrl_flag(if ctrl_flag { 1 } else { 0 });
            }

            if let Some(vel) = state_container.velset {
                char.set_velocity(vel);
            }

            char.set_state_type(state_container.state_type);
            char.set_state_physics(state_container.physics);
        } else if state_no != -1 {
            char.increment_state_time();
        }

        // deal with persitency
        if state_no > 0 {
            if char.get_persistent_value() == 0 && char.get_persistent_counter() > 0 {
                return;
            } else if char.get_persistent_value() > 1
                && ((char.get_persistent_counter() + 1) % char.get_persistent_value()) != 0
            {
                return;
            }
            char.increment_persistent_counter();
        }
        for state in &state_container.states {
            if state_no == 400 {
                //dbg!(state_no);
                //dbg!(&state.label);
                //dbg!(char.get_state_time());
            }

            ctx.mid_frame_update(char);
            if state.trigger_handler.evaluate(ctx) {
                (state.controller)(char, state.args.clone(), ctx);
            }
            ctx.mid_frame_update(char);
        }
    }
}
