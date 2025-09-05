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

    match match_results.get_one::<bool>("new_line") {
        Some(false) => println!(
            "{}",
            // match_results.get_one::<bool>("new_line").unwrap()
            match_results.get_one::<String>("inputString").unwrap()
        ),
        Some(true) => print!(
            "{}",
            match_results.get_one::<String>("inputString").unwrap()
        ),
        None => println!("Oops"),
    }
    if match_results.contains_id("escapeCharacters") {
        println!("escapeCharacters found");
    }
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
    fn test_remove_first_and_last_chars() {
        assert_eq!(
            remove_first_and_last_chars("this is a test"),
            "his is a tes"
        );
        assert_eq!(
            remove_first_and_last_chars("This is a test"),
            "his is a tes"
        );
        assert_eq!(
            remove_first_and_last_chars(" This is a test "),
            "This is a test"
        );
    }
}
