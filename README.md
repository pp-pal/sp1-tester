## Description

This is a simple example of SP1 code. It runs the `check-adult` function within the SP1 zkVM to determine whether the given date of birth indicates that the individual is over 18 years old.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)

## Directory Structure

```text
folder-structure
├── check-adult
│   ├── Cargo.toml
│   ├── elf      <-- [The ELF binary of the program will be located here]
│   └── src
│       └── main.rs     <-- [The check-adult guest code goes here]
└── script
  ├── Cargo.toml
  └── src
         └── lib.rs     <-- [The host guest code goes here]
```

## How To Run

- Build ELF binary
```bash
cd check-adult && cargo prove build
```
- Build host code
```bash
cd ../script/ && cargo build -r
```
- Run `check_adult`, expected `true`
```bash
./target/release/check-adult-host 2006-02-18
```
- Run `check_adult`, expected `false`
```bash
./target/release/check-adult-host 2023-02-18
```

## References

- [SP1 Installation](https://succinctlabs.github.io/sp1/getting-started/install.html)
- [SP1 Hello World](https://succinctlabs.github.io/sp1/writing-programs/setup.html)
