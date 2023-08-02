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

### Fix Docker build issues

Either delete the line with `credsStore` from `~/.docker/config.json`:

```sh
vim ~/.docker/config.json
```

Or simply delete the whole file and restart Docker:

```sh
rm ~/.docker/config.json
```

### Fix Docker build issues

if VSCode's Rust Analyzer extension doesn't work:

```sh
cargo clean
```

### Generate Random password

```sh
openssl rand -base64 36 | pbcopy
```
