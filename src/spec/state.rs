use std::str::FromStr;

use super::controllers::StateArgs;
use super::triggers::{Condition, ExpressionContext};
use crate::game::char::CharState;

pub struct StateDef {
    pub state_type: StateType,
    pub move_type: MoveType,
    pub physics: Physics,
    pub anim: Option<i32>,
    pub velset: Option<(i32, i32)>,
    pub ctrl: Option<bool>,
    pub power_add: Option<i32>,
    pub juggle: Option<i32>,
    pub face_p2: bool,
    pub hit_def_persist: bool,
    pub move_hit_persist: bool,
    pub hit_count_persist: bool,
    pub spr_priority: Option<u8>,
    pub states: Vec<State>,
}

pub struct State {
    pub label: String,
    pub controller: Box<dyn Fn(&mut CharState, StateArgs, &ExpressionContext)>,
    pub args: StateArgs,
    pub ignore_hit_pause: bool,
    pub persistency: i32,
    pub trigger_handler: TriggerHandler,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StateType {
    S,
    C,
    A,
    L,
    U,
}

impl ToString for StateType {
    fn to_string(&self) -> String {
        match self {
            StateType::S => "s".to_string(),
            StateType::C => "c".to_string(),
            StateType::A => "a".to_string(),
            StateType::L => "l".to_string(),
            StateType::U => "u".to_string(),
        }
    }
}

impl FromStr for StateType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(StateType::S),
            "c" => Ok(StateType::C),
            "a" => Ok(StateType::A),
            "l" => Ok(StateType::L),
            "u" => Ok(StateType::U),
            _ => Err("invalid state type"),
        }
    }
}

impl Default for StateType {
    fn default() -> Self {
        StateType::S
    }
}

pub enum MoveType {
    A,
    I,
    H,
    U,
}

impl FromStr for MoveType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(MoveType::A),
            "i" => Ok(MoveType::I),
            "h" => Ok(MoveType::H),
            "u" => Ok(MoveType::U),
            _ => Err("invalid move type"),
        }
    }
}

impl Default for MoveType {
    fn default() -> Self {
        MoveType::I
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Physics {
    S,
    C,
    A,
    N,
    U,
}

impl FromStr for Physics {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(Physics::S),
            "c" => Ok(Physics::C),
            "a" => Ok(Physics::A),
            "n" => Ok(Physics::N),
            "u" => Ok(Physics::U),
            _ => Err("invalid physics type"),
        }
    }
}

impl Default for Physics {
    fn default() -> Self {
        Physics::N
    }
}

pub struct TriggerHandler {
    pub triggers: TriggerSet,
    pub triggerall: Option<Trigger>,
}

impl TriggerHandler {
    pub fn evaluate(&self, ctx: &ExpressionContext) -> bool {
        // Check all TriggerAll conditions
        if self.triggerall.is_some() {
            if !self.triggerall.as_ref().unwrap().evaluate(ctx) {
                return false;
            }
        }

        self.triggers.evaluate(ctx)
    }
}

// Define a Trigger as a collection of Conditions (AND logic)
pub struct Trigger {
    pub conditions: Vec<Condition>,
}

impl Trigger {
    fn evaluate(&self, ctx: &ExpressionContext) -> bool {
        for condition in &self.conditions {
            if !condition.evaluate(ctx) {
                return false;
            }
        }
        return true;
    }
}

impl FromIterator<Condition> for Trigger {
    fn from_iter<T: IntoIterator<Item = Condition>>(iter: T) -> Self {
        let conditions: Vec<Condition> = iter.into_iter().collect();
        Self { conditions }
    }
}

// Collection of Triggers (OR logic)
pub struct TriggerSet {
    pub triggers: Vec<Trigger>,
}

impl TriggerSet {
    fn evaluate(&self, ctx: &ExpressionContext) -> bool {
        // OR logic between triggers, AND logic within a trigger (same as before)
        for trigger in &self.triggers {
            if trigger.evaluate(ctx) {
                return true;
            }
        }
        false
    }
}

pub mod common_states {
    pub const STAND: i32 = 0; 
    pub const STAND_TO_CROUCH: i32 = 10; 
    pub const CROUCHING: i32 = 11; 
    pub const CROUCH_TO_STAND: i32 = 12; 
    pub const WALK: i32 = 20; 
    pub const JUMP_START: i32 = 40; 
    pub const AIR_JUMP_START: i32 = 45; 
    pub const JUMP_UP: i32 = 50; 
    pub const JUMP_DOWN: i32 = 51; 
    pub const JUMP_LAND: i32 = 52; 
    pub const RUN_FWD: i32 = 100; 
    pub const HOP_BACKWARDS: i32 = 105;
    pub const HOP_BACKWARDS_LAND: i32 = 106; 
    pub const GUARD_START: i32 = 120; 
    pub const STAND_GUARD_GUARDING: i32 = 130; 
    pub const CROUCH_GUARD_GUARDING: i32 = 131; 
    pub const AIR_GUARD_GUARDING: i32 = 132; 
    pub const GUARD_END: i32 = 140; 
    pub const STAND_GUARD_HIT_SHAKING: i32 = 150; 
    pub const STAND_GUARD_HIT_KNOCKED_BACK: i32 = 151; 
    pub const CROUCH_GUARD_HIT_SHAKING: i32 = 152; 
    pub const CROUCH_GUARD_HIT_KNOCKED_BACK: i32 = 153; 
    pub const AIR_GUARD_HIT_SHAKING:i32 = 154; 
    pub const AIR_GUARD_HIT_KNOCKED_AWAY:i32 = 155; 
    pub const LOSE_TIME_OVER: i32 = 170; 
    pub const DRAW_GAME_TIME_OVER: i32 = 175; 
    pub const PRE_INTRO: i32 = 190; 
    pub const INTRO: i32 = 191; 
    pub const STAND_GET_HIT_SHAKING:i32 = 5000; 
    pub const STAND_GET_HIT_KNOCKED_BACK:i32 = 5001; 
    pub const CROUCH_GET_HIT_SHAKING: i32 = 5010; 
    pub const CROUCH_GET_HIT_KNOCKED_BACK: i32 = 5011; 
    pub const AIR_GET_HIT_SHAKING: i32 = 5020; 
    pub const AIR_GET_HIT_KNOCKED_AWAY:i32 = 5030; 
    pub const AIR_GET_HIT_TRANSITION: i32 = 5035; 
    pub const AIR_GET_HIT_RECOVERING: i32 = 5040; 
    pub const AIR_GET_HIT_FALLING: i32 = 5050; 
    pub const TRIPPED_GET_HIT_SHAKING: i32 = 5070; 
    pub const TRIPPED_GET_HIT_KNOCKED_AWAY: i32 = 5071; 
    pub const DOWNED_GET_HIT_SHAKING: i32 = 5080; 
    pub const DOWNED_GET_HIT_KNOCKED_BACK:i32 = 5081; 
    pub const DOWNED_GET_HIT_HIT_GROUND:i32 = 5100;
    pub const DOWNED_GET_HIT_BOUNCE:i32 = 5101; 
    pub const DOWNED_GET_HIT_LYING:i32 = 5110;
    pub const DOWNED_GET_HIT_GETTING_UP:i32 = 5120;
    pub const DOWNED_GET_HIT_DEFEATED:i32 = 5150; 
    pub const AIR_GET_HIT_FALL_RECORY_STILL_FALLING:i32 = 5200; 
    pub const AIR_GET_HIT_FALL_RECOVERY_ON_GROUND:i32 = 5201; 
    pub const AIR_GET_HIT_FALL_RECOVERY_IN_AIR:i32 = 5210; 
    pub const CONTINUE_SCREEN:i32 = 5500; 
    pub const INITIALIZE_AT_ROUND_START:i32 = 5900; 
}