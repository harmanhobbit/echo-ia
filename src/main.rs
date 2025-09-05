use clap::{command, Arg};

mod helpers;

fn main() {
    let match_results = command!()
        .about("Echo in Rust")
        .author("Ian Allaway, ian@allaway.tech")
        .after_long_help(helpers::clap_helpers::long_help())
        .arg(Arg::new("inputString"))
        .arg(helpers::clap_helpers::make_bool_arg(
            "new_line",
            'n',
            "do not output the trailing newline",
        ))
        .arg(helpers::clap_helpers::make_bool_arg(
            "escapeCharacters",
            'e',
            "enable interpretation of backslash escapes",
        ))
        .get_matches();

    let s = match_results.get_one::<String>("inputString").unwrap();
    let mut processed_string = s.to_string();
    if match_results.contains_id("escapeCharacters") {
        processed_string = helpers::string_helpers::escape_characters(s);
    }
    match match_results.get_one::<bool>("new_line") {
        Some(false) => println!("{}", processed_string),
        Some(true) => print!("{}", processed_string),
        None => println!("Oops"),
    }
}
