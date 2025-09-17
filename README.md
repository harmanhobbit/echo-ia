# Echo in Rust

A simple command-line tool that echoes user input with optional formatting.

## Features

- Echoes input string to the console
- Supports optional flags:
  - `-n`: Do not output the trailing newline
  - `-e`: Enable interpretation of backslash escapes

## Installation

### From Source

1. Clone the repository:
   ```
   git clone https://github.com/harmanhobbit/ia-echo.git
   cd ia-echo
   ```
2. Build the project:
   ```
   cargo build --release
   ```
3. The compiled binary will be available in `target/release/echo`.

## Usage

```
echo [OPTIONS] <inputString>
```

### Options

- `-n, --new-line`: Do not output the trailing newline
- `-e, --escapeCharacters`: Enable interpretation of backslash escapes

### Examples

Echo a simple string:
```
echo "Hello, World!"
```

Echo a string without a trailing newline:
```
echo -n "Hello, World!"
```

Echo a string with backslash escapes:
```
echo -e "Hello,\nWorld!"
```

## Context

The project consists of the following files:

[1] **main.rs**:
This is the main entry point of the application, which handles the command-line arguments and processes the input string.

[2] **mod.rs**:
This module contains helper functions for working with the command-line interface and string manipulation.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
