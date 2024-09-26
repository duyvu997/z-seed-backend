# z-seed-backend

A seed project for Rust backend development.

## Getting Started

### Prerequisites

- Rust: Install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-username/z-seed-backend.git
   cd z-seed-backend
   ```

2. Build the project:
   ```
   cargo build
   ```

## Usage

Run the project:
```
cargo run
```

Run tests:
```
cargo test
```

## Development

Useful commands for development:

- Check code without building:
  ```
  cargo check
  ```

- Format code:
  ```
  cargo fmt
  ```

- Run the linter:
  ```
  cargo clippy
  ```

## Configuration Files

### Makefile.toml

This project uses `cargo-make` for task running. See `Makefile.toml` for all available tasks.

### rustfmt.toml

The `rustfmt.toml` file in the root directory configures the Rust formatter. It sets rules for code formatting, ensuring consistency across the project. Some key settings include:

- `max_width = 100`: Maximum line width
- `tab_spaces = 4`: Use 4 spaces for indentation
- `edition = "2021"`: Use Rust 2021 edition
- `reorder_imports = true`: Automatically reorder import statements
- `reorder_modules = true`: Automatically reorder module declarations

For a full list of formatting options, refer to the [rustfmt documentation](https://rust-lang.github.io/rustfmt/).

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- List any libraries, tools, or resources you've used or been inspired by
