use self::battle::{BattleState, BattleSystem};

pub mod animation;
pub mod battle;
pub mod char;
pub mod input;
pub mod state_manager;

pub struct GameSystem {
    pub battle: BattleSystem,
}

#[derive(Copy, Clone)]
pub struct GameState {
    pub battle: BattleState,
}
