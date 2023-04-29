# api

## Helpful commands

### Update Rust

Run:

```sh
rustup update
```

### Fix Docker build issues

Delete the line with `credsStore` from `~/.docker/config.json`, or simply the whole file:

```sh
rm ~/.docker/config.json
```

then restart Docker.
