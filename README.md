# What
The goal of this project is to generate a simplified version of the `grep` command following the chapter 12 of [The Rust Book](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html).

# How to run
In your terminal you need to call
```bash
cargo run -- <query> <file_path>
```

As an example:
```bash
cargo run -- the files/poem.txt
```

If you want the search to be case insensitive you need to define the IGNORE_CASE=true before calling the script as follows:

```bash
IGNORE_CASE=true cargo run -- the files/poem.txt
```

# How to test
In your terminal run:
```bash
cargo test
```
