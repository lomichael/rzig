# rzig 
[![MIT License](https://img.shields.io/badge/license-MIT-blue)](/LICENSE)
[![justforfunnoreally.dev badge](https://img.shields.io/badge/justforfunnoreally-dev-9ff)](https://justforfunnoreally.dev)

A Zig compiler written in Rust.

> **DISCLAIMER**: This is a hobby project and a work-in-progress.

## Build
rzig has been tested on Debian Linux.

Prerequisites:
- Rust
- gcc
- x86-64 processor

To build the compiler:
```sh
make
```
or:
```sh
cargo build
```

## Usage
**Run the compiler**:
```sh
./target/debug/rzig <your_zig_file>
```

## Tests
**Build and run the tests**:
```sh
make test
```

- [X] Integers
- [ ] Unary Operators
    - [ ] Negation (`-`)
    - [ ] Bitwise Complement (`~`)
    - [ ] Logical Negation (`!`)
- [ ] Binary Operators
    - [ ] Addition (`+`)
    - [ ] Subtraction (`-`)
    - [ ] Multiplication (`*`)
    - [ ] Division (`/`)
- [ ] More Binary Operators
    - [ ] Logical AND (`&&`)
    - [ ] Logical OR (`||`)
    - [ ] Equal (`==`)
    - [ ] Not Equal (`!=`)
    - [ ] Less Than (`<`)
    - [ ] Less Than or Equal (`<=`)
    - [ ] Greater Than (`>`)
    - [ ] Greater Than or Equal (`>=`)
- [ ] Local Variables
    - [ ] Local Integer Variables in `main` 
- [ ] Conditionals
    - [ ] Conditional Statements (`if`)
    - [ ] Ternary Conditional Statements (`a ? b : c`)
- [ ] Compound Statements
- [ ] Loops
    - [ ] `for` Loops
    - [ ] `while` and `do` Loops
    - [ ] `break` and `continue`
- [ ] Functions
- [ ] Global Variables

## License
rzig is licensed under the MIT License. See `LICENSE` in the project for details.
