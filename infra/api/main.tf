data "terraform_remote_state" "shared" {
  backend = "local"
  config  = { path = "${path.module}/../shared/terraform.tfstate.d/shared/terraform.tfstate" }
}

terraform {
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
    ssh = {
      source = "loafoe/ssh"
    }
  }
}

variable "do_api_key" {
  description = "The API key for the DigitalOcean account"
  type        = string
  sensitive   = true
}

variable "do_ssh_pub_key" {
  description = "The SSH key for the DigitalOcean droptlet"
  type        = string
  sensitive   = true
}

variable "do_ssh_key" {
  description = "The private SSH key for the DigitalOcean droptlet"
  type        = string
  sensitive   = true
}

variable "do_droplet_user" {
  description = "The user for the DigitalOcean droplet"
  type        = string
  sensitive   = true
}

variable "do_droplet_password" {
  description = "The password for the DigitalOcean droplet's user"
  type        = string
  sensitive   = true
}

locals {
  app_port             = "9090"
  app_folder_name      = "asj"
  home                 = "/home/${var.do_droplet_user}"
  app_folder           = "${local.home}/${local.app_folder_name}"
  certificate_folder   = "/etc/ssl/certs"
  app_name             = "algeriastartupjobs-api"
  service_name         = "algeriastartupjobs-api"
  stage                = terraform.workspace
  root_domain_name     = "api.algeriastartupjobs.com"
  web_root_domain_name = "algeriastartupjobs.com"
  sub_domain_name      = local.stage
  domain_name          = "${local.sub_domain_name}.${local.root_domain_name}"
  web_domain_name      = "${local.stage == "production" ? "www" : "staging"}.${local.web_root_domain_name}"
}

provider "digitalocean" {
  token = var.do_api_key
}

provider "aws" {
  region = "eu-west-1"
}

resource "digitalocean_droplet" "api" {
  image     = "ubuntu-22-04-x64"
  name      = local.domain_name
  region    = "fra1"
  size      = "s-1vcpu-512mb-10gb"
  ssh_keys  = [data.terraform_remote_state.shared.outputs.digitalocean_ssh_key_fingerprint]
  user_data = <<EOT
    #cloud-config
    users:
      - name: ${var.do_droplet_user}
        ssh-authorized-keys:
          - ${var.do_ssh_pub_key}
        sudo: ['ALL=(ALL) NOPASSWD:ALL']
        groups: sudo
        shell: /bin/bash
    write_files:
    - content: |
        [Unit]
        Description=Algeria Startup Jobs API
        After=network.target

        [Service]
        ExecStart=sudo /home/${var.do_droplet_user}/${local.app_folder_name}/${local.app_name}
        StandardOutput=syslog
        SyslogIdentifier=${local.service_name}
        WorkingDirectory=/home/${var.do_droplet_user}/${local.app_folder_name}
        Restart=always
        RestartSec=5
        User=${var.do_droplet_user}

        [Install]
        WantedBy=multi-user.target
      path: /etc/systemd/system/${local.service_name}.service
    runcmd:
      - sudo apt update
      - sudo apt install nginx -y
      - sudo ufw allow 'Nginx HTTP'
      - sudo sh -c "echo '
          server {
              listen 80;
              server_name ${local.domain_name};
              return 301 https://\$server_name\$request_uri;
          }
          server {
              listen 443 ssl;
              server_name ${local.domain_name};
              ssl_certificate ${local.certificate_folder}/${local.service_name}.crt;
              ssl_certificate_key ${local.certificate_folder}/${local.service_name}.key;

              proxy_buffering off;

              location / {
                  if (\$host = ${local.web_root_domain_name}) {
                      return 301 https://www.${local.web_root_domain_name}\$request_uri;
                  }
                  if (\$host != ${local.domain_name}) {
                      rewrite ^(.*)\$ /web\$1 break;
                  }

                  proxy_pass http://127.0.0.1:${local.app_port};
                  proxy_set_header Host \$host;
                  proxy_set_header X-Real-IP \$remote_addr;
                  proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
                  proxy_set_header X-Forwarded-Proto \$scheme;
              }
          }' > /etc/nginx/sites-available/${local.domain_name}.conf"
      - sudo rm /etc/nginx/sites-enabled/*
      - sudo ln -s /etc/nginx/sites-available/${local.domain_name}.conf /etc/nginx/sites-enabled/
      - sudo systemctl enable nginx
      - sudo systemctl start nginx
      - sudo systemctl daemon-reload
      - sudo systemctl start ${local.service_name}
    EOT
}

output "digitalocean_droplet_api_ipv4_address" {
  value     = digitalocean_droplet.api.ipv4_address
  sensitive = true
}

resource "ssh_resource" "upload_ssl_certificates_to_vps" {
  triggers = {
    vps_id = digitalocean_droplet.api.id
    cert   = sha256(data.terraform_remote_state.shared.outputs.acme_certificate_api_certificate_pem)
  }

  host        = digitalocean_droplet.api.ipv4_address
  user        = var.do_droplet_user
  private_key = var.do_ssh_key
  timeout     = "2m"

  commands = [
    "sudo sh -c \"echo '${data.terraform_remote_state.shared.outputs.acme_certificate_api_certificate_pem}' > ${local.certificate_folder}/${local.service_name}.crt\"",
    "sudo sh -c \"echo '${data.terraform_remote_state.shared.outputs.acme_certificate_api_private_key_pem}' > ${local.certificate_folder}/${local.service_name}.key\""
  ]
}

resource "digitalocean_project_resources" "api" {
  project = data.terraform_remote_state.shared.outputs.digitalocean_project_id
  resources = [
    digitalocean_droplet.api.urn
  ]
}

resource "aws_route53_record" "api" {
  zone_id = data.terraform_remote_state.shared.outputs.route53_zone_id
  name    = local.domain_name
  type    = "A"
  ttl     = "300"
  records = [digitalocean_droplet.api.ipv4_address]
}

resource "aws_route53_record" "web" {
  zone_id = data.terraform_remote_state.shared.outputs.route53_zone_id
  name    = local.web_domain_name
  type    = "A"
  ttl     = "300"
  records = [digitalocean_droplet.api.ipv4_address]
}

resource "aws_route53_record" "web-naked" {
  count   = local.stage == "production" ? 1 : 0
  zone_id = data.terraform_remote_state.shared.outputs.route53_zone_id
  name    = local.web_root_domain_name
  type    = "A"
  ttl     = "300"
  records = [digitalocean_droplet.api.ipv4_address]
}

data "terraform_remote_state" "ructc" {
  backend = "local"
  config  = { path = "${path.module}/build-on-vps/terraform.tfstate" }
}

resource "ssh_resource" "upload_app_and_deps_to_vps" {
  triggers = {
    vps_ip = data.terraform_remote_state.ructc.outputs.digitalocean_droplet_rustc_ipv4_address
  }

  host        = digitalocean_droplet.api.ipv4_address
  user        = var.do_droplet_user
  private_key = var.do_ssh_key
  timeout     = "5m"

  commands = [
    #
    "sudo mkdir -p ${local.app_folder} || true",
    #
    "sudo touch ${local.app_folder}/.env",
    "sudo sh -c \"echo '${filebase64("${path.module}/../../api/${local.stage}.env")}' > ${local.app_folder}/base64.env\"",
    "sudo sh -c \"base64 -d ${local.app_folder}/base64.env > ${local.app_folder}/.env\"",
    #
    "sudo systemctl stop nginx || true",
    "sudo systemctl stop ${local.service_name} || true",
    #
    "sudo curl -o ${local.app_folder}/${local.app_name} http://${data.terraform_remote_state.ructc.outputs.digitalocean_droplet_rustc_ipv4_address}:8000/target/release/algeriastartupjobs-api",
    "sudo chmod +x ${local.app_folder}/${local.app_name} || true",
    #
    "sudo systemctl daemon-reload",
    "sudo systemctl start ${local.service_name} || true",
    "sudo systemctl start nginx",
  ]
}
