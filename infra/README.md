# `./infra`

## Helpful commands

# SSH into `./api` VPS

```sh
ssh -i [dzjob-ssh-file] [do_droplet_user]@[droplet-ip-address]
```

# Check `./api` service

Status:

```sh
sudo systemctl status dzjob-api
```

Logs:

```sh
journalctl -u dzjob-api
```
