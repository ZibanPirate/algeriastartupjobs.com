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
  stage          = terraform.workspace
  rootDomainName = "algeriastartupjobs.com"
  subDomainName  = local.stage == "production" ? "api" : "api.${local.stage}"
  domainName     = "${local.subDomainName}.${local.rootDomainName}"
  serviceName    = "${local.subDomainName}.${local.rootDomainName}"
}

provider "digitalocean" {
  token = var.do_api_key
}

resource "digitalocean_droplet" "api" {
  image     = "ubuntu-18-04-x64"
  name      = local.serviceName
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
    EOT
}

resource "digitalocean_project_resources" "api" {
  project = data.terraform_remote_state.shared.outputs.digitalocean_project_id
  resources = [
    digitalocean_droplet.api.urn
  ]
}

resource "ssh_resource" "always_run" {
  triggers = {
    # @TODO-ZM: change to only run when code change
    always_run = "${timestamp()}"
  }

  host        = digitalocean_droplet.api.ipv4_address
  user        = var.do_droplet_user
  private_key = var.do_ssh_key

  commands = [
    "pwd",
    "echo \"Hello There!\""
  ]
}
