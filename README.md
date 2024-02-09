# Vi Editor in Rust

This project implements a basic Vi text editor in Rust using the Crossterm library for terminal manipulation. Vi is a popular text editor known for its modal editing style, and this implementation aims to provide a simplified version of Vi's functionality.

## Features

- Insert mode for typing and editing text
- Save mode for saving changes to a file
- Basic cursor movement (left, right, up, down)
- Insertion, deletion, and modification of text
- Saving edited content to a file

## Installation

To use the Vi editor, follow these steps:

1. Clone the repository to your local machine:

```bash
git clone 
```

2. Navigate to the project directory:

```bash
cd vi
```

3. Build and run the application:

```bash
cargo run
```

## How to Use

1. **Insert Mode**: When you launch the Vi editor, you'll start in insert mode. Here, you can type and edit text directly.

2. **Save Mode**: Press `Esc` to enter save mode. In save mode, you can save your changes to a file by pressing `s`. Press `q` to quit without saving.

3. **Navigation**: Use arrow keys to move the cursor left, right, up, and down.

4. **Editing**: Use the backspace key to delete characters. You can also insert characters at the cursor position.

5. **File Management**: When you run the Vi editor, you can specify a file path as a command-line argument to edit an existing file. Changes will be saved to the same file.

## Example

To edit a file named `a`, run the Vi editor with:

```bash
cargo run a
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or create a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project was inspired by the classic Vi text editor and built using the Crossterm library for Rust terminal manipulation.
