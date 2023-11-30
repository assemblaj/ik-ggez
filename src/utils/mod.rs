use ini_core;
pub mod ini;
pub mod sprite_sheet;

pub(crate) fn strip_comment_from_lines(value: &str) -> String {
    let lines: Vec<&str> = value.lines().collect();
    let stripped_lines: Vec<String> = lines
        .iter()
        .map(|&line| strip_comment(line).trim().to_string())
        .collect();
    stripped_lines.join("\n")
}

pub(crate) fn strip_comment(value: &str) -> &str {
    match value.split_once(";") {
        Some((input, _)) => input,
        None => value,
    }
}
