use air_rs::parse;
 
pub(crate) const I_MAX: i32 = (u32::MAX >> 1) as i32;
pub(crate) const I_ERR: i32 = !I_MAX;

pub(crate) fn split_and_trim<'a> (string: &'a str, sep: &str) -> Vec<&'a str> {
    string.split(sep).into_iter().map(|s| s.trim()).collect()
}

pub(crate) fn index_any(s: &str, chars: &str) -> Option<usize> {
    for (index, c) in s.char_indices() {
        if chars.contains(c) {
            return Some(index);
        }
    }
    None
}

// Need to return specific values upon an error of a parse. 
pub(crate) fn atoi(string: &str) -> i32 {
    let parse_result = str::parse::<i32>(string); 
    match parse_result {
        Ok(i) =>  i, 
        Err(e) => {
            match e.kind() {
                std::num::IntErrorKind::PosOverflow => I_MAX, 
                std::num::IntErrorKind::NegOverflow => I_ERR, 
                _ => I_ERR 
            }
        }, 
    }
}