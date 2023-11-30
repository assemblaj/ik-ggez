use std::collections::HashSet;
use std::ops::Index;

use super::state::StateType;
use crate::game::char::{CharState, CharSystem};
use crate::spec::{cmd::CommandList, constants::char_constants::*};
use evalexpr::*;
use ggez::Context;
use regex::Regex;

pub struct ExpressionContext {
    pub context: HashMapContext,
    pub screen_width: f32,
}

impl ExpressionContext {
    // will want to adjust this to take all sorts of constants later
    pub fn new(screen_width: f32, anim_set: HashSet<u64>) -> ExpressionContext {
        let mut context = HashMapContext::new();
        context.set_function(
            "ifelse".to_string(),
            Function::new(|args| {
                let arguments = args.as_tuple()?;

                if let Ok(condition_true) = arguments[0].as_boolean() {
                    if condition_true {
                        Ok(arguments[1].clone())
                    } else {
                        Ok(arguments[2].clone())
                    }
                } else {
                    Err(EvalexprError::expected_boolean(arguments[0].clone()))
                }
            }),
        );

        // Todo, take user's configuration and actually do a proper conversion for this
        context.set_function(
            "720p".to_string(),
            Function::new(move |argument| {
                let function_width: f32 = screen_width;
                if let Ok(expm) = argument.as_float() {
                    Ok(Value::Float(expm * (function_width as f64 / 1280.0)))
                } else if let Ok(expn) = argument.as_int() {
                    Ok(Value::Float(expn as f64 * (function_width as f64 / 1290.0)))
                } else {
                    Err(EvalexprError::expected_float(argument.clone()))
                }
            }),
        );

        context.set_function(
            "selfanimexist".to_string(),
            Function::new(move |argument| {
                let func_anim_set = anim_set.clone();
                if let Ok(expn) = argument.as_int() {
                    Ok(Value::Boolean(func_anim_set.contains(&(expn as u64))))
                } else {
                    Err(EvalexprError::expected_int(argument.clone()))
                }
            }),
        );

        Self {
            context,
            screen_width,
        }
    }

    pub fn insert_float(&mut self, identifier: &str, value: f64) {
        self.insert(identifier, Value::Float(value));
    }

    pub fn insert_int(&mut self, identifier: &str, value: i64) {
        self.insert(identifier, Value::Int(value));
    }

    pub fn insert_bool(&mut self, identifier: &str, value: bool) {
        self.insert(identifier, Value::Boolean(value));
    }

    pub fn insert_string(&mut self, identifier: &str, value: String) {
        self.insert(identifier, Value::String(value));
    }

    fn insert(&mut self, identifier: &str, value: Value) {
        self.context.set_value(identifier.to_string(), value);
    }

    pub fn set_char_constants(&mut self, constants: &CharConstants) {
        // init context with constants
        for (key, val) in constants.iter() {
            let value = match val {
                ConstantValue::Float(f) => Value::Float(f as f64),
                ConstantValue::Int(i) => Value::Int(i as i64),
            };
            let mut new_name: String = format!("{}", key).replace("\\", "").replace("\"", "");
            self.context.set_value(new_name, value);
        }
        dbg!(&self.context);
    }

    fn update_values(&mut self, char: &CharState) {
        self.context
            .set_value(ANIM_TRIGGER.to_string(), Value::Int(anim(char) as i64));
        self.context
            .set_value(ANIM_TIME_TRIGGER.to_string(), Value::Int(anim_time(char)));

        self.context
            .set_value(TIME_TRIGGER.to_string(), Value::Int(time(char) as i64));

        self.context
            .set_value(VEL_X_TRIGGER.to_string(), Value::Float(vel_x(char) as f64));
        self.context
            .set_value(VEL_Y_TRIGGER.to_string(), Value::Float(vel_y(char) as f64));

        self.context
            .set_value(POS_X_TRIGGER.to_string(), Value::Float(pos_x(char) as f64));
        self.context
            .set_value(POS_Y_TRIGGER.to_string(), Value::Float(pos_y(char) as f64));

        self.context
            .set_value(ALIVE_TRIGGER.to_string(), Value::Boolean(alive(char) != 0));

        self.context.set_value(
            STATENO_TRIGGER.to_string(),
            Value::Int(stateno(char) as i64),
        );
        self.context.set_value(
            STATE_TYPE_TRIGGER.to_string(),
            Value::String(state_type(char).to_string()),
        );

        self.context.set_value(
            IN_GAURD_DIST_TRIGGER.to_string(),
            Value::Boolean(in_gaurd_dist(char)),
        );

        self.context
            .set_value(CTRL_TRIGGER.to_string(), Value::Boolean(ctrl(char) != 0));

        self.context.set_value(
            PREV_STATE_NO_TRIGGER.to_string(),
            Value::Int(prev_state_no(char) as i64),
        );

        self.context.set_value(
            MOVE_CONTACT_TRIGGER.to_string(),
            Value::Boolean(move_contact(char) != 0),
        );

        self.context.set_value(ANIM_ELEM_TRIGGER.to_string(), Value::Int(anim_elem(char) as i64)); 
    }

    fn update_commands(&mut self, char: &CharState, command_list: &CommandList) {
        for cmd in &command_list.commands {
            let mut new_name: String = format!("command_{}", cmd.name).replace("\"", "");
            self.context
                .set_value(new_name, Value::Boolean(command(char, &cmd.name)));
        }
    }

    fn update_vars(&mut self, char: &CharState) {
        for i in 0..6 {
            self.context.set_value(
                format!("sysvar_{:?}", i as usize),
                Value::Int(char.sys_var(i as usize) as i64),
            );
        }
        for i in 0..60 {
            self.context.set_value(
                format!("var_{}", i as usize),
                Value::Int(char.get_int_var(i as usize) as i64),
            );

            self.context.set_value(
                format!("fvar_{}", i as usize),
                Value::Float(char.get_flaot_var(i as usize) as f64),
            );
        }
    }
    pub fn mid_frame_update(&mut self, char: &CharState) {
        self.update_values(char);
        self.update_vars(char);
    }

    pub fn update(&mut self, char: &CharState) {
        self.update_values(char);
        self.update_commands(char, &char.command_list);
        self.update_vars(char);
    }
}

const ANIM_TRIGGER: &str = "anim";
pub fn anim(char: &CharState) -> u64 {
    char.get_anim_no()
}

const ANIM_TIME_TRIGGER: &str = "animtime";
pub fn anim_time(char: &CharState) -> i64 {
    char.get_anim_time()
}

const TIME_TRIGGER: &str = "time";
pub fn time(char: &CharState) -> i32 {
    char.get_state_time()
}

pub fn abs<T>(val: T) -> T
where
    T: PartialOrd + std::ops::Neg<Output = T> + Copy + Default,
{
    if val < T::default() {
        -val
    } else {
        val
    }
}

const VEL_X_TRIGGER: &str = "vel_x";
pub fn vel_x(char: &CharState) -> f32 {
    char.get_velocity().0
}

const VEL_Y_TRIGGER: &str = "vel_y";
pub fn vel_y(char: &CharState) -> f32 {
    char.get_velocity().1
}

const POS_X_TRIGGER: &str = "pos_x";
pub fn pos_x(char: &CharState) -> f32 {
    char.get_position().0
}

const POS_Y_TRIGGER: &str = "pos_y";
pub fn pos_y(char: &CharState) -> f32 {
    char.get_position().1
}

const ALIVE_TRIGGER: &str = "alive";
pub fn alive(char: &CharState) -> i32 {
    char.get_alive()
}

const STATENO_TRIGGER: &str = "stateno";
pub fn stateno(char: &CharState) -> i32 {
    char.get_state_no()
}

const CONST_TRIGGER: &str = "const";
pub fn constant(name: &str, char: &CharSystem) -> Option<ConstantValue> {
    char.constant(name)
}

const IN_GAURD_DIST_TRIGGER: &str = "ingaurddist";
pub fn in_gaurd_dist(char: &CharState) -> bool {
    false
}

const STATE_TYPE_TRIGGER: &str = "statetype";
pub fn state_type(char: &CharState) -> StateType {
    char.get_state_type()
}

const CTRL_TRIGGER: &str = "ctrl";
pub fn ctrl(char: &CharState) -> i32 {
    char.get_ctrl()
}

const COMMAND_TRIGGER: &str = "command";
pub fn command(char: &CharState, name: &str) -> bool {
    char.command(name)
}

const SYS_VAR_TRIGGER: &str = "sysvar";
pub fn sys_var(char: &CharState, index: usize) -> i32 {
    char.sys_var(index)
}

const PREV_STATE_NO_TRIGGER: &str = "prevstateno";
pub fn prev_state_no(char: &CharState) -> i32 {
    char.get_prev_state_no()
}

const MOVE_CONTACT_TRIGGER: &str = "movecontact";
pub fn move_contact(char: &CharState) -> i32 {
    char.get_move_contact()
}

const ANIM_ELEM_TRIGGER: &str = "animelem"; 
pub fn anim_elem(char: &CharState) -> usize {
    char.get_anim_element()
}

#[derive(Clone)]
pub struct Expression {
    original_expression: String,
    expn: Node,
}

impl Expression {
    pub fn new(expn: &str) -> Self {
        Self {
            original_expression: expn.to_string(),
            expn: compile_espression(expn),
        }
    }

    pub fn evaluate_int(&self, ctx: &ExpressionContext) -> i32 {
        self.expn.eval_int_with_context(&ctx.context).unwrap() as i32
    }

    // This guy doesn't like "0" otherwise.
    pub fn evaluate_float(&self, ctx: &ExpressionContext) -> f32 {
        self.expn.eval_number_with_context(&ctx.context).unwrap() as f32
    }

    pub fn evaluate_boolean(&self, ctx: &ExpressionContext) -> bool {
        self.expn.eval_boolean_with_context(&ctx.context).unwrap()
    }

    pub fn evaluate_string(&self, ctx: &ExpressionContext) -> String {
        self.expn.eval_string_with_context(&ctx.context).unwrap()
    }
}

// Define a condition
pub struct Condition {
    original_expression: String,
    compiled_expression: Node,
}

const expression_map: [(&str, &str); 6] = [
    ("pos x", "pos_x"),
    ("pos y", "pos_y"),
    ("vel x", "vel_x"),
    ("vel y", "vel_y"),
    ("=", "=="),
    ("abs", "math::abs"),
];

fn convert_command_syntax(input: &str) -> String {
    let mut input_no_whitespace: String = input.split_whitespace().collect();
    input_no_whitespace.replace("=", "_").replace("\"", "") //+ ")"
}

fn convert_const_syntax(input: &str) -> String {
    input.replace("const", "")
}

fn convert_command_negation_syntax(input: &str) -> String {
    let input: String = input.split_whitespace().collect();
    let mut result = input
        .replace("command!=", "!command")
        .replace("\"", "")
        .replace(" ", "");

    // Find the position right after "!command" to insert "("
    if let Some(pos) = result.find("!command") {
        result.insert(pos + "!command".len(), '_');
    }

    result //+ ")"
}

pub fn sanitize_expression(exp: &str) -> String {
    let mut result = String::from(exp);

    let range_syntax_regex = Regex::new(r"(\w+)\s*=\s*\[(\d+),(\d+)\]").unwrap();
    result = range_syntax_regex
        .replace_all(&result, "${1} > $2 && ${1} < $3")
        .to_string();

    let patterns = vec![
        ("sysvar\\((\\d+)\\)", "sysvar_$1"),
        ("var\\((\\d+)\\)", "var_$1"),
        ("fvar\\((\\d+)\\)", "fvar_$1"),
    ];

    for (pattern, replacement) in patterns {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, replacement).into_owned();
    }

    if result.contains("command =") {
        result = convert_command_syntax(&result);
    } else if result.contains("command !=") {
        result = convert_command_negation_syntax(&result);
    //} else if result.split_whitespace().collect::<String>().eq("1") {
    //    result = String::from("true");
    } else {
        result = result.to_lowercase();
    }

    if result.contains("const") {
        result = convert_const_syntax(&result);
    }

    let mut new_val: String = result.split_whitespace().collect();
    if new_val.contains("=") {
        let index = new_val.find("=").unwrap();
        if new_val.chars().nth(index + 1).unwrap().is_alphabetic() {
            new_val.insert(index + 1, '\"');
            new_val.push('\"');
            result = new_val;

            if !result.contains("!=") {
                let index = result.find("=").unwrap();
                result.insert(index, ' ');
                let index = result.find("\"").unwrap();
                result.insert(index, ' ');
                dbg!(&result);
            }
        }
    }

    for (from, to) in expression_map {
        result = result.replace(from, to);
    }
    result = result.replace("!==", "!=");
    result = result.replace(">==", ">=");
    dbg!(&result);
    result
}

pub fn compile_espression(exp: &str) -> Node {
    let sanitized_exp = sanitize_expression(exp);
    build_operator_tree(&sanitized_exp).unwrap()
}

impl Condition {
    pub fn from_str(exp: &str) -> Condition {
        let orig = exp.to_string();
        let compiled_exp = compile_espression(&orig);
        Condition {
            original_expression: orig,
            compiled_expression: compiled_exp,
        }
    }

    pub fn evaluate(&self, ctx: &ExpressionContext) -> bool {
        let result = self
            .compiled_expression
            .eval_with_context(&ctx.context)
            .unwrap();
        match result {
            Value::Boolean(val) => return val,
            Value::Float(val) => return val != 0.0,
            Value::Int(val) => return val != 0,
            _ => result.as_boolean().unwrap(), // just panic bruh, for now
        }

        // if self.original_expression.contains("holddown") || self.original_expression.contains("statetype = C") || self.original_expression.contains("= \"x") || self.original_expression.contains("ctrl") || self.original_expression.contains("AnimTime = 0") {
        //     dbg!(&self.original_expression);
        //     dbg!(&result);
        // }
        // dbg!(&self.original_expression);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expression evaluation
    #[test]
    /*
       trigger1 = statetype != A
       trigger1 = ctrl
       trigger2 = (stateno = [200,299]) || (stateno = [400,499])
       trigger2 = stateno != 440
       trigger2 = movecontact
       trigger3 = stateno = 1310 || stateno = 1330
    */
    fn test_range() {
        //context.insert_int(identifier, value)
        let original = "(stateno = [200,299]) || (stateno = [400,499])";
        assert_eq!(
            sanitize_expression(original),
            String::from("(stateno > 200 && stateno < 299) || (stateno > 400 && stateno < 499)")
        )
    }

    #[test]
    fn test_var_sanitization() {
        let original = "sysvar(1) = 1";
        let result = "sysvar_1 == 1";
        assert_eq!(sanitize_expression(original), result.to_string())
    }

    #[test]
    fn test_command_eq_conversion() {
        let original = "command = \"holdfwd\"";
        let new = String::from("command_holdfwd");

        assert_eq!(convert_command_syntax(original), new);
    }

    #[test]
    fn test_command_neq_conversiont() {
        let original = "command != \"holdfwd\"";
        let new = String::from("!command_holdfwd");

        assert_eq!(convert_command_negation_syntax(original), new);
    }

    #[test]
    fn test_command_eq_with_spaces() {
        let original = "command  =    \"holdfwd\"";
        let new = String::from("command_holdfwd");

        assert_eq!(convert_command_syntax(original), new);
    }

    #[test]
    fn test_command_neq_with_spaces() {
        let original = "command     !=         \"holdfwd\"";
        let new = String::from("!command_holdfwd");

        assert_eq!(convert_command_negation_syntax(original), new);
    }
}
