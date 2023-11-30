use air_rs::*;
use std::collections::HashSet;
use std::{collections::HashMap, env, fs, path};

pub struct Animator {
    action_map: HashMap<u64, Action>,
    time: u64,
    frame_time: i64,
    pub current_action: u64,
    current_element: usize,
    shown_animations: HashSet<String>,
    loop_start: usize,
    pub current_total_frames: i32,
}

impl Animator {
    pub fn new(air_file_path: &str) -> Self {
        let action_map = Animator::load_air(air_file_path);
        Self {
            time: 0,
            current_action: 0,
            current_element: 0,
            frame_time: 0,
            shown_animations: HashSet::new(),
            loop_start: 0,
            action_map,
            current_total_frames: 0,
        }
    }

    fn load_air(file_path: &str) -> HashMap<u64, Action> {
        let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");
        air_rs::parse(&unparsed_file).unwrap()
    }

    pub fn update(&mut self) {
        let action = self.action_map.get(&self.current_action).unwrap();
        //self.current_total_frames = action.elements.iter().map(|e| e.time).sum::<i64>() as i32;
        let current_frame = action.elements.get(self.current_element).unwrap();

        self.loop_start = action.loop_start;
        if (self.time as i32) < self.current_total_frames {
            self.time += 1;
        }
        // dbg!(self.time);
        self.frame_time += 1;
        let x = current_frame.x;
        let y = current_frame.y;
        if self.frame_time > current_frame.time {
            self.current_element = if self.current_element + 1 >= action.elements.len() {
                self.loop_start
            } else {
                self.current_element + 1
            };
            self.frame_time = 0;
        }
    }

    pub fn set_action(&mut self, action_no: u64) {
        self.current_action = action_no;
        self.current_element = 0;
        self.time = 0;
        let action = self.action_map.get(&self.current_action).unwrap();
        self.current_total_frames = action.elements.iter().map(|e| e.time).sum::<i64>() as i32;
        //dbg!("current total frames");
        //dbg!(self.current_total_frames);
        //dbg!(action_no);
        //dbg!(self.time);
        self.frame_time = 0;
    }

    pub fn draw(&self) -> (u16, u16) {
        let action = self.action_map.get(&self.current_action).unwrap();
        let current_frame = action.elements.get(self.current_element);
        (
            current_frame.unwrap().group.try_into().unwrap(),
            current_frame.unwrap().image.try_into().unwrap(),
        )
    }

    // for the triggers
    pub fn get_anim_no(&self) -> u64 {
        self.current_action
    }

    pub fn get_anim_time(&self) -> i64 {
        // dbg!((self.current_total_frames  - self.time as i32) as i64);
        (self.current_total_frames as i64 - self.time as i64) as i64
    }

    pub fn get_anim_element(&self) -> usize {
        self.current_element
    }
    
    pub fn set_element(&mut self, animation_element: i32) {
        self.current_element = animation_element as usize;
    }

    pub fn get_anim_action_no_set(&self) -> HashSet<u64> {
        let vec = self.action_map.keys().cloned().collect::<Vec<u64>>(); 
        let mut set = HashSet::new(); 
        for i in vec {
            set.insert(i); 
        }
        set
    }
}
