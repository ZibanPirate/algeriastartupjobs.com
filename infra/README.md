# `./infra`

## Helpful commands

# SSH into `./api` VPS

```sh
ssh -i [asj-ssh-file] [do_droplet_user]@[droplet-ip-address]
```

# Check `./api` service

Status:

```sh
sudo systemctl status algeriastartupjobs-api
```

Logs:

```sh
journalctl -u algeriastartupjobs-api
```
