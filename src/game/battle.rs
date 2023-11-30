use crate::spec::controllers::StateController;
use std::collections::HashMap;

// Non Game State
pub struct BattleSystem {
    state_map: HashMap<String, StateController>,
}

// Game State
#[derive(Copy, Clone)]
pub struct BattleState {}
