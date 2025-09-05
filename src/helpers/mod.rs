pub mod clap_helpers {
    use clap::Arg;

    pub fn long_help() -> String {
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
\\ not used a rust appears to do it already"#
            .to_string()
    }

    pub fn make_bool_arg(
        identifier: &'static str,
        short_flag: char,
        help_text: &'static str,
    ) -> Arg {
        Arg::new(identifier)
            .short(short_flag)
            .help(help_text)
            .num_args(0)
    }
}

pub mod string_helpers {
    pub fn escape_characters(value: &str) -> String {
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
}
#[cfg(test)]
mod tests {
    use super::string_helpers::*;

    #[test]
    fn test_escape_characters() {
        assert_eq!(escape_characters("\\n"), "\n");
        assert_eq!(escape_characters("\\"), r"\");
        // assert_eq!(escape_characters("\\a"), r"\a");
        // assert_eq!(escape_characters(r"test \b"), "test");
    }
}
