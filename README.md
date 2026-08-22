# Rust Programs

A collection of Rust programs, examples, and mini-projects created while learning and exploring the Rust programming language.

This repository is a hands-on Rust learning playground covering fundamental concepts such as ownership, borrowing, structs, enums, traits, iterators, strings, multithreading, message passing, error handling, and command-line applications.

> The goal of this repository is to learn Rust by writing code, experimenting with concepts, and building small programs.

---

## What You'll Find Here

The repository contains examples and experiments covering:

* Rust fundamentals and syntax
* Ownership and borrowing
* Structs and implementations
* Enums and pattern matching
* Traits
* Iterators
* String manipulation
* Stack and heap concepts
* Multithreading
* Message passing with channels
* Error handling with `Result` and `Option`
* File handling
* External crates
* Command-line games
* Small programming exercises

---

## Mini Projects

### Guess the Fruit

A simple command-line guessing game.

The program randomly selects a fruit from a predefined list and asks the user to guess it.

**Concepts demonstrated:**

* User input
* String handling
* Conditional statements
* Random number generation
* External crates

**Source:** [`src/guess_fruit.rs`](src/guess_fruit.rs)

---

### Tic-Tac-Toe

A two-player command-line Tic-Tac-Toe game played on a 3×3 board.

The game includes:

* Player turns
* Board management
* Input validation
* Winner detection
* Draw detection
* 2D arrays
* Functions
* Loops
* References

**Source:** [`src/tic-tac-toe-game.rs`](src/tic-tac-toe-game.rs)

---

## Rust Concepts

### Ownership & Borrowing

Examples demonstrating Rust's ownership model, including:

* Moving values
* Cloning values
* Immutable borrowing
* Mutable borrowing
* References
* String ownership
* Slices

**Source:** [`src/main_ownership_borrowing_structs.rs`](src/main_ownership_borrowing_structs.rs)

---

### Structs

Examples covering:

* Defining structs
* Creating struct instances
* Accessing fields
* Mutable structs
* Methods
* `&self` references

---

### Enums & Pattern Matching

Examples exploring:

* Defining enums
* Enum variants
* Associated data
* `match` expressions
* Using enums to model different types of values

**Source:** [`src/main_enum_external_packages.rs`](src/main_enum_external_packages.rs)

---

### Traits

Examples covering:

* Defining traits
* Implementing traits for structs
* Trait methods
* Passing traits as function parameters

---

### Iterators

Examples demonstrating:

* `iter()`
* `iter_mut()`
* `into_iter()`
* `next()`
* `map()`
* `filter()`
* Iterating with `for` loops

These examples help explore Rust's iterator model and functional-style operations.

**Source:** [`src/main.rs`](src/main.rs)

---

### Multithreading

Examples exploring Rust's concurrency model using:

* `std::thread`
* Thread spawning
* Joining threads
* Concurrent execution

---

### Message Passing

Examples demonstrating communication between threads using Rust's:

```rust
std::sync::mpsc
```

The examples explore:

* Channels
* Sender (`tx`)
* Receiver (`rx`)
* Sending data between threads
* Handling received values

---

### Error Handling

Examples exploring Rust's approach to handling errors using:

* `Result`
* `Option`
* `match`
* Custom error types
* File-reading errors

---

### Strings & Slices

Examples covering:

* Creating `String`
* Mutating strings
* `push_str`
* `replace_range`
* String slices
* Finding words
* Borrowing strings instead of unnecessarily copying them

---

### Stack & Heap

Examples demonstrating the difference between stack and heap allocation using primitive values and `String`.

---

## Project Structure

```text
rust-programs/
├── src/
│   ├── guess_fruit.rs
│   ├── main.rs
│   ├── main_basic_program.rs
│   ├── main_enum_external_packages.rs
│   ├── main_ownership_borrowing_structs.rs
│   └── tic-tac-toe-game.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

### File Overview

| File                                  | Purpose                                                                                        |
| ------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `main.rs`                             | General Rust learning examples including iterators, strings, traits, concurrency, and channels |
| `main_basic_program.rs`               | Basic Rust concepts and stack/heap examples                                                    |
| `main_ownership_borrowing_structs.rs` | Ownership, borrowing, references, structs, and methods                                         |
| `main_enum_external_packages.rs`      | Enums, pattern matching, file handling, and error-handling examples                            |
| `guess_fruit.rs`                      | Command-line fruit guessing game                                                               |
| `tic-tac-toe-game.rs`                 | Command-line Tic-Tac-Toe game                                                                  |

---

## Tech Stack

* **Language:** Rust
* **Edition:** Rust 2021
* **Build Tool:** Cargo
* **Package Manager:** Cargo
* **Application Type:** Command-line / learning programs

---

## Getting Started

### Prerequisites

Install Rust and Cargo before working with this repository.

You can install Rust using [rustup](https://rustup.rs/).

Verify the installation:

```bash
rustc --version
cargo --version
```

### Clone the Repository

```bash
git clone https://github.com/shrinjoy979/rust-programs.git
```

Navigate into the project:

```bash
cd rust-programs
```

---

## Running the Project

The repository is structured as a learning collection rather than a single application.

The default Cargo binary is `src/main.rs`.

Run it with:

```bash
cargo run
```

Build the project with:

```bash
cargo build
```

Build for release:

```bash
cargo build --release
```

---

## Exploring Individual Examples

Most of the learning examples are kept inside the files under `src/`.

Open the files and uncomment or adapt the examples you want to experiment with.

```text
src/
├── main.rs
├── main_basic_program.rs
├── main_enum_external_packages.rs
├── main_ownership_borrowing_structs.rs
├── guess_fruit.rs
└── tic-tac-toe-game.rs
```

The learning files contain multiple small examples rather than a single application entry point.

---

## Running the Tic-Tac-Toe Game

The Tic-Tac-Toe implementation is located at:

```text
src/tic-tac-toe-game.rs
```

The game uses a 3×3 board and accepts row and column coordinates from the players.

Example input:

```text
Player X input (row, col)
0 1
```

The game validates the coordinates, updates the board, checks for a winner, and detects draws.

---

## Guess the Fruit

The fruit guessing game is located at:

```text
src/guess_fruit.rs
```

The program randomly selects one of:

```text
apple
banana
orange
```

and asks the user to guess the selected fruit.

> **Note:** The current implementation uses the `rand` crate. If you want to run this example through Cargo, make sure the corresponding dependency is declared in `Cargo.toml`.

---

## Learning Path

A useful way to explore this repository is to go through the concepts in the following order:

```text
1. Basic Rust Syntax
        ↓
2. Variables & Data Types
        ↓
3. Strings
        ↓
4. Ownership
        ↓
5. Borrowing & References
        ↓
6. Structs
        ↓
7. Enums & Pattern Matching
        ↓
8. Traits
        ↓
9. Iterators
        ↓
10. Error Handling
        ↓
11. File Handling
        ↓
12. Multithreading
        ↓
13. Message Passing
        ↓
14. CLI Projects
```

This progression makes it easier to understand how Rust's core concepts build on each other.

---

## Purpose

This repository is primarily intended for:

* Learning Rust from fundamentals
* Practicing Rust syntax
* Understanding ownership and borrowing
* Experimenting with Rust's type system
* Learning concurrency
* Building small command-line applications
* Keeping track of Rust learning progress
* Revisiting Rust concepts through practical examples

---

## Future Improvements

Possible additions to this repository include:

* [ ] More Rust exercises
* [ ] More CLI applications
* [ ] Better project organization
* [ ] Unit and integration tests
* [ ] More error-handling examples
* [ ] Async Rust examples
* [ ] File-processing utilities
* [ ] Networking examples
* [ ] Data structures and algorithms
* [ ] Rust web development examples
* [ ] Solana programs using Rust

---

## Contributing

This repository is primarily a personal learning project, but suggestions and improvements are welcome.

If you'd like to contribute:

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Commit your changes
5. Open a pull request

---

## License

This repository is intended for educational and learning purposes.

No license is currently specified for the repository. If you plan to allow others to reuse, modify, or distribute the code, consider adding an appropriate open-source license.

---

## Author

**Shrinjoy Saha**

GitHub: [@shrinjoy979](https://github.com/shrinjoy979)

---

If you're also learning Rust, feel free to explore the examples and experiment with the code.
