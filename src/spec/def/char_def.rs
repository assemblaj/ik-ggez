use crate::utils::ini::*;
use std::str::FromStr;

// None, different entities may have their own .defs.
pub struct CharDef {
    ini_file: Ini,
}

pub struct CharInfo {
    pub name: Option<String>,           // Name of character
    pub display_name: Option<String>,   // Name of character to display
    pub version_data: Option<String>,   // Version of character (MM-DD-YYYY or X.XX)
    pub mugen_version: Option<String>,  // Version of M.U.G.E.N character works on (X.XX)
    pub author: Option<String>,         // Character author name
    pub pal_defaults: Option<Vec<i32>>, // Default palettes in order of preference (up to 4). Numbering starts from 1
    pub localcoord: Option<(u32, u32)>, // Local coordinate space width and height
}

pub struct CharFiles {
    pub cmd: Option<String>,           // Command set
    pub constants: Option<String>,     // Constants
    pub states: Option<String>,        // States
    pub common_states: Option<String>, // Common states (from data/ or motif)
    pub sprite_sheet: Option<String>,  // Sprite
    pub animations: Option<String>,    // Animation
    pub sounds: Option<String>,        // Sound
    pub movelist: Option<String>,      // Ikemen feature: Movelist
    ai: Option<String>,                // AI hints data (not used)
}

impl CharDef {
    pub fn new(def_file_path: &str) -> Self {
        let ini_file = load_ini(def_file_path);
        CharDef { ini_file }
    }

    const INFO_KEY: &str = "info"; // [Info]  Player information
    pub fn get_from_info<T: FromStr>(&self, key: &str) -> Option<T> {
        self.ini_file.get(Self::INFO_KEY, key)
    }

    const NAME_KEY: &str = "name";
    const DISPLAY_NAME_KEY: &str = "displayname";
    const VERSION_DATE_KEY: &str = "versiondate";
    const MUGEN_VERSION_KEY: &str = "mugenversion";
    const AUTHOR_KEY: &str = "author";
    const PAL_DEFAUULTS_KEY: &str = "pal.defaults";
    const LOCAL_COORDS_KEY: &str = "localcoord";

    pub fn get_info(&self) -> CharInfo {
        CharInfo {
            name: self.get_from_info(Self::NAME_KEY),
            display_name: self.get_from_info(Self::DISPLAY_NAME_KEY),
            version_data: self.get_from_info(Self::VERSION_DATE_KEY),
            mugen_version: self.get_from_info(Self::MUGEN_VERSION_KEY),
            author: self.get_from_info(Self::AUTHOR_KEY),
            pal_defaults: self.get_pal_defaults(),
            localcoord: self.get_local_coords(),
        }
    }

    fn get_pal_defaults(&self) -> Option<Vec<i32>> {
        self.ini_file
            .get_list(Self::INFO_KEY, Self::PAL_DEFAUULTS_KEY)
    }

    fn get_local_coords(&self) -> Option<(u32, u32)> {
        self.ini_file
            .get_tuple(Self::INFO_KEY, Self::LOCAL_COORDS_KEY)
    }

    const FILES_KEY: &str = "files"; // [Files] Files for the player
    pub fn get_from_files(&self, key: &str) -> Option<String> {
        self.ini_file.get(Self::FILES_KEY, key)
    }

    const CMD_KEY: &str = "cmd";
    const CONSTANTS_KEY: &str = "cns";
    const STATES_KEY: &str = "st";
    const COMMON_STATES_KEY: &str = "stcommon";
    const SPRITE_KEY: &str = "sprite";
    const ANIMATION_KEY: &str = "anim";
    const SOUND_KEY: &str = "sound";
    const AI_KEY: &str = "ai";
    const MOVELIST_KEY: &str = "movelist";

    pub fn get_filenames(&self) -> CharFiles {
        CharFiles {
            cmd: self.get_from_files(Self::CMD_KEY),
            constants: self.get_from_files(Self::CONSTANTS_KEY),
            states: self.get_from_files(Self::STATES_KEY),
            common_states: self.get_from_files(Self::COMMON_STATES_KEY),
            sprite_sheet: self.get_from_files(Self::SPRITE_KEY),
            animations: self.get_from_files(Self::ANIMATION_KEY),
            sounds: self.get_from_files(Self::SOUND_KEY),
            ai: self.get_from_files(Self::AI_KEY),
            movelist: self.get_from_files(Self::MOVELIST_KEY),
        }
    }
}
