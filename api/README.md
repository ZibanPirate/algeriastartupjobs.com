# `./api`

## Helpful commands

### Update Rust

Rust is continuously improving, to make sure it doesn't get `Rust`y, run:

```sh
rustup update
```

### Update Dependencies

```sh
cargo update
```

### Fix Rust Analyzer issues

if VSCode's Rust Analyzer extension doesn't work:

```sh
cargo clean
```

### Generate Random password

```sh
openssl rand -base64 36 | pbcopy
```
