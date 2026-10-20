# Rust Logs 🦀

A hands-on record of my journey learning Rust through experiments, exercises, and projects.

I'm learning Rust step by step and using Git to track meaningful milestones. Each commit captures a change I can revisit later to understand how the code evolved and what I learned along the way.

### Project Structure
```
Rust-Logs/
├── README.md
├── LICENSE
├── .gitignore
│
└── Let's-Get-Rusty/
    ├── ch-01/
    │   ├── lesson-01/
    │   │   └── main.rs
    │   │
    │   └── lesson-02/
    │       ├── Cargo.toml
    │       └── src/
    │           └── main.rs
    │
    └── ch-02/
        └── ...
```
### How to Run

Install Rust using [rustup](https://rustup.rs/),then navigate to the directory of the lesson you want to run and execute:
```bash
cargo run
```

**Note:** `cargo run` requires a `Cargo.toml` file in the lesson directory. The current `lesson-01` contains only `main.rs`, so you'll need to add a Cargo project structure or run it directly using `rustc`.
### Learning Resources

- [The Rust Programming Language - Official Book](https://doc.rust-lang.org/book/)
- [Rustlings - Interactive Exercises](https://rustlings.rust-lang.org/)
- [Let's Get Rusty ](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8)

### My Learning Approach

- Build one concept at a time.
- Commit meaningful, working milestones.
- Write notes when I learn something important.
- Keep building projects as my Rust skills grow.

### License

My original code and notes are licensed under the [MIT License](https://github.com/K2K22K2K22/Rust-Logs/blob/main/LICENSE), unless otherwise specified. Third-party examples and tutorial-derived material remain subject to their original terms and are credited to their sources where applicable.
