terraform {
  required_providers {
    ssh = {
      source = "loafoe/ssh"
    }
  }
}

locals {
  stage               = terraform.workspace
  api_app_folder_name = "dzjob"
  api_app_folder      = "/home/${var.do_droplet_user}/${local.api_app_folder_name}"
  service_name        = "dzjob-api"
}

data "terraform_remote_state" "api" {
  backend = "local"
  config  = { path = "${path.module}/../terraform.tfstate.d/${local.stage}/terraform.tfstate" }
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

resource "ssh_resource" "stop_api" {
  triggers = {
    always = timestamp()
  }

  host        = data.terraform_remote_state.api.outputs.digitalocean_droplet_api_ipv4_address
  user        = var.do_droplet_user
  private_key = var.do_ssh_key
  timeout     = "5m"

  commands = [
    "sudo mkdir -p ${local.api_app_folder}/sqlite_db_data || true",
    "sudo systemctl stop nginx || true",
    "sudo systemctl stop ${local.service_name} || true",
  ]
}


resource "null_resource" "create_tmp_private_key" {
  triggers = {
    always = timestamp()
  }
  depends_on = [ssh_resource.stop_api]

  provisioner "local-exec" {
    command = "echo '${var.do_ssh_key}' > ${path.module}/tmp_private_key && chmod 600 ${path.module}/tmp_private_key && mkdir -p ./sqlite_db_data/${local.stage}"
  }
}


resource "null_resource" "download_db_from_vps" {
  triggers = {
    always = timestamp()
  }
  depends_on = [null_resource.create_tmp_private_key]

  provisioner "local-exec" {
    command = "scp -i ${path.module}/tmp_private_key -o StrictHostKeyChecking=no -r ${var.do_droplet_user}@${data.terraform_remote_state.api.outputs.digitalocean_droplet_api_ipv4_address}:${local.api_app_folder}/sqlite_db_data ${path.module}/sqlite_db_data/${local.stage}"
  }
}

resource "null_resource" "delete_tmp_private_key" {
  triggers = {
    always = timestamp()
  }
  depends_on = [null_resource.download_db_from_vps]

  provisioner "local-exec" {
    command = "rm ${path.module}/tmp_private_key"
  }
}

resource "ssh_resource" "start_api" {
  triggers = {
    always = timestamp()
  }
  depends_on = [null_resource.delete_tmp_private_key]

  host        = data.terraform_remote_state.api.outputs.digitalocean_droplet_api_ipv4_address
  user        = var.do_droplet_user
  private_key = var.do_ssh_key
  timeout     = "5m"

  commands = [
    "sudo mkdir -p ${local.api_app_folder}/sqlite_db_data || true",
    "sudo systemctl daemon-reload",
    "sudo systemctl start ${local.service_name} || true",
    "sudo systemctl start nginx",
  ]
}

