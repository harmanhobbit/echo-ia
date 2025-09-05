use clap::{command, Arg};

fn main() {
    let match_results = command!()
        .about("Echo in Rust")
        .author("Ian Allaway, ian@allaway.tech")
        .after_long_help(
            r#"If -e is in effect, the following sequences are recognized:

\a      alert (BEL) 
\b      backspace 
\c      produce no further output 
\e      escape 
\f      form feed 
\n      new line 
\r      carriage return 
\t      horizontal tab 
\v      vertical tab

\0NNN and \xHH not implemented yet
\\ not used a rust appears to do it already"#,
        )
        .arg(Arg::new("inputString"))
        .arg(
            Arg::new("new_line")
                .help("do not output the trailing newline")
                .num_args(0)
                .short('n'),
        )
        .arg(
            Arg::new("escapeCharacters")
                .help("enable interpretation of backslash escapes")
                .num_args(0)
                .short('e'),
        )
        .get_matches();

    let s = match_results.get_one::<String>("inputString").unwrap();
    let mut processed_string = s.to_string();
    if match_results.contains_id("escapeCharacters") {
        processed_string = escape_characters(s);
    }
    match match_results.get_one::<bool>("new_line") {
        Some(false) => println!("{}", processed_string),
        Some(true) => print!("{}", processed_string),
        None => println!("Oops"),
    }
}

fn escape_characters(value: &str) -> String {
    let split_value = value.split("\\c").collect::<Vec<&str>>();
    split_value[0]
        // .replace("\\", "\x5c")
        .replace("\\a", "\x07")
        .replace("\\b", "\x08")
        .replace("\\c", "\x03")
        .replace("\\e", "\x1b")
        .replace("\\n", "\x0a")
        .replace("\\r", "\x0d")
        .replace("\\t", "\x09")
        .replace("\\v", "\x0b")
}

#[allow(dead_code)]
fn remove_first_and_last_chars(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_characters() {
        assert_eq!(escape_characters("\\n"), "\n");
        assert_eq!(escape_characters("\\"), r"\");
        // assert_eq!(escape_characters("\\a"), r"\a");
        // assert_eq!(escape_characters(r"test \b"), "test");
    }
}
