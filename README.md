# 🔒 Password Generator

[![CI Tests](https://github.com/sdanialraza/password-generator/actions/workflows/tests.yml/badge.svg)](https://github.com/sdanialraza/password-generator/actions/workflows/tests.yml)

A cross-platform password generator application that can generate passwords with a length of up to **128** characters.

You can also choose whether to include uppercase letters, lowercase letters, numbers, and special characters in the password.

By default, it generates a password with a length of **16** characters and includes uppercase letters, lowercase letters, numbers, and special characters.

## Installation

Make sure you have the standard Rust toolchain installed. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

```shell
git clone https://github.com/sdanialraza/password-generator.git
cd password-generator
cargo build --release
```

## Usage

Simply run `cargo run` and it will open the password generator application.

You can then choose the length using the slider and whether to include uppercase letters, lowercase letters, numbers, and special characters using the checkboxes.

Click the "Generate" button to generate a password and the "Copy" button to copy the generated password to the clipboard.

### Recent Passwords

It also saves the 10 most recently generated passwords.

> [!TIP]
> You can disable this feature by removing `features = ["persistence"]` from the [Cargo](Cargo.toml) file.

App data location:

- **Linux**: home/_USERNAME_/.local/share/password-generator
- **macOS**: Users/_USERNAME_/Library/Application Support/password-generator
- **Windows**: C:\Users\\_USERNAME_\AppData\Roaming\password-generator

## 🤝 Contributing

Pull requests are welcome, and very much appreciated. But before you start working on a pull request, please make sure to open an issue first, so that we can discuss the changes you want to make.

## ⚖️ License

This project is licensed under the [MIT License](LICENSE).
