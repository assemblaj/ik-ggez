use std::{collections::HashMap, ops::Index};

use ggez::input::mouse::cursor_grabbed;

pub(crate) struct IniSection {
    map: HashMap<String, String>,
}

impl IniSection {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn remove(&mut self, name: &str) -> Option<String> {
        self.map.remove(name)
    }

    pub(crate) fn parse(&mut self, lines: Vec<&str>, i: &mut usize) {
        while *i < lines.len() {
            let mut cur_line = lines.get(*i as usize).unwrap();
            if cur_line.len() > 0 && cur_line.starts_with("[") {
                break;
            }
            let mut split_str = cur_line.splitn(2, ";").nth(0).unwrap();
            let mut trimmed_str = split_str.trim();
            cur_line = &trimmed_str;
            let mut ia = cur_line.find("= \t");
            if let Some(index) = ia {
                let name = cur_line[..index].to_lowercase();
                let mut data: String = String::new();
                ia = cur_line.find("=");
                if let Some(index) = ia {
                    data = cur_line[index + 1..].to_string()
                }
                if self.map.get(&name).is_none() {
                    self.map.insert(name, data);
                }
            }
        }
    }

    pub(crate) fn read(lines: Vec<&str>, i: &mut usize) -> (IniSection, (String, String)) {
        let mut name: String = String::new();
        let mut subname: String = String::new();

        while *i < lines.len() {
            (name, subname) = IniSection::section_name(lines[*i]);
            if name.len() > 0 {
                (*i) += 1;
                break;
            }
            *i = *i + 1;
        }

        if name.len() == 0 {
            return (IniSection::new(), (name, subname));
        }

        let mut ini_section = IniSection::new();
        ini_section.parse(lines, i);
        return (ini_section, (name, subname));
    }

    pub(crate) fn section_name(mut sec: &str) -> (String, String) {
        if sec.len() == 0 || !sec.starts_with('[') {
            return ("".to_string(), "".to_string());
        }
        let mut split_str = sec.splitn(2, ";").nth(0).unwrap();
        let mut trimmed_str = split_str.trim();

        if !trimmed_str.ends_with("]") {
            return ("".to_string(), "".to_string());
        }
        trimmed_str = &trimmed_str[1..trimmed_str.find("]").unwrap()];
        let name: &str;
        let i = trimmed_str.find(" ");
        if let Some(index) = i {
            name = &trimmed_str[..index + 1];
            trimmed_str = &trimmed_str[index + 1..];
        } else {
            name = trimmed_str.clone().as_ref();
            trimmed_str = "";
        }
        (name.to_lowercase(), trimmed_str.to_owned())
    }
}
