data "terraform_remote_state" "shared" {
  backend = "local"
  config  = { path = "${path.module}/../../shared/terraform.tfstate.d/shared/terraform.tfstate" }
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

locals {
  user     = "root"
  code_dir = "~/code"
}

provider "digitalocean" {
  token = var.do_api_key
}

resource "digitalocean_droplet" "rustc" {
  image    = "ubuntu-22-04-x64"
  name     = "build-rust-app-droplet"
  region   = "fra1"
  size     = "c-4-intel"
  ssh_keys = [data.terraform_remote_state.shared.outputs.digitalocean_ssh_key_fingerprint]
}

resource "digitalocean_project_resources" "api" {
  project = data.terraform_remote_state.shared.outputs.digitalocean_project_id
  resources = [
    digitalocean_droplet.rustc.urn
  ]
}

resource "ssh_resource" "setup" {
  triggers = {
    vps_id = digitalocean_droplet.rustc.id
  }

  host        = digitalocean_droplet.rustc.ipv4_address
  user        = local.user
  private_key = var.do_ssh_key
  timeout     = "5m"

  depends_on = [
    digitalocean_droplet.rustc
  ]

  commands = [
    "sudo apt-get update -y && sudo apt-get install -y build-essential unzip pkg-config libssl-dev python3",
    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain nightly"
  ]
}

data "archive_file" "src_zipped" {
  type        = "zip"
  source_dir  = "${path.module}/../../../api/src"
  output_path = "${path.module}/src_zip.zip"
}

data "archive_file" "db_zipped" {
  type        = "zip"
  source_dir  = "${path.module}/../../../api/db"
  output_path = "${path.module}/db_zip.zip"
}

resource "ssh_resource" "upload_code_to_vps" {
  triggers = {
    always = timestamp()
  }

  depends_on = [
    ssh_resource.setup
  ]

  host        = digitalocean_droplet.rustc.ipv4_address
  user        = local.user
  private_key = var.do_ssh_key
  timeout     = "1m"

  pre_commands = [
    "mkdir -p ${local.code_dir}/src",
    "mkdir -p ${local.code_dir}/db",
  ]

  file {
    source      = "${path.module}/../../../api/Cargo.toml"
    destination = "${local.code_dir}/Cargo.toml"
  }

  file {
    source      = "${path.module}/../../../api/Cargo.lock"
    destination = "${local.code_dir}/Cargo.lock"
  }

  file {
    source      = "${path.module}/src_zip.zip"
    destination = "${local.code_dir}/src.zip"
  }

  file {
    source      = "${path.module}/db_zip.zip"
    destination = "${local.code_dir}/db.zip"
  }

  commands = [
    "unzip -o ${local.code_dir}/src.zip -d ${local.code_dir}/src",
    "rm ${local.code_dir}/src.zip",
    "unzip -o ${local.code_dir}/db.zip -d ${local.code_dir}/db",
    "rm ${local.code_dir}/db.zip",
  ]
}

resource "ssh_resource" "release" {
  triggers = {
    always = timestamp()
  }

  depends_on = [
    ssh_resource.upload_code_to_vps
  ]

  host        = digitalocean_droplet.rustc.ipv4_address
  user        = local.user
  private_key = var.do_ssh_key
  timeout     = "15m"

  commands = [
    "killall python3 || true",
    "screen -dm python3 -m http.server --directory ${local.code_dir}",
    "cd ${local.code_dir} && $HOME/.cargo/bin/cargo build --release"
  ]
}

output "digitalocean_droplet_rustc_ipv4_address" {
  value = digitalocean_droplet.rustc.ipv4_address
}

# resource "null_resource" "download_release" {
#   triggers = {
#     always = timestamp(),
#   }

#   depends_on = [
#     ssh_resource.release
#   ]

#   provisioner "local-exec" {
#     command = "curl -o ${path.module}/algeriastartupjobs-api http://${digitalocean_droplet.rustc.ipv4_address}:8000/target/release/algeriastartupjobs-api"
#   }
# }
