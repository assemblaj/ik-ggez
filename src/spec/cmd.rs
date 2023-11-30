use crate::utils::ini::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::{cns::CNSFile, state::StateDef, triggers::ExpressionContext};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    a,
    b,
    c,
    x,
    y,
    z,
    s,
}

impl Button {
    pub const buttons: [Button; 7] = [
        Button::a,
        Button::b,
        Button::c,
        Button::x,
        Button::y,
        Button::z,
        Button::s,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonKind {
    Simultaneous(Vec<Button>),
    Single(Button),
}

impl FromStr for ButtonKind {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("+") {
            let button_chars: Vec<char> = s.trim().chars().filter(|&c| c != '+').collect();
            let mut buttons = Vec::new();
            for btn_char in button_chars {
                match Button::try_from(btn_char) {
                    Ok(btn) => {
                        buttons.push(btn);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            return Ok(Self::Simultaneous(buttons));
        } else {
            let btn_char = s.trim().chars().nth(0);
            if btn_char.is_none() {
                return Err("Invalid button");
            }
            match Button::try_from(btn_char.unwrap()) {
                Ok(btn) => Ok(Self::Single(btn)),
                Err(e) => Err(e),
            }
        }
    }
}

impl TryFrom<char> for Button {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' => Ok(Button::a),
            'b' => Ok(Button::b),
            'c' => Ok(Button::c),
            'x' => Ok(Button::x),
            'y' => Ok(Button::y),
            'z' => Ok(Button::z),
            's' => Ok(Button::s),
            _ => Err("Invalid Button"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    B,
    DB,
    D,
    DF,
    F,
    UF,
    U,
    UB,
}

impl Direction {
    fn four_way_match(&self, other: &Direction) -> bool {
        match (self, other) {
            (Direction::B, Direction::DB)
            | (Direction::B, Direction::UB)
            | (Direction::B, Direction::B)
            | (Direction::D, Direction::DB)
            | (Direction::D, Direction::DF)
            | (Direction::D, Direction::D)
            | (Direction::U, Direction::UB)
            | (Direction::U, Direction::UF)
            | (Direction::U, Direction::U)
            | (Direction::F, Direction::DF)
            | (Direction::F, Direction::UF)
            | (Direction::F, Direction::F) => true,
            _ => false,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DirectionKind {
    FourWay(Direction),
    Single(Direction),
}

impl FromStr for DirectionKind {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().starts_with("$") {
            let dir_str = &s[1..];
            match Direction::from_str(dir_str) {
                Ok(dir) => return Ok(DirectionKind::FourWay(dir)),
                Err(e) => return Err(e),
            }
        }
        match Direction::from_str(s.trim()) {
            Ok(dir) => Ok(DirectionKind::Single(dir)),
            Err(e) => Err(e),
        }
    }
}

// probably need to be lowercase to handle multiple cases
// wait, i have to NOT do that for commands, but my whole inisection is
// default case insensitive LOOOL
impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "B" => Ok(Direction::B),
            "DB" => Ok(Direction::DB),
            "D" => Ok(Direction::D),
            "DF" => Ok(Direction::DF),
            "F" => Ok(Direction::F),
            "UF" => Ok(Direction::UF),
            "U" => Ok(Direction::U),
            "UB" => Ok(Direction::UB),
            _ => Err("Invalid Button"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Direction(DirectionKind),
    Button(ButtonKind),
}

pub fn combine_buttons(buttons: HashSet<Button>) -> Key {
    let buttons_vec: Vec<Button> = buttons.into_iter().collect();

    if buttons_vec.len() > 1 {
        Key::Button(ButtonKind::Simultaneous(buttons_vec))
    } else {
        Key::Button(ButtonKind::Single(buttons_vec[0]))
    }
}

pub fn get_ordinal_pair(cardinals: (Direction, Direction)) -> Key {
    let dir = match cardinals {
        (Direction::D, Direction::B) => Direction::DB,
        (Direction::D, Direction::F) => Direction::DF,
        (Direction::U, Direction::F) => Direction::UF,
        (Direction::U, Direction::B) => Direction::UB,
        _ => cardinals.0,
    };

    Key::Direction(DirectionKind::Single(dir))
}

pub fn get_ordinals(cardinals: HashSet<Direction>) -> Key {
    let dir = if cardinals.contains(&Direction::D) && cardinals.contains(&Direction::B) {
        Direction::DB
    } else if cardinals.contains(&Direction::D) && cardinals.contains(&Direction::F) {
        Direction::DF
    } else if cardinals.contains(&Direction::U) && cardinals.contains(&Direction::F) {
        Direction::UF
    } else if cardinals.contains(&Direction::U) && cardinals.contains(&Direction::B) {
        Direction::UB
    } else {
        cardinals.into_iter().nth(0usize).unwrap()
    };
    Key::Direction(DirectionKind::Single(dir))
}

impl FromStr for Key {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ButtonKind::from_str(s) {
            Ok(bk) => Ok(Key::Button(bk)),
            Err(e) => match DirectionKind::from_str(s) {
                Ok(dk) => Ok(Key::Direction(dk)),
                Err(e) => Err("Invalid key or direction."),
            },
        }
    }
}

#[derive(Debug)]
struct Modifier {
    held: bool,
    released: bool,
    direction_4way: bool,
    simultaneous: bool,
    no_other_keys: bool,
    ticks: Option<u32>,
}

pub fn matches_sequence_exact(relevant_cmds: &[Element], cmd: &[Element]) -> bool {
    if cmd.is_empty() || relevant_cmds.len() < cmd.len() {
        return false;
    }
    for window_start in 0..=relevant_cmds.len() - cmd.len() {
        let window_end = window_start + cmd.len();
        let window = &relevant_cmds[window_start..window_end];

        if window
            .iter()
            .map(Element::get_key_pressed)
            .eq(cmd.iter().map(Element::get_key_pressed))
        {
            return true;
        }
    }

    false
}

fn get_dir_tuple(elems: &[Element]) -> Option<(Direction, Direction)> {
    let mut dir_tuple = None;
    let mut dir_1 = None;
    let mut dir_2 = None;

    for dir in elems {
        if dir.get_direction().is_some() {
            if dir_1 == None {
                dir_1 = Some(dir.get_direction().unwrap());
            } else if dir_2 == None {
                dir_2 = Some(dir.get_direction().unwrap());
            }
        }
    }

    if !(dir_1.is_some() && dir_2.is_some()) {
        return dir_tuple;
    }
    dir_tuple = Some((dir_1.unwrap(), dir_2.unwrap()));
    dir_tuple
}

// this is returning true to dp when qcf is attempted
// my thinking is that maybe i need reset total matched
pub fn matches_sequence(cmd: &Command, relevant_cmds: &[Element]) -> bool {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut last_matched: usize = 0;
    let mut total_matched = 0;
    while i < cmd.command.elements.len() && j < relevant_cmds.len() {
        if cmd.command.elements.get(i).unwrap().get_key() == relevant_cmds.get(j).unwrap().get_key()
        {
            last_matched = j;
            total_matched += 1;
            i += 1;
        } else if cmd.command.elements.get(i).unwrap().is_ordinal() {
            let total_pairs = (j - last_matched) / 2;
            let mut pairs_seen: usize = 0;
            while pairs_seen < total_pairs && (last_matched + (pairs_seen + 1) * 2) <= j {
                // let mut dir_set: HashSet<Direction> = HashSet::new();
                // for dir in &relevant_cmds[last_matched..last_matched + (pairs_seen+1)*2] {
                //     if dir.get_direction().is_some() {
                //         dir_set.insert(dir.get_direction().unwrap());
                //     }
                // }
                // if dir_set.len() > 0  {
                //     dbg!(&dir_set);
                // }
                let dir_tuple = get_dir_tuple(
                    &relevant_cmds[last_matched..last_matched + (pairs_seen + 1) * 2],
                );
                if dir_tuple.is_some()
                    && cmd
                        .command
                        .elements
                        .get(i)
                        .unwrap()
                        .get_key()
                        .unwrap()
                        .eq(&get_ordinal_pair(dir_tuple.unwrap()))
                {
                    last_matched = j;
                    total_matched += 1;
                    i += 1;
                    break;
                } else {
                    pairs_seen += 1;
                }

                // if dir_set.len() > 0
                //     && cmd
                //         .command
                //         .elements
                //         .get(i)
                //         .unwrap()
                //         .get_key()
                //         .unwrap()
                //         .eq(&get_ordinals(dir_set))
                // {
                //     last_matched = j;
                //     total_matched += 1;
                //     i += 1;
                //     break;
                // } else {
                //     pairs_seen += 1;
                // }
            }
        }
        j += 1;
    }
    return total_matched >= cmd.command.elements.len();
}

pub fn remove_inbetweens(cmds: Vec<Element>) -> Vec<Element> {
    let mut result = cmds.clone();
    if result.len() == 1 {
        return result;
    } else {
        let mut i = 0 as usize;
        let mut j: i32 = cmds.len() as i32 - 1;
        while j >= 0 {
            if cmds.get(j as usize).unwrap().get_key().is_none() {
                result.remove(j as usize);
            }
            j = j - 1;
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Element {
    Released(Key, u32),
    Held(Key),
    NoOtherKeys(Box<Element>),
    Pressed(Key),
    NoInput,
}

impl Element {
    fn no_other_keys(&self) -> bool {
        match self {
            Element::NoOtherKeys(_) => true,
            _ => false,
        }
    }

    pub fn get_key(&self) -> Option<&Key> {
        match self {
            Self::Released(key, _) => Some(key),
            Self::Held(key) => Some(key),
            Self::Pressed(key) => Some(key),
            Self::NoOtherKeys(elm) => elm.get_key(),
            _ => None,
        }
    }

    pub fn get_direction(&self) -> Option<Direction> {
        match self.get_key() {
            Some(Key::Direction(DirectionKind::Single(dir))) => Some(*dir),
            _ => None,
        }
    }

    pub fn get_key_pressed(&self) -> Option<&Key> {
        match self {
            Self::Pressed(key) => Some(key),
            //Self::NoOtherKeys(elm) => elm.get_key_pressed(),
            _ => None,
        }
    }

    pub fn is_ordinal(&self) -> bool {
        match self.get_key() {
            Some(Key::Direction(DirectionKind::Single(Direction::DB)))
            | Some(Key::Direction(DirectionKind::Single(Direction::DF)))
            | Some(Key::Direction(DirectionKind::Single(Direction::UB)))
            | Some(Key::Direction(DirectionKind::Single(Direction::UF))) => true,
            _ => false,
        }
    }

    pub fn is_four_way(&self) -> bool {
        match self.get_key() {
            Some(Key::Direction(DirectionKind::FourWay(dir))) => true,
            _ => false,
        }
    }

    pub fn four_way_match(&self, elem: &Element) -> bool {
        match self.get_key() {
            Some(Key::Direction(DirectionKind::FourWay(dir1))) => match elem.get_key() {
                Some(Key::Direction(DirectionKind::Single(dir2))) => dir1.four_way_match(dir2),
                _ => false,
            },
            _ => false,
        }
    }
}

pub struct CmdFile {
    ini: Ini,
    defaults: CmdDefaults,
}

struct CmdDefaults {
    time: i32,
    buffer_time: i32,
}

impl CmdFile {
    const DEFAULT_BUFFER_TIME: i32 = 1;
    const DEFAULT_TIME: i32 = 15;

    pub fn new(cmd_file_path: &str) -> CmdFile {
        let ini = load_ini(cmd_file_path);
        let defaults = Self::get_defaults(&ini);
        CmdFile { ini, defaults }
    }

    pub fn parse_states(&self) -> HashMap<i32, StateDef> {
        CNSFile::parse_states(&self.ini)
    }

    const DEFAULTS_KEY: &str = "defaults";
    const COMMAND_TIME_KEY: &str = "command.time";
    const COMMAND_BUFFER_TIME_KEY: &str = "command.buffer.time";
    fn get_defaults(ini: &Ini) -> CmdDefaults {
        CmdDefaults {
            time: ini
                .get_int(Self::DEFAULTS_KEY, Self::COMMAND_TIME_KEY)
                .unwrap_or(Self::DEFAULT_TIME),
            buffer_time: ini
                .get_int(Self::DEFAULTS_KEY, Self::COMMAND_BUFFER_TIME_KEY)
                .unwrap_or(Self::DEFAULT_BUFFER_TIME),
        }
    }

    fn get_commands(&self) -> Result<Vec<Command>, String> {
        let commands_ini = self
            .ini
            .get_section(Self::COMMAND_KEY)
            .ok_or("no commands".to_string())?;

        let commands = match commands_ini {
            SectionContainer::Multiple(commands) => commands
                .iter()
                .map(|ini| self.command_from_ini(ini))
                .collect(),
            SectionContainer::Single(command) => vec![self.command_from_ini(command)],
        };
        Ok(commands)
    }

    const COMMAND_KEY: &str = "command";
    const NAME_KEY: &str = "name";
    const TIME_KEY: &str = "time";
    const BUFFER_TIME_KEY: &str = "buffer.time";
    fn command_from_ini(&self, ini: &IniSection) -> Command {
        let name: String = ini.get::<String>(Self::NAME_KEY).unwrap().replace("\"", "");
        let sequence_str: String = ini.get(Self::COMMAND_KEY).unwrap();
        let sequence: Sequence = Sequence::from_str(&sequence_str).unwrap();

        let time: i32 = ini.get(Self::TIME_KEY).unwrap_or(self.defaults.time);
        let buffer_time: i32 = ini
            .get(Self::BUFFER_TIME_KEY)
            .unwrap_or(self.defaults.buffer_time);

        Command {
            name,
            command: sequence,
            time,
            buffer_time,
        }
    }
}

#[derive(Debug)]
pub struct CommandList {
    pub commands: Vec<Command>,
    default_time: u32,
    default_buffer_time: u32,
}

impl CommandList {
    const DEFAULT_BUFFER_TIME: u32 = 1;
    const DEFAULT_TIME: u32 = 15;

    pub fn new(cmd_file: &CmdFile) -> CommandList {
        // let ini = parse_cns(cmd_file);

        // let defaults = match ini.get("Defaults") {
        //     Some(v) => Some(v.get(0).unwrap()),
        //     None => None,
        // };
        let (default_time, default_buffer_time): (u32, u32) =
            (CommandList::DEFAULT_TIME, CommandList::DEFAULT_BUFFER_TIME);

        // if defaults.is_some() {
        //     // let defaults_map = defaults.unwrap();
        //     // (
        //     //     defaults_map
        //     //         .get("command.time")
        //     //         .unwrap()
        //     //         .parse()
        //     //         .unwrap_or(CommandList::DEFAULT_TIME),
        //     //     defaults_map
        //     //         .get("command.buffer.time")
        //     //         .unwrap()
        //     //         .parse()
        //     //         .unwrap_or(CommandList::DEFAULT_BUFFER_TIME),
        //     // )
        // } else {
        // };

        CommandList {
            commands: cmd_file.get_commands().unwrap(),
            default_time,
            default_buffer_time,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    pub name: String,
    pub command: Sequence,
    pub time: i32,
    pub buffer_time: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sequence {
    pub elements: Vec<Element>,
}

impl FromStr for Sequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = Vec::new();
        let symbol_strs: Vec<&str> = s.trim().split(',').collect();

        for symbol_str in symbol_strs {
            let mut chars = symbol_str.chars();
            let mut modifiers = Modifier {
                held: false,
                released: false,
                direction_4way: false,
                simultaneous: false,
                no_other_keys: false,
                ticks: None,
            };

            // Parsing modifiers
            let mut key_chars = Vec::new();
            while let Some(c) = chars.next() {
                match c {
                    '/' => modifiers.held = true,
                    '~' => {
                        let mut tick_str = String::new();
                        while let Some(d) = chars.next() {
                            if d.is_numeric() {
                                tick_str.push(d);
                            } else {
                                key_chars.push(d); // Capture the character that isn’t numeric.
                                break;
                            }
                        }
                        if !tick_str.is_empty() {
                            modifiers.ticks = tick_str.parse().ok();
                        }
                        modifiers.released = true;
                    }
                    '>' => modifiers.no_other_keys = true,
                    _ => {
                        key_chars.push(c); // Capture the character that isn’t a modifier.
                        break;
                    }
                }
            }

            key_chars.extend(chars); // Add the remaining characters to key_chars.
            let key_str: String = key_chars.into_iter().collect();
            let key = Key::from_str(&key_str).map_err(|_| "Invalid Key")?;
            let symbol = if modifiers.held {
                Element::Held(key)
            } else if modifiers.released {
                Element::Released(key, modifiers.ticks.unwrap_or_default())
            } else {
                Element::Pressed(key)
            };

            let symbol = if modifiers.no_other_keys {
                Element::NoOtherKeys(Box::new(symbol))
            } else {
                symbol
            };

            symbols.push(symbol);
        }

        Ok(Sequence { elements: symbols })
    }
}

/* impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = Vec::new();
        let symbol_strs: Vec<&str> = s.split(',').collect();

        for symbol_str in symbol_strs {

            let mut modifiers = Modifier {
                held: false,
                released: false,
                direction_4way: false,
                simultaneous: false,
                no_other_keys: false,
                ticks: None,
            };

            let mut chars = symbol_str.chars();

            // Parsing modifiers and keys
            let mut key_str = String::new();
            while let Some(c) = chars.next() {
                match c {
                    '/' => modifiers.held = true,
                    '~' => {
                        let mut tick_str = String::new();
                        while let Some(d) = chars.next() {
                            if d.is_numeric(){
                                tick_str.push(d);
                            } else {
                                key_str.push(d); // Preserve non-numeric character for key identification
                                break;
                            }
                        }
                        if !tick_str.is_empty() {
                            modifiers.ticks = tick_str.parse().ok();
                        }
                        modifiers.released = true;
                    }
                    '$' => modifiers.direction_4way = true,
                    '+' => modifiers.simultaneous = true,
                    '>' => modifiers.no_other_keys = true,
                    _ => key_str.push(c), // Collecting characters for key identification
                }
            }

            // After the loop, key_str should hold the necessary characters to identify the key
            let key = Key::from_str(&key_str);

            let mut symbol = if modifiers.simultaneous && key.is_err() {
                let mut buttons = Vec::new();

                for btn_char in key_str.trim().chars() {
                    let button = match Button::try_from(btn_char) {
                        Ok(btn) => btn,
                        Err(e) => return Err(format!("{:?} : {:?}", e, btn_char))
                    };
                    buttons.push(button);
                }

                Symbol::Simultaneous(buttons)
            } else if modifiers.held {
                Symbol::Held(key.unwrap())
            } else if modifiers.released {
                Symbol::Released(key.unwrap(), modifiers.ticks.unwrap_or_default())
            } else if modifiers.direction_4way {
                Symbol::FourWay(
                    match key.unwrap() {
                        Key::Direction(dir) => dir,
                        _ => return Err("Invalid Direction".to_string()),
                    }
                )
            } else {
                Symbol::Pressed(key.unwrap())
            };


            if modifiers.no_other_keys {
                symbol = Symbol::NoOtherKeys(Box::new(symbol));
            }

            symbols.push(symbol);
        }

        Ok(Command { symbols })
    }
}
 */
/*
; - command
;   list of buttons or directions, separated by commas. Each of these
;   buttons or directions is referred to as a "symbol".
;   Directions and buttons can be preceded by special characters:
;   slash (/) - means the key must be held down
;          egs. command = /D       ;hold the down direction
;               command = /DB, a   ;hold down-back while you press a
;   tilde (~) - to detect key releases
;          egs. command = ~a       ;release the a button
;               command = ~D, F, a ;release down, press fwd, then a
;          If you want to detect "charge moves", you can specify
;          the time the key must be held down for (in game-ticks)
;          egs. command = ~30a     ;hold a for at least 30 ticks, then release
;   dollar ($) - Direction-only: detect as 4-way
;          egs. command = $D       ;will detect if D, DB or DF is held
;               command = $B       ;will detect if B, DB or UB is held
;   plus (+) - Buttons only: simultaneous press
;          egs. command = a+b      ;press a and b at the same time
;               command = x+y+z    ;press x, y and z at the same time
;   greater-than (>) - means there must be no other keys pressed or released
;                      between the previous and the current symbol.
;          egs. command = a, >~a   ;press a and release it without having hit
;                                  ;or released any other keys in between
;   You can combine the symbols:
;     eg. command = ~30$D, a+b     ;hold D, DB or DF for 30 ticks, release,
;                                  ;then press a and b together
;
;   Note: Successive direction symbols are always expanded in a manner similar
;         to this example:
;           command = F, F
;         is expanded when MUGEN reads it, to become equivalent to:
;           command = F, >~F, >F
*/
