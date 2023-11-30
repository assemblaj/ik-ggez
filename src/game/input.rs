use ggez::{
    event,
    glam::*,
    graphics::{self, Color, ImageFormat},
    Context, GameResult,
};
use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use ggez::winit::event::VirtualKeyCode;

use crate::spec::cmd::{
    combine_buttons, get_ordinals, matches_sequence_exact, remove_inbetweens, matches_sequence, get_ordinal_pair,  Button, ButtonKind, Command,
    CommandList, Direction, DirectionKind, Element, Key,
};

pub struct InputSystem {
    pub button_map: HashMap<VirtualKeyCode, Button>,
    pub direction_map: HashMap<VirtualKeyCode, Direction>,
}

impl InputSystem {
    pub fn new() -> InputSystem {
        let dir_map = [
            // directions
            (VirtualKeyCode::Up, Direction::U),
            (VirtualKeyCode::Down, Direction::D),
            (VirtualKeyCode::Left, Direction::B),
            (VirtualKeyCode::Right, Direction::F),
        ];

        let btn_map = [
            // Buttons
            (VirtualKeyCode::A, Button::a),
            (VirtualKeyCode::S, Button::b),
            (VirtualKeyCode::D, Button::c),
            (VirtualKeyCode::Z, Button::x),
            (VirtualKeyCode::X, Button::y),
            (VirtualKeyCode::C, Button::z),
            (VirtualKeyCode::Space, Button::s),
        ];

        InputSystem {
            button_map: HashMap::from(btn_map),
            direction_map: HashMap::from(dir_map),
        }
    }

    fn update_buttons(&self, context: &mut Context) -> InputFrame {
        let mut pressed_buttons = HashSet::new();
        // let mut held_buttons: HashSet<_> = HashSet::new();
        let mut released_buttons = HashSet::new();

        for (&virtual_key, game_key) in &self.button_map {
            if context.keyboard.is_key_just_pressed(virtual_key) {
                pressed_buttons.insert(game_key.clone());
            // } else if context.keyboard.is_key_pressed(virtual_key) {
            //     held_buttons.insert(game_key.clone());
            } else if context.keyboard.is_key_just_released(virtual_key) {
                released_buttons.insert(game_key.clone());
            }
        }

        if !pressed_buttons.is_empty() {
            InputFrame::Pressed(combine_buttons(pressed_buttons))
        // } else if pressed_buttons.is_empty() && !held_buttons.is_empty() {
        //     InputFrame::Held(combine_buttons(held_buttons))
        } else if !released_buttons.is_empty() {
            InputFrame::Released(combine_buttons(released_buttons))
        } else {
            InputFrame::NoInput
        }
    }

    fn update_directions(&self, context: &mut Context) -> InputFrame {
        let mut pressed_dirs = HashSet::new();
        let mut held_dirs = HashSet::new();
        let mut released_dirs = HashSet::new();
        for (&virtual_key, game_key) in &self.direction_map {
            if context.keyboard.is_key_just_pressed(virtual_key) {
                pressed_dirs.insert(game_key.clone());
            } else if context.keyboard.is_key_just_released(virtual_key) {
                released_dirs.insert(game_key.clone());
            } else if context.keyboard.is_key_pressed(virtual_key) {
                held_dirs.insert(game_key.clone());
            }
        }

        if !pressed_dirs.is_empty() {
            InputFrame::Pressed(get_ordinals(pressed_dirs))
        } else if !released_dirs.is_empty() {
            InputFrame::Released(get_ordinals(released_dirs))
        } else if pressed_dirs.is_empty() && !held_dirs.is_empty() {
            InputFrame::Held(get_ordinals(held_dirs))
        } else {
            InputFrame::NoInput
        }
    }

    // Here's the rules.
    // Pressed takes priority over held, which takes priority over released
    // Buttons take priority over directions
    // If more than one direction is pressed, held or released, then
    // the first one seen (in order of the Direction Map (Up Down Left Right )) is
    // returned
    pub fn update(&self, context: &mut Context) -> InputFrame {
        let cur_button = self.update_buttons(context);
        // M.U.G.E.N.
        if cur_button.no_input() {
            let cur_direction = self.update_directions(context);
            return cur_direction;
        } else {
            return cur_button;
        }
    }
}

#[derive(Debug)]
pub enum InputFrame {
    Pressed(Key),
    Released(Key),
    Held(Key),
    NoInput,
}

impl InputFrame {
    pub fn no_input(&self) -> bool {
        match self {
            InputFrame::NoInput => true,
            _ => false,
        }
    }
}

pub struct InputState {
    input_buffer: Vec<Element>,
    seccess_buffer: SuccessBuffer,
}

struct SuccessBuffer {
    buffer: Vec<Command>,
    insert_times: Vec<u64>,
}

impl SuccessBuffer {
    pub fn new() -> SuccessBuffer {
        Self {
            buffer: Vec::new(),
            insert_times: Vec::new(),
        }
    }

    // if it's already in there, we are going to replace it with
    // the new one.
    pub fn insert(&mut self, command: Command, frame: u64) {
        if self.contains(&command) {
            self.remove(&command);
        }
        self.buffer.push(command);
        self.insert_times.push(frame);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Command> {
        self.buffer.iter()
    }

    pub fn contains(&self, command: &Command) -> bool {
        self.buffer.contains(command)
    }

    pub fn remove(&mut self, command: &Command) {
        let idx = self.buffer.iter().position(|cmd| command == cmd).unwrap();
        self.remove_at_index(idx);
    }

    fn remove_at_index(&mut self, idx: usize) {
        self.buffer.remove(idx);
        self.insert_times.remove(idx);
    }

    fn get_expired_command_indices(&self, frame: u64) -> Vec<usize> {
        self.buffer
            .iter()
            .zip(self.insert_times.iter())
            .enumerate()
            .filter_map(|(index, (command, &insert_time))| {
                // default buffer time ignored for hold only commands
                let buffer_time = if command.command.elements.len() == 1 {
                    match command.command.elements.first().unwrap() {
                        Element::Held(key) => 1,
                        _ => command.buffer_time,
                    }
                } else {
                    command.buffer_time
                };

                if frame as i32 - insert_time as i32 > buffer_time {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove_expired_commands(&mut self, frame: u64) {
        let mut to_remove = self.get_expired_command_indices(frame);
        for &idx in to_remove.iter().rev() {
            self.remove_at_index(idx);
        }
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            input_buffer: Vec::new(),
            seccess_buffer: SuccessBuffer::new(),
        }
    }

    // idea here is that we're converting individual key pressed/holds/releases
    // into something that can be used to determine if M.U.G.E.N commands were
    // activated.
    fn update_input_buffer(&mut self, frame: InputFrame) {
        if self.input_buffer.is_empty() {
            match frame {
                // InputFrame::Held(key) => self.input_buffer.push(Element::Held(key)),
                InputFrame::Pressed(key) => self.input_buffer.push(Element::Pressed(key)),
                InputFrame::Released(key) => self.input_buffer.push(Element::Released(key, 1)),
                _ => {
                    self.input_buffer.push(Element::NoInput);
                }
            }
        } else {
            if frame.no_input() && !self.input_buffer.last().unwrap().get_key().is_none() {
                self.input_buffer.push(Element::NoOtherKeys(Box::new(
                    self.input_buffer.last().unwrap().clone(),
                )));
            }
            match frame {
                // nputFrame::Held(key) => self.input_buffer.push(Element::Held(key)),
                InputFrame::Pressed(key) => {
                    self.input_buffer.push(Element::Pressed(key));
                }
                InputFrame::Released(key) => match self.input_buffer.last().unwrap() {
                    Element::Released(last_key, val) => {
                        if key == *last_key {
                            self.input_buffer.push(Element::Released(key, val + 1));
                        }
                    }
                    _ => {
                        self.input_buffer.push(Element::Released(key, 1));
                    }
                },
                InputFrame::Held(key) => {
                    self.input_buffer.push(Element::Held(key));
                }
                _ => {
                    self.input_buffer.push(Element::NoInput);
                }
            }
        }
    }

    pub fn update_success_buffer(&mut self, frame_no: i32, command_list: &CommandList) {
        self.seccess_buffer.remove_expired_commands(frame_no as u64);
        for command in &command_list.commands {
            if self.process_command(frame_no, command) {
                self.seccess_buffer.insert(command.clone(), frame_no as u64);
            }
        }
    }

    pub fn update(&mut self, frame_no: i32, frame: InputFrame, command_list: &CommandList) {
        self.update_input_buffer(frame);
        self.update_success_buffer(self.input_buffer.len() as i32, command_list);
    }

    pub fn command(&self, name: &str) -> bool {
        self.seccess_buffer
            .iter()
            .find(|cmd| cmd.name == name)
            .is_some()
    }

    pub fn buffered(&self, key: Key) -> bool {
        if self.input_buffer.last().is_some() {
            match self.input_buffer.last() {
                Some(Element::Pressed(buf_key)) => {
                    return key == *buf_key;
                }
                Some(Element::Held(buf_key)) => {
                    return key == *buf_key;
                }
                Some(Element::NoOtherKeys(buf_key)) => {
                    return key == *buf_key.get_key_pressed().unwrap()
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }

    pub fn pressing_any_button(&self) -> bool {
        for button in Button::buttons {
            if self.buffered_button(button) {
                return true;
            }
        }
        false
    }

    pub fn buffered_button(&self, button: Button) -> bool {
        self.buffered(Key::Button(ButtonKind::Single(button)))
    }

    pub fn buffered_key(&self, direction: Direction) -> bool {
        self.buffered(Key::Direction(DirectionKind::Single(direction)))
    }

    // TODO!: This still needs to detect when a FourWay could have been applicable.
    fn process_command(&self, frame_no: i32, cmd: &Command) -> bool {
        //if self.seccess_buffer.contains(cmd) {
        //    return true;
        //}
        let frames_ago = if self.input_buffer.len() < cmd.time as usize {
            0
        } else {
            self.input_buffer.len() - cmd.time as usize
        };

        let mut relevant_cmds = self.input_buffer[frames_ago..].to_vec();
        
        // relevant_cmds.dedup();

        if relevant_cmds.eq(&cmd.command.elements) {
            return true;
        } else {
            if relevant_cmds.len() == 0 {
                return false;
            }
            if relevant_cmds.len() == 1 && cmd.command.elements.len() == 1 {
                let single_buffer_ele = relevant_cmds.first().unwrap();
                let single_cmd_ele = cmd.command.elements.first().unwrap();
                if single_cmd_ele.is_four_way() && single_cmd_ele.four_way_match(single_buffer_ele)
                {
                    return true;
                }
            }
            if relevant_cmds.len() > cmd.command.elements.len() {
                relevant_cmds = remove_inbetweens(relevant_cmds);
                relevant_cmds.dedup_by(|ele1, ele2| {
                    match ele1 {
                        Element::Released(key1, n) => {
                            match ele2 {
                                Element::Released(key2, n) => key1 == key2, 
                                _ => false, 
                            }
                        }
                        Element::Held(key1) => {
                            match ele2 {
                                Element::Held(key2) => key1 == key2, 
                                _ => false, 
                            }
                        }
                        _ => false 
                    }
                }); 
                if cmd.command.elements.len() >= 3 && relevant_cmds.len() >= 3 {

                    if matches_sequence(&cmd, &relevant_cmds) {
                        return true; 
                    }
                    // let mut i: usize = 0;
                    // let mut j: usize = 0;
                    // let mut last_matched: usize = 0;
                    // let mut total_matched = 0;
                    // while i < cmd.command.elements.len() && j < relevant_cmds.len() {
                    //     if cmd.command.elements.get(i).unwrap().get_key()
                    //         == relevant_cmds.get(j).unwrap().get_key()
                    //     {
                    //         last_matched = j;
                    //         total_matched += 1;
                    //         i += 1;
                    //     } else if cmd.command.elements.get(i).unwrap().is_ordinal() {
                    //         let total_pairs = (j - last_matched)  / 2; 
                    //         let mut pairs_seen: usize = 0; 
                    //         while pairs_seen < total_pairs && (last_matched + (pairs_seen + 1)*2) <= j  {
                    //             let mut dir_set: HashSet<Direction> = HashSet::new(); 
                    //             for dir in &relevant_cmds[last_matched..last_matched + (pairs_seen+1)*2] {
                    //                 if dir.get_direction().is_some() {
                    //                     dir_set.insert(dir.get_direction().unwrap()); 
                    //                 }
                    //             }
                    //             if dir_set.len() > 0
                    //                 && cmd
                    //                     .command
                    //                     .elements
                    //                     .get(i)
                    //                     .unwrap()
                    //                     .get_key()
                    //                     .unwrap()
                    //                     .eq(&get_ordinals(dir_set))
                    //             {
                    //                 last_matched = j;
                    //                 total_matched += 1;
                    //                 i += 1;
                    //                 break; 
                    //             } else {
                    //                 pairs_seen += 1; 
                    //             }

                    //             // let mut k: usize = last_matched;
                    //             // let mut dir_set: HashSet<Direction> = HashSet::new();
                    //             // for dir in &relevant_cmds[..k] {
                    //             //     if dir.get_direction().is_some() {
                    //             //         dir_set.insert(dir.get_direction().unwrap());
                    //             //     }
                    //             // }
                    //             // if dir_set.len() > 0
                    //             //     && cmd
                    //             //         .command
                    //             //         .elements
                    //             //         .get(i)
                    //             //         .unwrap()
                    //             //         .get_key()
                    //             //         .unwrap()
                    //             //         .eq(&get_ordinals(dir_set))
                    //             // {
                    //             //     last_matched = j;
                    //             //     total_matched += 1;
                    //             //     i += 1;
                    //             // }
                    //         }
                    //     }
                    //     // if i > cmd.command.elements.len() { i = 0; }
                    //     j += 1;
                    // }
                    // if total_matched >= cmd.command.elements.len() {
                    //     // dbg!(total_matched);
                    //     //dbg!(&cmd.command.elements);
                    //     // dbg!(&relevant_cmds); 
                    //     return true;
                    // } else if total_matched > 1 {
                    //     // dbg!("missed"); 
                    //     // dbg!(&relevant_cmds);
                    // }
                }
                let result = matches_sequence_exact(&relevant_cmds, &cmd.command.elements);
                return result;
            }

            return false;
        }
    }

}