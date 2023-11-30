use crate::utils::ini::*;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct CharConstants {
    float_constants: HashMap<String, f32>,
    int_constants: HashMap<String, i32>,
}

pub enum ConstantValue {
    Int(i32),
    Float(f32),
}

impl CharConstants {
    pub fn new() -> CharConstants {
        Self {
            float_constants: HashMap::new(),
            int_constants: HashMap::new(),
        }
    }

    pub fn iter(&self) -> ConstantIterator {
        ConstantIterator {
            float_iter: self.float_constants.iter(),
            int_iter: self.int_constants.iter(),
        }
    }

    pub fn insert_float(&mut self, name: &str, float: f32) {
        self.float_constants.insert(name.to_string(), float);
    }

    pub fn insert_int(&mut self, name: &str, int: i32) {
        self.int_constants.insert(name.to_string(), int);
    }

    pub fn get(&self, name: &str) -> Option<ConstantValue> {
        if let Some(&float_value) = self.float_constants.get(name) {
            Some(ConstantValue::Float(float_value))
        } else if let Some(&int_value) = self.int_constants.get(name) {
            Some(ConstantValue::Int(int_value))
        } else {
            None
        }
    }
}

pub struct ConstantIterator<'a> {
    float_iter: Iter<'a, String, f32>,
    int_iter: Iter<'a, String, i32>,
}

impl<'a> Iterator for ConstantIterator<'a> {
    type Item = (&'a String, ConstantValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, &value)) = self.float_iter.next() {
            Some((key, ConstantValue::Float(value)))
        } else {
            self.int_iter
                .next()
                .map(|(key, &value)| (key, ConstantValue::Int(value)))
        }
    }
}

// [Data]
const DATA_KEY: &str = "data";
const LIFE_KEY: &str = "life"; // Amount of life to start with
const POWER_KEY: &str = "power";
const ATTACK_KEY: &str = "attack"; // attack power (more is stronger)
const DEFENCE_KEY: &str = "defence"; // defensive power (more is stronger)
const FALL_DEFENCE_UP_KEY: &str = "fall.defence_up"; // Percentage to increase defense everytime player is knocked down
const LIE_DOWN_TIME_KEY: &str = "liedown.time"; // Time which player lies down for, before getting up
const AIR_JUGGLE_KEY: &str = "airjuggle"; // Number of points for juggling
const SPARK_NO_KEY: &str = "sparkno"; // Default hit spark number for HitDefs
const GAURD_SPARKNO_KEY: &str = "guard.sparkno"; // Default guard spark number
const KO_ECHO_KEY: &str = "ko.echo"; // 1 to enable echo on KO
const VOLUME_KEY: &str = "volume"; // Volume offset (negative for softer)

/*
    ;Variables with this index and above will not have their values
    ;reset to 0 between rounds or matches. There are 60 int variables,
    ;indexed from 0 to 59, and 40 float variables, indexed from 0 to 39.
    ;If omitted, then it defaults to 60 and 40 for integer and float
    ;variables repectively, meaning that none are persistent, i.e. all
    ;are reset. If you want your variables to persist between matches,
    ;you need to override state 5900 from common1.cns.
*/
const INT_PERSIST_INDEX_KEY: &str = "intpersistindex";
const FLOAT_PERSIST_INDEX_KEY: &str = "floatpersistindex";

const DATA_CONSTANT_SCHEMA: [(&str, ConstantValue); 13] = [
    (LIFE_KEY, ConstantValue::Int(1000)),
    (POWER_KEY, ConstantValue::Int(3000)),
    (ATTACK_KEY, ConstantValue::Int(100)),
    (DEFENCE_KEY, ConstantValue::Int(100)),
    (FALL_DEFENCE_UP_KEY, ConstantValue::Float(50.0)),
    (LIE_DOWN_TIME_KEY, ConstantValue::Int(60)),
    (AIR_JUGGLE_KEY, ConstantValue::Int(15)),
    (SPARK_NO_KEY, ConstantValue::Int(2)),
    (GAURD_SPARKNO_KEY, ConstantValue::Int(40)),
    (KO_ECHO_KEY, ConstantValue::Int(0)),
    (VOLUME_KEY, ConstantValue::Int(0)),
    (INT_PERSIST_INDEX_KEY, ConstantValue::Int(60)),
    (FLOAT_PERSIST_INDEX_KEY, ConstantValue::Int(40)),
];

// [Size]
const SIZE_KEY: &str = "size";
const X_SCALE_KEY: &str = "xscale"; // Horizontal scaling factor.
const Y_SCALE_KEY: &str = "yscale"; // Vertical scaling factor.
const GROUND_BACK_KEY: &str = "ground.back"; // Player width (back, ground)
const GROUND_FRONT_KEY: &str = "ground.front"; // Player width (front, ground)
const AIR_BACK_KEY: &str = "air.back"; // Player width (back, air)
const AIR_FRONT_KEY: &str = "air.front"; // Player width (front, air)
const HEIGHT_KEY: &str = "height"; // Height of player (for opponent to jump over)
const ATTACK_DIST_KEY: &str = "attack.dist"; // Default attack distance
const PROJ_ATTACK_DIST_KEY: &str = "proj.attack.dist"; // Default attack distance for projectiles
const PROJ_DO_SCALE_KEY: &str = "proj.doscale"; // Set to 1 to scale projectiles too
const HEAD_POS_KEY: &str = "head.pos"; // Approximate position of head
const HEAD_POS_X_KEY: &str = "head.pos.x"; // Approximate position of head
const HEAD_POS_Y_KEY: &str = "head.pos.y"; // Approximate position of head
const MID_POS_KEY: &str = "mid.pos"; // Approximate position of midsection
const MID_POS_X_KEY: &str = "mid.pos.x"; // Approximate position of midsection
const MID_POS_Y_KEY: &str = "mid.pos.y"; // Approximate position of midsection
const SHADOW_OFFSET_KEY: &str = "shadowoffset"; // Number of pixels to vertically offset the shadow
const DRAW_OFFSET_KEY: &str = "draw.offset"; // Player drawing offset in pixels (x, y). Recommended 0,0
const DRAW_OFFSET_X_KEY: &str = "draw.offset.x"; // Player drawing offset in pixels (x, y). Recommended 0,0
const DRAW_OFFSET_Y_KEY: &str = "draw.offset.y"; // Player drawing offset in pixels (x, y). Recommended 0,0

const SIZE_CONSTANT_SCHEMA: [(&str, ConstantValue); 11] = [
    (X_SCALE_KEY, ConstantValue::Float(1.0)),
    (Y_SCALE_KEY, ConstantValue::Float(1.0)),
    (GROUND_BACK_KEY, ConstantValue::Int(15)),
    (GROUND_FRONT_KEY, ConstantValue::Int(16)),
    (AIR_BACK_KEY, ConstantValue::Int(12)),
    (AIR_FRONT_KEY, ConstantValue::Int(12)),
    (HEIGHT_KEY, ConstantValue::Int(60)),
    (ATTACK_DIST_KEY, ConstantValue::Int(160)),
    (PROJ_ATTACK_DIST_KEY, ConstantValue::Int(90)),
    (PROJ_DO_SCALE_KEY, ConstantValue::Int(0)),
    (SHADOW_OFFSET_KEY, ConstantValue::Int(0)),
];

const SIZE_XY_CONSTANT_SCHEMA: [(&str, (ConstantValue, ConstantValue)); 3] = [
    (
        MID_POS_KEY,
        (ConstantValue::Int(-20), ConstantValue::Int(-240)),
    ),
    (
        HEAD_POS_KEY,
        (ConstantValue::Int(-20), ConstantValue::Int(-240)),
    ),
    (
        DRAW_OFFSET_KEY,
        (ConstantValue::Int(0), ConstantValue::Int(0)),
    ),
];

// [Velocity]
const VELOCITY_KEY: &str = "velocity";
const WALK_FWD_KEY: &str = "walk.fwd"; // Walk forward
const WALK_BACK_KEY: &str = "walk.back"; // Walk backward
const RUN_FWD_KEY: &str = "run.fwd"; // Run forward (x, y)
const RUN_BACK_KEY: &str = "run.back"; // Hop backward (x, y)
const JUMP_NEU_KEY: &str = "jump.neu"; // Neutral jumping velocity (x, y)
const JUMP_BACK_KEY: &str = "jump.back"; // Jump back Speed (x, y)
const JUMP_FWD_KEY: &str = "jump.fwd"; // Jump forward Speed (x, y)
const RUNJUMP_BACK_KEY: &str = "runjump.back"; // Running jump speeds (opt)
const RUNJUMP_FWD_KEY: &str = "runjump.fwd"; //
const AIRJUMP_NEU_KEY: &str = "airjump.neu"; //
const AIRJUMP_BACK_KEY: &str = "airjump.back"; // Air jump speeds (opt)
const AIRJUMP_FWD_KEY: &str = "airjump.fwd"; //
const AIR_GETHIT_GROUNDRECOVER_KEY: &str = "air.gethit.groundrecover"; // Velocity for ground recovery state (x, y)
const AIR_GETHIT_AIRRECOVER_MUL_KEY: &str = "air.gethit.airrecover.mul"; // Multiplier for air recovery velocity (x, y)
const AIR_GETHIT_AIRRECOVER_ADD_KEY: &str = "air.gethit.airrecover.add"; // Velocity offset for air recovery (x, y)
const AIR_GETHIT_AIRRECOVER_BACK_KEY: &str = "air.gethit.airrecover.back"; // Extra x-velocity for holding back during air recovery
const AIR_GETHIT_AIRRECOVER_FWD_KEY: &str = "air.gethit.airrecover.fwd"; // Extra x-velocity for holding forward during air recovery
const AIR_GETHIT_AIRRECOVER_UP_KEY: &str = "air.gethit.airrecover.up"; // Extra y-velocity for holding up during air recovery
const AIR_GETHIT_AIRRECOVER_DOWN_KEY: &str = "air.gethit.airrecover.down"; // Extra y-velocity for holding down during air recovery

const VELOCITY_CONSTANT_SCHEMA: [(&str, ConstantValue); 4] = [
    (AIR_GETHIT_AIRRECOVER_BACK_KEY, ConstantValue::Float(-4.0)),
    (AIR_GETHIT_AIRRECOVER_FWD_KEY, ConstantValue::Float(0.0)),
    (AIR_GETHIT_AIRRECOVER_UP_KEY, ConstantValue::Float(-8.0)),
    (AIR_GETHIT_AIRRECOVER_DOWN_KEY, ConstantValue::Float(6.0)),
];

const VELOCITY_XY_CONSTANT_SCHEMA: [(&str, (ConstantValue, ConstantValue)); 9] = [
    (
        RUN_FWD_KEY,
        (ConstantValue::Float(18.4), ConstantValue::Float(0.0)),
    ),
    (
        RUN_BACK_KEY,
        (ConstantValue::Float(-18.0), ConstantValue::Float(-15.2)),
    ),
    (
        AIR_GETHIT_GROUNDRECOVER_KEY,
        (ConstantValue::Float(-0.6), ConstantValue::Float(-14.0)),
    ),
    (
        AIR_GETHIT_AIRRECOVER_MUL_KEY,
        (ConstantValue::Float(0.5), ConstantValue::Float(0.2)),
    ),
    (
        AIR_GETHIT_AIRRECOVER_ADD_KEY,
        (ConstantValue::Float(0.0), ConstantValue::Float(-18.0)),
    ),
    (
        JUMP_BACK_KEY,
        (ConstantValue::Float(-10.2), ConstantValue::Float(-33.6)),
    ),
    (
        JUMP_FWD_KEY,
        (ConstantValue::Float(10.0), ConstantValue::Float(-33.6)),
    ),
    (
        RUNJUMP_BACK_KEY,
        (ConstantValue::Float(-10.2), ConstantValue::Float(-32.4)),
    ),
    (
        RUNJUMP_FWD_KEY,
        (ConstantValue::Float(16.0), ConstantValue::Float(-32.4)),
    ),
];

const VELOCITY_X_CONSTANT_SCHEMA: [(&str, ConstantValue); 4] = [
    (WALK_FWD_KEY, ConstantValue::Float(9.6)),
    (WALK_BACK_KEY, ConstantValue::Float(-8.8)),
    (AIRJUMP_BACK_KEY, ConstantValue::Float(-10.2)),
    (AIRJUMP_FWD_KEY, ConstantValue::Float(10.0)),
];

const VELOCITY_NEU_CONSTANT_SCHEMA: [(&str, (ConstantValue, ConstantValue)); 2] = [
    (
        JUMP_NEU_KEY,
        (ConstantValue::Float(0.0), ConstantValue::Float(-33.6)),
    ),
    (
        AIRJUMP_NEU_KEY,
        (ConstantValue::Float(0.0), ConstantValue::Float(-32.4)),
    ),
];

// [Movement]
const MOVEMENT_KEY: &str = "movement";
const AIRJUMP_NUM_KEY: &str = "airjump.num"; // Number of air jumps allowed (opt)
const AIRJUMP_HEIGHT_KEY: &str = "airjump.height"; // Minimum distance from ground before you can air jump (opt)
const YACCEL_KEY: &str = "yaccel"; // Vertical acceleration
const STAND_FRICTION_KEY: &str = "stand.friction"; // Friction coefficient when standing
const CROUCH_FRICTION_KEY: &str = "crouch.friction"; // Friction coefficient when crouching
const STAND_FRICTION_THRESHOLD_KEY: &str = "stand.friction.threshold"; // Threshold for standing friction
const CROUCH_FRICTION_THRESHOLD_KEY: &str = "crouch.friction.threshold"; // Threshold for crouching friction
const AIR_GETHIT_GROUNDLEVEL_KEY: &str = "air.gethit.groundlevel"; // Y-position for falling player hit the ground
const AIR_GETHIT_GROUNDRECOVER_GROUND_THRESHOLD_KEY: &str =
    "air.gethit.groundrecover.ground.threshold"; // Y-position for ground recovery command
const AIR_GETHIT_GROUNDRECOVER_GROUNDLEVEL_KEY: &str = "air.gethit.groundrecover.groundlevel"; // Y-position for ground recovery state touch ground
const AIR_GETHIT_AIRRECOVER_THRESHOLD_KEY: &str = "air.gethit.airrecover.threshold"; // Y-velocity above which player may use the air recovery command
const AIR_GETHIT_AIRRECOVER_YACCEL_KEY: &str = "air.gethit.airrecover.yaccel"; // Vertical acceleration for air recovery state
const AIR_GETHIT_TRIP_GROUNDLEVEL_KEY: &str = "air.gethit.trip.groundlevel"; // Y-position at which player in the tripped state touches the ground
const DOWN_BOUNCE_OFFSET_KEY: &str = "down.bounce.offset"; // Offset for player bouncing off the ground (x, y)
const DOWN_BOUNCE_YACCEL_KEY: &str = "down.bounce.yaccel"; // Vertical acceleration for bouncing off the ground
const DOWN_BOUNCE_GROUNDLEVEL_KEY: &str = "down.bounce.groundlevel"; // Y-position at which player bouncing off the ground touches the ground again **MUGEN 1.0**
const DOWN_FRICTION_THRESHOLD_KEY: &str = "down.friction.threshold"; // If the player's speed drops below this threshold while lying down, stop his movement **MUGEN 1.0**

const MOVEMENT_CONSTANT_SCHEMA: [(&str, ConstantValue); 16] = [
    (AIRJUMP_NUM_KEY, ConstantValue::Int(1)),
    (AIRJUMP_HEIGHT_KEY, ConstantValue::Int(140)),
    (YACCEL_KEY, ConstantValue::Float(1.76)),
    (STAND_FRICTION_KEY, ConstantValue::Float(0.85)),
    (CROUCH_FRICTION_KEY, ConstantValue::Float(0.82)),
    (STAND_FRICTION_THRESHOLD_KEY, ConstantValue::Float(8.0)),
    (CROUCH_FRICTION_THRESHOLD_KEY, ConstantValue::Float(0.2)),
    (AIR_GETHIT_GROUNDLEVEL_KEY, ConstantValue::Float(100.0)),
    (
        AIR_GETHIT_GROUNDRECOVER_GROUND_THRESHOLD_KEY,
        ConstantValue::Float(-80.0),
    ),
    (
        AIR_GETHIT_GROUNDRECOVER_GROUNDLEVEL_KEY,
        ConstantValue::Float(40.0),
    ),
    (
        AIR_GETHIT_AIRRECOVER_THRESHOLD_KEY,
        ConstantValue::Float(-4.0),
    ),
    (AIR_GETHIT_AIRRECOVER_YACCEL_KEY, ConstantValue::Float(1.4)),
    (AIR_GETHIT_TRIP_GROUNDLEVEL_KEY, ConstantValue::Float(60.0)),
    (DOWN_BOUNCE_YACCEL_KEY, ConstantValue::Float(1.6)),
    (DOWN_BOUNCE_GROUNDLEVEL_KEY, ConstantValue::Float(48.0)),
    (DOWN_FRICTION_THRESHOLD_KEY, ConstantValue::Float(0.2)),
];
const MOVEMENT_XY_CONSTANT_SCHEMA: [(&str, (ConstantValue, ConstantValue)); 1] = [(
    DOWN_BOUNCE_OFFSET_KEY,
    (ConstantValue::Float(0.0), ConstantValue::Float(80.0)),
)];

fn parse_int_constant(
    ini: &Ini,
    section: &str,
    item: &str,
    default: i32,
    constants: &mut CharConstants,
) {
    let item_val = ini.get_int(section, item).unwrap_or(default);
    constants.insert_int(&format!("{:?}.{:?}", section, item), item_val);
}

fn parse_float_constant(
    ini: &Ini,
    section: &str,
    item: &str,
    default: f32,
    constants: &mut CharConstants,
) {
    let item_val = ini.get_float(section, item).unwrap_or(default as f64);
    constants.insert_float(&format!("{:?}.{:?}", section, item), item_val as f32);
}

fn parse_constant(
    ini: &Ini,
    section: &str,
    item: (&str, ConstantValue),
    constants: &mut CharConstants,
) {
    match item.1 {
        ConstantValue::Float(default) => {
            parse_float_constant(ini, section, item.0, default, constants);
        }
        ConstantValue::Int(default) => {
            parse_int_constant(ini, section, item.0, default, constants);
        }
    }
}

fn parse_tuple_constant(
    ini: &Ini,
    section: &str,
    item: (&str, (ConstantValue, ConstantValue)),
    suffix: (&str, &str),
    constants: &mut CharConstants,
) {
    match item.1 {
        (ConstantValue::Float(def1), ConstantValue::Float(def2)) => {
            parse_float_tuple_constant(ini, section, item.0, (def1, def2), suffix, constants);
        }
        (ConstantValue::Int(def1), ConstantValue::Int(def2)) => {
            parse_int_tuple_constant(ini, section, item.0, (def1, def2), suffix, constants);
        }
        // This shouldn't happen
        // Could probably create another enum to avoid doing this
        (ConstantValue::Float(def1), ConstantValue::Int(def2)) => {
            parse_float_tuple_constant(
                ini,
                section,
                item.0,
                (def1, def2 as f32),
                suffix,
                constants,
            );
        }
        (ConstantValue::Int(def1), ConstantValue::Float(def2)) => {
            parse_int_tuple_constant(ini, section, item.0, (def1, def2 as i32), suffix, constants);
        }
    }
}

fn parse_float_tuple_constant(
    ini: &Ini,
    section: &str,
    item: &str,
    default: (f32, f32),
    suffix: (&str, &str),
    constants: &mut CharConstants,
) {
    let vals = ini.get_float_tuple(section, item).unwrap_or(default);
    constants.insert_float(
        &format!("{:?}.{:?}.{:?}", section, item, suffix.0),
        vals.0 as f32,
    );
    constants.insert_float(
        &format!("{:?}.{:?}.{:?}", section, item, suffix.1),
        vals.1 as f32,
    );
}

fn parse_int_tuple_constant(
    ini: &Ini,
    section: &str,
    item: &str,
    default: (i32, i32),
    suffix: (&str, &str),
    constants: &mut CharConstants,
) {
    let vals = ini.get_int_tuple(section, item).unwrap_or(default);
    constants.insert_int(&format!("{:?}.{:?}.{:?}", section, item, suffix.0), vals.0);
    constants.insert_int(&format!("{:?}.{:?}.{:?}", section, item, suffix.1), vals.1);
}

fn parse_constant_with_suffix(
    ini: &Ini,
    section: &str,
    suffix: &str,
    item: (&str, ConstantValue),
    constants: &mut CharConstants,
) {
    parse_constant(
        ini,
        section,
        (format!("{:?}.{:?}", item.0, suffix).as_str(), item.1),
        constants,
    )
}

fn parse_xy_constant(
    ini: &Ini,
    section: &str,
    item: (&str, (ConstantValue, ConstantValue)),
    constants: &mut CharConstants,
) {
    parse_tuple_constant(ini, section, item, ("x", "y"), constants)
}

fn parse_data_constants(ini: &Ini, constants: &mut CharConstants) {
    for constant in DATA_CONSTANT_SCHEMA {
        parse_constant(ini, DATA_KEY, constant, constants)
    }
}

fn parse_size_constants(ini: &Ini, constants: &mut CharConstants) {
    for constant in SIZE_CONSTANT_SCHEMA {
        parse_constant(ini, SIZE_KEY, constant, constants)
    }
    for constant in SIZE_XY_CONSTANT_SCHEMA {
        parse_xy_constant(ini, SIZE_KEY, constant, constants)
    }
}

fn parse_velocity_neu_constants(ini: &Ini, constants: &mut CharConstants) {
    for constant in VELOCITY_NEU_CONSTANT_SCHEMA {
        let default = match constant.1 {
            (ConstantValue::Float(def1), ConstantValue::Float(def2)) => (def1, def2),
            _ => unreachable!(),
        };
        let vals = ini
            .get_float_tuple(VELOCITY_KEY, constant.0)
            .unwrap_or(default);
        let replaced = constant.0.replace(".neu", "");
        constants.insert_float(&format!("{:?}.{:?}.x", VELOCITY_KEY, constant.0), vals.0);
        constants.insert_float(&format!("{:?}.{:?}.y", VELOCITY_KEY, replaced), vals.1);
    }
}

fn parse_velocity_constants(ini: &Ini, constants: &mut CharConstants) {
    for constant in VELOCITY_CONSTANT_SCHEMA {
        parse_constant(ini, VELOCITY_KEY, constant, constants);
    }
    for constant in VELOCITY_XY_CONSTANT_SCHEMA {
        parse_xy_constant(ini, VELOCITY_KEY, constant, constants);
    }
    for constant in VELOCITY_X_CONSTANT_SCHEMA {
        parse_constant_with_suffix(ini, VELOCITY_KEY, "x", constant, constants)
    }
    parse_velocity_neu_constants(ini, constants);
}

fn parse_movement_constants(ini: &Ini, constants: &mut CharConstants) {
    for constant in MOVEMENT_CONSTANT_SCHEMA {
        parse_constant(ini, MOVEMENT_KEY, constant, constants);
    }
    for constant in MOVEMENT_XY_CONSTANT_SCHEMA {
        parse_xy_constant(ini, MOVEMENT_KEY, constant, constants);
    }
}

pub fn parse_char_constants(ini: &Ini) -> CharConstants {
    let mut constants = CharConstants::new();
    parse_data_constants(ini, &mut constants);
    parse_size_constants(ini, &mut constants);
    parse_velocity_constants(ini, &mut constants);
    parse_movement_constants(ini, &mut constants);
    constants
}
