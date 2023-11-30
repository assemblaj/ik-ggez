use ini_core;
use std::collections::BTreeMap;
use std::str::FromStr;

use indexmap::{map::Iter, IndexMap};

use std::{collections::HashMap, fs};

use super::{strip_comment, strip_comment_from_lines};

#[derive(Debug)]
pub struct Ini {
    ini: IndexMap<String, SectionContainer>,
}

#[derive(Debug)]
pub enum SectionContainer {
    Single(IniSection),
    Multiple(Vec<IniSection>),
}

impl SectionContainer {
    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        match self {
            SectionContainer::Single(section) => section.get(key),
            SectionContainer::Multiple(section_vec) => section_vec.first()?.get(key),
        }
    }

    pub fn get_list<T: FromStr>(&self, key: &str) -> Option<Vec<T>> {
        match self {
            SectionContainer::Single(section) => section.get_list(key),
            SectionContainer::Multiple(section_vec) => section_vec.first()?.get_list(key),
        }
    }

    pub fn get_tuple<T: FromStr>(&self, key: &str) -> Option<(T, T)> {
        match self {
            SectionContainer::Single(section) => section.get_tuple(key),
            SectionContainer::Multiple(section_vec) => section_vec.first()?.get_tuple(key),
        }
    }
}
impl Ini {
    pub fn new() -> Ini {
        let ini: IndexMap<String, SectionContainer> = IndexMap::new();
        Ini { ini }
    }

    pub fn iter(&self) -> Iter<String, SectionContainer> {
        self.ini.iter()
    }

    pub fn is_single(&self, key: &str) -> Option<bool> {
        match self.ini.get(key) {
            Some(SectionContainer::Single(_)) => Some(true),
            Some(SectionContainer::Multiple(_)) => Some(false),
            None => None,
        }
    }

    pub fn is_multiple(&self, key: &str) -> Option<bool> {
        match self.ini.get(key) {
            Some(SectionContainer::Single(_)) => Some(false),
            Some(SectionContainer::Multiple(_)) => Some(true),
            None => None,
        }
    }

    pub fn get_section_mut(&mut self, key: &str) -> Option<&mut SectionContainer> {
        self.ini.get_mut(key)
    }

    pub fn get_section(&self, key: &str) -> Option<&SectionContainer> {
        self.ini.get(key)
    }

    fn remove_section(&mut self, key: &str) -> Option<SectionContainer> {
        self.ini.remove(key)
    }

    pub fn insert(&mut self, section_name: String, section: SectionContainer) {
        self.ini.insert(section_name, section);
    }

    pub fn get_int(&self, section: &str, key: &str) -> Option<i32> {
        self.get(section, key)
    }

    pub fn get_float(&self, section: &str, key: &str) -> Option<f64> {
        self.get(section, key)
    }

    pub fn get_bool(&self, section: &str, key: &str) -> Option<bool> {
        self.get(section, key)
    }

    fn get_int_list(&self, section: &str, key: &str) -> Option<Vec<i32>> {
        self.get_list(section, key)
    }

    fn get_float_list(&self, section: &str, key: &str) -> Option<Vec<f32>> {
        self.get_list(section, key)
    }

    fn get_bool_list(&self, section: &str, key: &str) -> Option<Vec<bool>> {
        self.get_list(section, key)
    }

    pub fn get_int_tuple(&self, section: &str, key: &str) -> Option<(i32, i32)> {
        self.get_tuple(section, key)
    }

    pub fn get_float_tuple(&self, section: &str, key: &str) -> Option<(f32, f32)> {
        self.get_tuple(section, key)
    }

    fn get_bool_tuple(&self, section: &str, key: &str) -> Option<(bool, bool)> {
        self.get_tuple(section, key)
    }

    pub fn get<T: FromStr>(&self, section: &str, key: &str) -> Option<T> {
        match self.ini.get(section) {
            Some(sec) => sec.get(key),
            None => None,
        }
    }

    pub fn get_list<T: FromStr>(&self, section: &str, key: &str) -> Option<Vec<T>> {
        match self.ini.get(section) {
            Some(sec) => sec.get_list(key),
            None => None,
        }
    }

    pub fn get_tuple<T: FromStr>(&self, section: &str, key: &str) -> Option<(T, T)> {
        match self.ini.get(section) {
            Some(sec) => sec.get_tuple(key),
            None => None,
        }
    }
}

// it's an expression if it contains special characters

#[derive(Debug)]
pub struct IniSection {
    section: IndexMap<String, IniEntry>,
}

#[derive(Debug)]
enum IniEntry {
    Single(String),
    Multiple(Vec<String>),
}

impl IniSection {
    fn new() -> IniSection {
        IniSection {
            section: IndexMap::new(),
        }
    }

    pub fn is_single(&self, key: &str) -> Option<bool> {
        match self.section.get(key) {
            Some(IniEntry::Single(_)) => Some(true),
            Some(IniEntry::Multiple(_)) => Some(false),
            None => None,
        }
    }

    pub fn is_multiple(&self, key: &str) -> Option<bool> {
        match self.section.get(key) {
            Some(IniEntry::Single(_)) => Some(false),
            Some(IniEntry::Multiple(_)) => Some(true),
            None => None,
        }
    }

    fn insert(&mut self, key: String, value: String) {
        if self.section.contains_key(&key) {
            if let Some(IniEntry::Single(entry)) = self.section.get(&key) {
                self.section
                    .insert(key, IniEntry::Multiple(vec![entry.to_string(), value]));
            } else if let Some(IniEntry::Multiple(entries)) = self.section.get_mut(&key) {
                entries.push(value)
            }
        } else {
            self.section.insert(key, IniEntry::Single(value));
        }
    }

    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        match self.section.get(key) {
            Some(IniEntry::Single(entry)) => entry.parse().ok(),
            Some(IniEntry::Multiple(entries)) => entries.first()?.parse().ok(),
            None => None,
        }
    }

    pub fn get_strings(&self, key: &str) -> Option<&Vec<String>> {
        match self.section.get(key) {
            Some(IniEntry::Multiple(entries)) => Some(entries),
            _ => None,
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)
    }

    pub fn get_list<T: FromStr>(&self, key: &str) -> Option<Vec<T>> {
        self.get_string(key)?
            .split(",")
            .map(|s| s.parse().ok())
            .collect::<Option<Vec<T>>>()
    }

    pub fn get_tuple<T: FromStr>(&self, key: &str) -> Option<(T, T)> {
        let value = self.get_string(key)?;
        let mut values = value.split(",").map(|s| s.parse().ok());
        Some((values.next()??, values.next()??))
    }

    fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key)
    }

    fn get_int(&self, key: &str) -> Option<i32> {
        self.get(key)
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)
    }

    fn get_int_list(&self, key: &str) -> Option<Vec<i32>> {
        self.get_list(key)
    }

    fn get_float_list(&self, key: &str) -> Option<Vec<f32>> {
        self.get_list(key)
    }

    fn get_bool_list(&self, key: &str) -> Option<Vec<bool>> {
        self.get_list(key)
    }

    fn get_int_tuple(&self, key: &str) -> Option<(i32, i32)> {
        self.get_tuple(key)
    }

    fn get_float_tuple(&self, key: &str) -> Option<(f32, f32)> {
        self.get_tuple(key)
    }

    fn get_bool_tuple(&self, key: &str) -> Option<(bool, bool)> {
        self.get_tuple(key)
    }
}
impl From<IndexMap<String, IniEntry>> for IniSection {
    fn from(section: IndexMap<String, IniEntry>) -> Self {
        IniSection { section }
    }
}

pub(crate) fn load_ini(ini_file_path: &str) -> Ini {
    let ini_string = fs::read_to_string(ini_file_path).expect("Error.");
    parse_ini(&ini_string)
}

fn load_cns(cns_file: &str) -> HashMap<String, Vec<HashMap<String, String>>> {
    let data = std::fs::read_to_string(cns_file).expect("error");
    parse_cns(data.as_str())
}

// Allows for case insentive value lookup
// But consider that error reporting values on states
// [State 123, SomeValue]
// Will be lowercase when the time comes
fn parse_ini_old(ini_str: &str) -> HashMap<String, HashMap<String, String>> {
    let mut ini: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut cur_section: String = String::new();
    let stripped_lines = strip_comment_from_lines(ini_str);
    for item in ini_core::Parser::new(&stripped_lines) {
        match item {
            ini_core::Item::Section(s) => {
                cur_section = s.to_lowercase();
                ini.insert(cur_section.clone(), HashMap::new());
            }
            ini_core::Item::Property(s, t) => {
                if let Some(section) = ini.get_mut(&cur_section) {
                    section.insert(
                        s.trim().to_lowercase(),
                        strip_comment(t.unwrap_or_default()).trim().to_string(),
                    );
                }
            }
            _ => {}
        }
    }
    ini
}

fn parse_cns(ini_str: &str) -> HashMap<String, Vec<HashMap<String, String>>> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    let mut current_section: String = String::new();
    let mut current_statedef: String = String::new();

    let mut current_properties: HashMap<String, String> = HashMap::new();

    let stripped_lines = strip_comment_from_lines(ini_str);

    for item in ini_core::Parser::new(&stripped_lines) {
        match item {
            ini_core::Item::Section(section) => {
                if section.to_lowercase().starts_with("statedef") {
                    current_statedef = section.to_string()
                }
                current_section = section.to_string();
                current_properties = HashMap::new();
            }
            ini_core::Item::Property(key, value) => {
                current_properties.insert(key.to_string(), value.unwrap_or_default().to_string());
            }
            ini_core::Item::SectionEnd => {
                let key_value: String;
                if current_section.to_lowercase().starts_with("state") {
                    key_value = current_statedef.clone();
                    current_properties.insert("state_name".to_string(), current_section.clone());
                } else {
                    key_value = current_section.clone();
                }
                let section_data = result.entry(key_value).or_insert(Vec::new());
                section_data.push(current_properties.clone());
            }
            _ => {}
        }
    }

    result
}

// pub(crate) fn parse_ini_with_subsection(ini_str: &str, root:&str, subsection:&str) -> EngineIni {
//     let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
//     let mut current_section: String = String::new();
//     let mut current_statedef: String = String::new();

//     let mut current_properties: HashMap<String, String> = HashMap::new();

//     let stripped_lines = strip_comment_from_lines(ini_str);

//     for item in ini_core::Parser::new(&stripped_lines) {
//         match item {
//             ini_core::Item::Section(section) => {
//                 if section.to_lowercase().starts_with(root) {
//                     current_statedef = section.to_string()
//                 }
//                 current_section = section.to_string();
//                 current_properties = HashMap::new();
//             }
//             ini_core::Item::Property(key, value) => {
//                 current_properties.insert(key.to_string(), value.unwrap_or_default().to_string());
//             }
//             ini_core::Item::SectionEnd => {
//                 let key_value: String;
//                 if current_section.to_lowercase().starts_with(subsection) {
//                     key_value = current_statedef.clone();
//                     current_properties.insert("subsection_name".to_string(), current_section.clone());
//                 } else {
//                     key_value = current_section.clone();
//                 }
//                 let section_data = result.entry(key_value).or_insert(Vec::new());
//                 section_data.push(current_properties.clone());
//             }
//             _ => {}
//         }
//     }

//     result
// }

// Allows for case insentive value lookup
// But consider that error reporting values on states
// [State 123, SomeValue]
// Will be lowercase when the time comes
pub(crate) fn parse_ini(ini_str: &str) -> Ini {
    let mut ini = Ini::new();
    let mut cur_section: String = String::new();
    let stripped_lines = strip_comment_from_lines(ini_str);
    let mut in_subsection = false;

    for item in ini_core::Parser::new(&stripped_lines) {
        match item {
            ini_core::Item::Section(s) => {
                cur_section = s.to_lowercase();

                if let Some(SectionContainer::Multiple(sections)) =
                    ini.get_section_mut(&cur_section)
                {
                    in_subsection = true;
                    sections.push(IniSection::new())
                } else if let Some(SectionContainer::Single(section)) =
                    ini.remove_section(&cur_section)
                {
                    in_subsection = true;
                    ini.insert(
                        cur_section.clone(),
                        SectionContainer::Multiple(vec![section, IniSection::new()]),
                    )
                } else {
                    in_subsection = false;
                    ini.insert(
                        cur_section.clone(),
                        SectionContainer::Single(IniSection::new()),
                    );
                }
            }
            ini_core::Item::Property(s, t) => {
                if let Some(section) = ini.get_section_mut(&cur_section) {
                    let key = s.trim().to_lowercase();
                    let value = strip_comment(t.unwrap_or_default()).trim().to_string();
                    if in_subsection {
                        match section {
                            SectionContainer::Multiple(sec) => {
                                let mut last = sec.last_mut().unwrap();
                                last.insert(key, value);
                            }
                            SectionContainer::Single(sec) => {
                                sec.insert(key, value);
                            }
                        }
                    } else {
                        match section {
                            SectionContainer::Single(s) => s.insert(key, value),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    ini
}
