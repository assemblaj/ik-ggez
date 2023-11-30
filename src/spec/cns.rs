use std::collections::HashMap;
use std::str::FromStr;

use crate::utils::ini::*;

use super::constants::char_constants::{parse_char_constants, CharConstants};
use super::controllers::{get_controller, StateArgs, *};
use super::state::{
    MoveType, Physics, State, StateDef, StateType, Trigger, TriggerHandler, TriggerSet,
};
use super::triggers::{Condition, ExpressionContext};

pub struct CNSFile {
    ini: Ini,
}

impl CNSFile {
    const TYPE_KEY: &str = "type";
    const MOVE_TYPE_KEY: &str = "movetype";
    const PHYSICS_KEY: &str = "physics";
    const ANIM_KEY: &str = "anim";
    const VEL_SET_KEY: &str = "velset";
    const CTRL_KEY: &str = "ctrl";
    const POWER_ADD_KEY: &str = "poweradd";
    const JUGGLE_KEY: &str = "juggle";
    const FACE_P2_KEY: &str = "facep2";
    const HIT_DEF_PERSIST_KEY: &str = "hitdefpersist";
    const MOVE_HIT_PERSIST_KEY: &str = "movehitpersist";
    const HIT_COUNT_PERSIST_KEY: &str = "hitcountpersist";
    const SPR_PRIORITY_KEY: &str = "sprpriority";

    pub fn new(cns_file_path: &str) -> CNSFile {
        let ini = load_ini(cns_file_path);
        Self { ini }
    }

    pub fn get_char_constants(self) -> (Self, CharConstants) {
        let constants = parse_char_constants(&self.ini);
        (self, constants)
    }

    pub fn get_states(self) -> HashMap<i32, StateDef> {
        Self::parse_states(&self.ini)
    }

    const STATE_DEF_KEY: &str = "statedef";
    const STATE_KEY: &str = "state "; // extra space to avoid capturing statedef

    pub fn parse_states(ini: &Ini) -> HashMap<i32, StateDef> {
        let mut state_map = HashMap::new();

        for (section_name, section) in ini.iter() {
            dbg!(section_name);
            if section_name.starts_with(Self::STATE_DEF_KEY) {
                let statedef_num = Self::get_statedef_num(section_name);
                let ini_section = match section {
                    SectionContainer::Single(section) => section,
                    _ => unreachable!(), // todo, actually categorize and report these errors. there shouldn't be
                                         // 2 statedefs with the same label
                };
                let statedef = Self::parse_statedef(section_name, ini_section);
                state_map.insert(statedef_num, statedef);
            }

            if section_name.starts_with(Self::STATE_KEY) {
                let statedef_num = Self::get_state_num(&section_name);
                let statedef = state_map.get_mut(&statedef_num).unwrap();

                match section {
                    SectionContainer::Multiple(ini_sections) => {
                        for ini_section in ini_sections {
                            statedef
                                .states
                                .push(Self::parse_state(&section_name, ini_section));
                        }
                    }
                    SectionContainer::Single(ini_section) => {
                        statedef
                            .states
                            .push(Self::parse_state(&section_name, ini_section));
                    }
                }
            }
        }
        state_map
    }

    fn get_statedef_num(statedef: &str) -> i32 {
        statedef.split(" ").nth(1).unwrap().parse().unwrap()
    }

    fn get_state_num(state_label: &str) -> i32 {
        state_label
            .split(",")
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap()
    }

    pub fn parse_statedef(state_name: &str, ini: &IniSection) -> StateDef {
        let state_type = if let Some(state_type_string) = ini.get::<String>(Self::TYPE_KEY) {
            StateType::from_str(&state_type_string.to_lowercase()).unwrap()
        } else {
            StateType::default()
        };

        let move_type = if let Some(move_type_string) = ini.get::<String>(Self::MOVE_TYPE_KEY) {
            MoveType::from_str(&move_type_string.to_lowercase()).unwrap()
        } else {
            MoveType::default()
        };

        let physics = if let Some(physics_string) = ini.get::<String>(Self::PHYSICS_KEY) {
            Physics::from_str(&physics_string.to_lowercase()).unwrap()
        } else {
            Physics::default()
        };

        let anim: Option<i32> = ini.get(Self::ANIM_KEY);
        let velset: Option<(i32, i32)> = ini.get_tuple(Self::VEL_SET_KEY);
        let ctrl = if let Some(ctrl_val) = ini.get::<i32>(Self::CTRL_KEY) {
            Some(ctrl_val != 0)
        } else {
            None
        };
        let power_add: Option<i32> = ini.get(Self::POWER_ADD_KEY);
        let juggle: Option<i32> = ini.get(Self::JUGGLE_KEY);
        let face_p2 = if let Some(face_p2_val) = ini.get::<i32>(Self::FACE_P2_KEY) {
            face_p2_val != 0
        } else {
            false
        };
        let hit_def_persist =
            if let Some(hit_def_persist_val) = ini.get::<i32>(Self::HIT_DEF_PERSIST_KEY) {
                hit_def_persist_val != 0
            } else {
                false
            };
        let move_hit_persist =
            if let Some(move_hit_persist_val) = ini.get::<i32>(Self::MOVE_HIT_PERSIST_KEY) {
                move_hit_persist_val != 0
            } else {
                false
            };
        let hit_count_persist =
            if let Some(hit_count_persist_val) = ini.get::<i32>(Self::HIT_COUNT_PERSIST_KEY) {
                hit_count_persist_val != 0
            } else {
                false
            };
        let spr_priority = ini.get::<u8>(Self::SPR_PRIORITY_KEY);

        StateDef {
            state_type,
            move_type,
            physics,
            anim,
            velset,
            ctrl,
            power_add,
            juggle,
            face_p2,
            hit_def_persist,
            move_hit_persist,
            hit_count_persist,
            spr_priority,
            states: Vec::new(),
        }
    }

    const PERSISTENCY_KEY: &str = "persistent";
    const PERSISTENCY_DEFAULT: i32 = 1;

    const IGNORE_HIT_PAUSE_KEY: &str = "ignorehipause";
    const IGNORE_HIT_PAUSE_DEFAULT: i32 = 0;
    pub fn parse_state(name: &str, ini: &IniSection) -> State {
        let state_label: String = name.split(",").nth(1).unwrap().to_string();

        let controller_name = ini.get_string(Self::TYPE_KEY).unwrap().to_lowercase();
        let controller = get_controller(&controller_name);
        let triggers = Self::parse_triggers(ini);
        let args = StateArgs::new(&controller_name, ini);
        State {
            label: state_label,
            controller,
            args,
            ignore_hit_pause: ini
                .get(Self::IGNORE_HIT_PAUSE_KEY)
                .unwrap_or(Self::IGNORE_HIT_PAUSE_DEFAULT)
                != 0,
            persistency: ini
                .get(Self::PERSISTENCY_KEY)
                .unwrap_or(Self::PERSISTENCY_DEFAULT),
            trigger_handler: triggers,
        }
    }

    pub fn parse_triggers(ini: &IniSection) -> TriggerHandler {
        let mut trigger_all: Option<Trigger> = None;

        if let Some(trigger_all_vec) = ini.get_strings("triggerall") {
            trigger_all = Some(
                trigger_all_vec
                    .iter()
                    .map(|s| Condition::from_str(s))
                    .collect(),
            );
        } else if let Some(trigger_all_string) = ini.get_string("triggerall") {
            trigger_all = Some(Trigger {
                conditions: vec![Condition::from_str(trigger_all_string.as_str())],
            });
        }

        // let mut trigger_num = 1;
        // let mut trigger_set: Vec<Trigger> = Vec::new();
        // while let Some(trigger_vec) = ini.get_strings(format!("trigger{:?}", trigger_num).as_str())
        //     || let Some(trigger_string) = ini.get_string(format!("trigger{:?}", trigger_num).as_str())
        // {
        //     trigger_set.push(trigger_vec.iter().map(|s| Condition::from_str(s)).collect());
        //     trigger_num += 1;
        // }
        let mut trigger_num = 1;
        let mut triggers = TriggerSet { triggers: vec![] };

        loop {
            let trigger_key = format!("trigger{}", trigger_num);
            if let Some(trigger_vec) = ini.get_strings(&trigger_key) {
                triggers.triggers.push(Trigger {
                    conditions: trigger_vec.iter().map(|s| Condition::from_str(s)).collect(),
                });
            } else if let Some(trigger_string) = ini.get_string(&trigger_key) {
                triggers.triggers.push(Trigger {
                    conditions: vec![Condition::from_str(&trigger_string)],
                });
            } else {
                break;
            }
            trigger_num += 1;
        }

        TriggerHandler {
            triggerall: trigger_all,
            triggers: triggers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_state_num() {
        let state_label = "state -1, crouching light kick";
        assert_eq!(CNSFile::get_state_num(state_label), -1);
    }
}
