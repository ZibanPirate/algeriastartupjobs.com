# `./api`

## Helpful commands

### Update Rust

Rust is continuously improving, to make sure it doesn't get `Rust`y, run:

```sh
rustup update
```

### Fix Docker build issues

Delete the line with `credsStore` from `~/.docker/config.json`, or simply the whole file:

```sh
rm ~/.docker/config.json
```

then restart Docker.
