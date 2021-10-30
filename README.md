# rust-echo

This time we will use third-party library: [clap](https://crates.io/crates/clap) to help us finish a simple utility: `echo`.

`clap` is an awesome Command Line Argument Parser for rust. It's easy to use and efficient for parsing command line arguments and subcommands.

It looks like this:

```bash
# rust-echo on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.0 
❯ cargo r 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-echo`
error: The following required arguments were not provided:
    <TEXT>...

USAGE:
    rust-echo [FLAGS] <TEXT>...

For more information try --help

# rust-echo on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.0 
[😅 ERROR]❯ cargo r -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-echo -h`
rust-echo 0.1.0
tomsawyer404@outlook.com
`echo` utility written in Rust

USAGE:
    rust-echo [FLAGS] <TEXT>...

FLAGS:
    -h, --help       Prints help information
    -n               do not output the trailing newline
    -V, --version    Prints version information

ARGS:
    <TEXT>...    Please input some words

# rust-echo on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.0 
❯ cargo r hello world
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-echo hello world`
hello world

```

The projcet inspired by this [video](https://www.bilibili.com/video/BV1AL411x7Lh?from=search&seid=11547016106583445792&spm_id_from=333.337.0.0)
