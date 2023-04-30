# `./api`

## Helpful commands

### Update Rust

Rust is continuously improving, to make sure it doesn't get `Rust`y, run:

```sh
rustup update
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
