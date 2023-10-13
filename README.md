# The Rust Programming Book

This repository contains all code examples from [The Rust Programming Book, 2nd Edition](https://nostarch.com/rust-programming-language-2nd-edition)
> NOTE: This version of the text assumes you’re using Rust 1.62.0 (released 2022-06-30) or later with edition="2021" in the Cargo.toml file of all projects to configure them to use Rust 2021 edition idioms. See “Installation” on page 1 for instructions on installing or updating Rust, and see Appendix E for information on editions.
Each directory corresponds to a chapter in the book and is broken down into individual projects for each section.

## Repository Structure

```
.
├── chapter-01/
│   └── hello-world/
├── chapter-02/
│   ├── guessing-game/
├── chapter-03/
│   ├── variables/
│   └── functions/
├── ...
└── chapter-20/
    └── final-project/
```

## Chapter Breakdown

### Chapter 01: Getting Started

#### hello-world

- How to install Rust
- The first "Hello, World!" program

#### another-example

- Additional basic concepts

### Chapter 02: Programming a Guessing Game

#### guessing-game

- Introduction to Rust's type system
- User input
- Loops and error handling

### Chapter 03: Common Programming Concepts

#### variables

- Immutable and mutable variables
- Data types

#### functions

- Function definition and parameters
- Returning values

---

The above structure repeats for chapters 04 to 20.

## How to Use

1. Navigate to the chapter's directory
2. Inside each project folder, you'll find the complete code examples.
3. Run `cargo run` inside the project folder to execute the code.

## Dependencies

Make sure you have Rust and Cargo installed to run these examples.

## License

MIT License. See [LICENSE](LICENSE.md) for details.

---

Feel free to customize this README.md according to your requirements, but this should give you a solid starting point. rust-programming-language
