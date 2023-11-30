use std::collections::HashMap;

use crate::{
    game::char::CharState,
    utils::ini::{Ini, IniSection},
};

use super::triggers::{Expression, ExpressionContext};
use evalexpr::{Node, ValueType};

const INVALID_TYPE_FOR_ARGS_ERR: &'static str = "invalid type for state args";

pub type StateController = Box<StateControllerFunc>;

pub type StateControllerFunc = dyn Fn(&mut CharState, StateArgs, &ExpressionContext);

pub fn get_controller(state_name: &str) -> StateController {
    match state_name {
        n if n == NULL_SCTRL => Box::new(null),
        n if n == CHANGE_STATE_SCTRL => Box::new(change_state),
        n if n == VEL_SET_SCTRL => Box::new(vel_set),
        n if n == VEL_MUL_SCTRL => Box::new(vel_mul),
        n if n == VEL_ADD_SCTRL => Box::new(vel_add),
        n if n == POS_SET_SCTRL => Box::new(pos_set),
        n if n == POS_ADD_SCTRL => Box::new(pos_add),
        n if n == CTRL_SET_SCTRL => Box::new(ctrl_set),
        n if n == CHANGE_ANIM_SCTRL => Box::new(change_anim),
        n if n == VAR_SET_SCTRL => Box::new(var_set),
        _ => {
            eprintln!("unknown sctrl");
            Box::new(null)
        }
    }
}

#[derive(Clone)]
pub enum StateArgs {
    Null,
    ChangeState(ChangeStateArgs),
    VelArgs(VelArgs),
    PosArgs(PosArgs),
    CtrlSet(i32),
    ChangeAnim(ChangeAnimArgs),
    VarSet(VarSetArgs),
}

impl StateArgs {
    const VALUE_ARG: &str = "value";
    const CTRL_ARG: &str = "ctrl";
    const ANIM_ARG: &str = "anim";
    const ELEM_ARG: &str = "elem";
    const X_ARG: &str = "x";
    const Y_ARG: &str = "y";

    pub fn new(name: &str, ini: &IniSection) -> Self {
        match name {
            n if n == NULL_SCTRL => StateArgs::Null,
            n if n == CHANGE_STATE_SCTRL => Self::change_state_args(ini),
            n if n == VEL_SET_SCTRL || n == VEL_MUL_SCTRL || n == VEL_ADD_SCTRL => {
                Self::vel_args(ini)
            }
            n if n == POS_SET_SCTRL || n == POS_ADD_SCTRL => Self::pos_args(ini),
            n if n == CTRL_SET_SCTRL => Self::ctrl_set_args(ini),
            n if n == CHANGE_ANIM_SCTRL => Self::change_anim_args(ini),
            n if n == VAR_SET_SCTRL => Self::var_set_args(ini),
            _ => {
                eprintln!("unknown sctrl");
                StateArgs::Null
            }
        }
    }

    fn xy(ini: &IniSection) -> (Option<Expression>, Option<Expression>) {
        let mut x: Option<Expression> = None;
        let mut y: Option<Expression> = None;

        if let Some(x_value) = ini.get::<String>(Self::X_ARG) {
            x = Some(Expression::new(x_value.as_str()));
        }

        if let Some(y_value) = ini.get::<String>(Self::Y_ARG) {
            y = Some(Expression::new(y_value.as_str()));
        }

        (x, y)
    }

    fn vel_args(ini: &IniSection) -> Self {
        let (x, y) = Self::xy(ini);
        Self::VelArgs(VelArgs { x, y })
    }

    fn pos_args(ini: &IniSection) -> Self {
        let (x, y) = Self::xy(ini);

        Self::PosArgs(PosArgs { x, y })
    }

    fn change_state_args(ini: &IniSection) -> Self {
        let value = ini.get::<i32>(Self::VALUE_ARG).unwrap();
        let mut ctrl: Option<i32> = None;
        let mut anim: Option<i32> = None;

        if let Some(ctrl_flag) = ini.get::<i32>(Self::CTRL_ARG) {
            ctrl = Some(ctrl_flag);
        }

        if let Some(anim_no) = ini.get::<i32>(Self::ANIM_ARG) {
            anim = Some(anim_no);
        }

        Self::ChangeState(ChangeStateArgs {
            state_no: value,
            ctrl_flag: ctrl,
            anim_no: anim,
        })
    }

    fn ctrl_set_args(ini: &IniSection) -> Self {
        let value = ini.get::<i32>(Self::VALUE_ARG).unwrap();
        Self::CtrlSet(value)
    }

    fn change_anim_args(ini: &IniSection) -> Self {
        let anim_no = Expression::new(ini.get::<String>(Self::VALUE_ARG).unwrap().as_str());
        let mut elem: Option<Expression> = None;
        if let Some(elem_exp) = ini.get::<String>(Self::ELEM_ARG) {
            elem = Some(Expression::new(elem_exp.as_str()));
        }
        Self::ChangeAnim(ChangeAnimArgs { anim_no, elem })
    }

    const V_ARG: &str = "v";
    const VAR_ARG: &str = "var";
    const FV_ARG: &str = "fv";
    const FVAR_ARG: &str = "fvar";
    const SYSVAR_ARG: &str = "sysvar";
    fn var_set_args(ini: &IniSection) -> Self {
        if let Some(v) = ini.get::<i32>(Self::V_ARG) {
            let value_expn: String = ini.get(Self::VALUE_ARG).unwrap();
            return Self::VarSet(VarSetArgs::Int(v as usize, Expression::new(&value_expn)));
        } else if let Some(fv) = ini.get::<i32>(Self::FV_ARG) {
            let value_expn: String = ini.get(Self::VALUE_ARG).unwrap();
            return Self::VarSet(VarSetArgs::Float(fv as usize, Expression::new(&value_expn)));
        } else {
            for i in 0 as usize..60 as usize {
                let var_label = format!("{}({})", Self::VAR_ARG, i);
                let fvar_label = format!("{}({})", Self::FVAR_ARG, i);
                let sysvar_label = format!("{}({})", Self::SYSVAR_ARG, i);
                if let Some(int_expn) = ini.get::<String>(&var_label) {
                    return Self::VarSet(VarSetArgs::Int(i, Expression::new(&int_expn)));
                } else if let Some(float_expn) = ini.get::<String>(&fvar_label) {
                    return Self::VarSet(VarSetArgs::Float(i, Expression::new(&float_expn)));
                } else if let Some(sysvar_expn) = ini.get::<String>(&sysvar_label) {
                    return Self::VarSet(VarSetArgs::System(i, Expression::new(&sysvar_expn)));
                }
            }
            dbg!("Invalid VarSet paramater.");
            Self::Null
        }
    }
}
// Null.
const NULL_SCTRL: (&'static str) = ("null");
fn null(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) { /* No OP */
}

// Change State
#[derive(Clone)]
pub struct ChangeStateArgs {
    pub state_no: i32,
    pub ctrl_flag: Option<i32>,
    pub anim_no: Option<i32>,
}

impl TryFrom<StateArgs> for ChangeStateArgs {
    type Error = &'static str;
    fn try_from(args: StateArgs) -> Result<Self, Self::Error> {
        match args {
            StateArgs::ChangeState(c) => Ok(c),
            _ => Err(INVALID_TYPE_FOR_ARGS_ERR),
        }
    }
}

impl From<ChangeStateArgs> for StateArgs {
    fn from(args: ChangeStateArgs) -> StateArgs {
        StateArgs::ChangeState(args)
    }
}

const CHANGE_STATE_SCTRL: (&'static str) = ("changestate");
fn change_state(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: ChangeStateArgs = args.try_into().unwrap();
    char.set_state(args.state_no);
    if args.anim_no.is_some() {
        char.set_animation_no(args.anim_no.unwrap());
    }
    if args.ctrl_flag.is_some() {
        char.set_ctrl_flag(args.ctrl_flag.unwrap());
    }
}

// Vels
#[derive(Clone)]
pub struct VelArgs {
    x: Option<Expression>,
    y: Option<Expression>,
}

impl TryFrom<StateArgs> for VelArgs {
    type Error = &'static str;
    fn try_from(args: StateArgs) -> Result<Self, Self::Error> {
        match args {
            StateArgs::VelArgs(v) => Ok(v),
            _ => Err(INVALID_TYPE_FOR_ARGS_ERR),
        }
    }
}

impl From<VelArgs> for StateArgs {
    fn from(args: VelArgs) -> StateArgs {
        StateArgs::VelArgs(args)
    }
}

pub const VEL_SET_SCTRL: (&'static str) = ("velset");
fn vel_set(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: VelArgs = args.try_into().unwrap();
    if args.x.is_some() {
        char.velocity.x = args.x.unwrap().evaluate_float(ctx); 
    } 

    if args.y.is_some() {
        char.velocity.y = args.y.unwrap().evaluate_float(ctx); 
    } 
}

pub const VEL_MUL_SCTRL: (&'static str) = ("velmul");
fn vel_mul(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: VelArgs = args.try_into().unwrap();
    if args.x.is_some() {
        char.velocity.x *= args.x.unwrap().evaluate_float(ctx); 
    } 
    if args.y.is_some() {
        char.velocity.y *= args.y.unwrap().evaluate_float(ctx); 
    } 
}

pub const VEL_ADD_SCTRL: (&'static str) = ("veladd");
fn vel_add(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: VelArgs = args.try_into().unwrap();
    if args.x.is_some() {
        char.velocity.x += args.x.unwrap().evaluate_float(ctx); 
    } 
    if args.y.is_some() {
        char.velocity.y += args.y.unwrap().evaluate_float(ctx); 
    }
}

// Pos'
#[derive(Clone)]
pub struct PosArgs {
    x: Option<Expression>,
    y: Option<Expression>,
}

impl TryFrom<StateArgs> for PosArgs {
    type Error = &'static str;
    fn try_from(args: StateArgs) -> Result<Self, Self::Error> {
        match args {
            StateArgs::PosArgs(v) => Ok(v),
            _ => Err(INVALID_TYPE_FOR_ARGS_ERR),
        }
    }
}

impl From<PosArgs> for StateArgs {
    fn from(args: PosArgs) -> StateArgs {
        StateArgs::PosArgs(args)
    }
}

pub const POS_SET_SCTRL: (&'static str) = ("posset");
fn pos_set(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: PosArgs = args.try_into().unwrap();
    if args.x.is_some() {
        char.position.x = args.x.unwrap().evaluate_float(ctx); 
    } 
    if args.y.is_some() {
        char.position.y = args.y.unwrap().evaluate_float(ctx); 
    } 
}

pub const POS_ADD_SCTRL: (&'static str) = ("posadd");
fn pos_add(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: PosArgs = args.try_into().unwrap();
    if args.x.is_some() {
        char.position.x += args.x.unwrap().evaluate_float(ctx); 
    }
    if args.y.is_some() {
        char.position.y += args.y.unwrap().evaluate_float(ctx); 
    }
}

// Ctrl
pub const CTRL_SET_SCTRL: (&'static str) = ("ctrlset");
fn ctrl_set(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    char.ctrl_flag = match args {
        StateArgs::CtrlSet(flag) => flag,
        _ => char.ctrl_flag,
    };
    dbg!(char.ctrl_flag);
}

// Change Anim
#[derive(Clone)]
pub struct ChangeAnimArgs {
    anim_no: Expression,
    elem: Option<Expression>,
}

impl TryFrom<StateArgs> for ChangeAnimArgs {
    type Error = &'static str;
    fn try_from(args: StateArgs) -> Result<Self, Self::Error> {
        match args {
            StateArgs::ChangeAnim(args) => Ok(args),
            _ => Err(INVALID_TYPE_FOR_ARGS_ERR),
        }
    }
}

impl From<ChangeAnimArgs> for StateArgs {
    fn from(args: ChangeAnimArgs) -> StateArgs {
        StateArgs::ChangeAnim(args)
    }
}

pub const CHANGE_ANIM_SCTRL: (&'static str) = ("changeanim");
fn change_anim(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: ChangeAnimArgs = args.try_into().unwrap();
    let anim_no = args.anim_no.evaluate_int(ctx);
    char.set_animation_no(anim_no);
    if args.elem.is_some() {
        let elem = args.elem.unwrap().evaluate_int(ctx);
        char.set_animation_element(elem);
    }
}

// VarSet
#[derive(Clone)]
pub enum VarSetArgs {
    Int(usize, Expression),
    Float(usize, Expression),
    System(usize, Expression),
}

impl TryFrom<StateArgs> for VarSetArgs {
    type Error = &'static str;
    fn try_from(args: StateArgs) -> Result<Self, Self::Error> {
        match args {
            StateArgs::VarSet(args) => Ok(args),
            _ => Err(INVALID_TYPE_FOR_ARGS_ERR),
        }
    }
}

pub const VAR_SET_SCTRL: &'static str = "varset";
fn var_set(char: &mut CharState, args: StateArgs, ctx: &ExpressionContext) {
    let args: VarSetArgs = args.try_into().unwrap();
    match args {
        VarSetArgs::Int(idx, val) => {
            char.set_int_var(idx, val.evaluate_int(ctx))
        }
        VarSetArgs::Float(idx, fval) => char.set_float_var(idx, fval.evaluate_float(ctx)),
        VarSetArgs::System(idx, val) => {
            char.set_sys_var(idx, val.evaluate_int(ctx))
        }
    }
}
