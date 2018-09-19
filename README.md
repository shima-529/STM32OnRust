# STM32 on Rust
Embedded(Bare Metal) programming project for STM32(**STM32F303K8T6**). To make this project work, you need:

- Rust conpiler(**nightly**)
- Xargo

Thus, you may first need to set the environment by executing:

```bash
$ rustup default nightly
$ rustup component add rust-src
$ cargo install xargo
```

Now you are ready for building. Just type:

```bash
$ git clone https://github.com/shima-529/stm32OnRust.git
$ cd stm32OnRust
$ make # build the elf binary
$ make flash # flash by SWD debugging
```

# LISENCE
MIT
