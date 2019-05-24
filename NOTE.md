# 実行環境

```bash
rustup --version
#=> rustup 1.18.3 (435397f48 2019-05-22)

rustc --version
#=> rustc 1.35.0 (3c235d560 2019-05-20)

cargo --version
#=> cargo 1.35.0 (6f3e9c367 2019-04-04)
```

# 初回セットアップ

```bash
mkdir rust-hello-world && cd $_
cargo init --name hello --bin

# setup linter (clippy)
rustup component add clippy
cargo clippy

# setup formatter (rustfmt)
rustup component add rustfmt
cargo fmt

# setup pre-commit
cargo install 
```

# 参考

* The Rust Programming Language - The Rust Programming Language - https://doc.rust-lang.org/book/
* Cargo Book - https://doc.rust-lang.org/cargo/
* rust-lang/rust-clippy: A bunch of lints to catch common mistakes and improve your Rust code - https://github.com/rust-lang/rust-clippy
* rust-lang/rustfmt: Format Rust code - https://github.com/rust-lang/rustfmt