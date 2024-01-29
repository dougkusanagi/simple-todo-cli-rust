## A simple todo in CLI powered by Rust

### Description

This is a todo app written in Rust that runs in terminal, it's based on [Nuno Maduro's - Rust For PHP Developers](https://www.youtube.com/watch?v=HfcfQiLhsV4)

I rewrite the storage and print systems in a separated modules for writing some tests

### How to use

First compile, for that you will need cargo to be installed in your machine just visit [Official Rust Website](https://www.rust-lang.org)

After clone or download this repository, just run in a terminal this commands:

```shell
cd path/to/repository
cargo build --release
cd target/release
```

To use, just type:

```shell
./todo <command> <args>
```

Example:

```shell
./todo add "Become a god in Rust"
./todo add "Get rich"
./todo add list

# It will print
# Become a god in Rust'
# Get rich
```
