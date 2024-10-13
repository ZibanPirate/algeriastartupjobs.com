data "terraform_remote_state" "shared-route53" {
  backend = "local"
  config  = { path = "${path.module}/../shared-route53/terraform.tfstate.d/shared/terraform.tfstate" }
}

variable "namecheap_api_user" {
  description = "The API user for Namecheap"
  type        = string
  sensitive   = true
}

variable "namecheap_api_key" {
  description = "The API key for Namecheap"
  type        = string
  sensitive   = true
}

variable "namecheap_client_ip" {
  description = "The client IP for Namecheap"
  type        = string
  sensitive   = true
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

locals {
  root_domain_name        = "magiframe.com"
  is_shared_workspace     = terraform.workspace == "shared"
  count                   = local.is_shared_workspace ? 1 : 0
  contact_email_address   = "contact@magiframe.com"
  api_root_domain_name    = "api.magiframe.com"
  assets_root_domain_name = "assets.magiframe.com"
  tags                    = { Project = "algeria-startup-jobs", Environment = terraform.workspace }
  # @TODO-ZM: make this more generic
  email_dns_records = [
    { type : "MX", name : "", value : ["10 mx.zoho.com", "20 mx2.zoho.com", "50 mx3.zoho.com"] },
    { type : "TXT", name : "", value : ["v=spf1 include:zeptomail.net mx include:zoho.com ~all", "google-site-verification=cSr0V7GyACe-BFr9GaKtVFFw6bz_fTuiJ1cJDraevzY"] },
    { type : "TXT", name : "dkim._domainkey", value : ["v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCm72cEJ58s0O+DDFEuYbfofjbNxav37gs4avX784W6s7IOYTdUJcodUCfSUVmb3rPxocVhu5yU3X81BuETG54kg9hgSePx8FANURvkEKbLyyYZZes1g5zhJ1KK7mLkKn5wKnD54WDlrokf6u2TBw9oNU5vlYU1ZHgtxYQ2xmSaQIDAQAB"] },
    { type : "TXT", name : "12182127._domainkey", value : ["k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQChtRggTaZ76bLzMWQWUi6AVQTWDqSRDyUJ0qBQEaZu14szegFLuvG6V1WEjEDaslwfwqGAdEO0cyPr9tV+K5NhYSF4g/86FqNiQnt4v/qjDPymuFv1gpE1tyTGxzlOUumrW1rf330/ksx/myxd1QSv6jBoyw5oZyYMba/Dn7yIBQIDAQAB"] },
    { type : "CNAME", name : "bounce", value : ["cluster89.zeptomail.com"] },
  ]
}

terraform {
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
    acme = {
      source  = "vancluever/acme"
      version = "~> 2.5.3"
    }
    namecheap = {
      source  = "namecheap/namecheap"
      version = ">= 2.0.0"
    }
  }
}

# Namecheap API credentials
provider "namecheap" {
  user_name   = var.namecheap_api_user
  api_user    = var.namecheap_api_user
  api_key     = var.namecheap_api_key
  client_ip   = var.namecheap_client_ip
  use_sandbox = false
}

provider "aws" {
  region = "eu-west-1"
}

# Needed for creating CloudFront distributions
provider "aws" {
  alias  = "virginia"
  region = "us-east-1"
}

provider "acme" {
  # server_url = "https://acme-staging-v02.api.letsencrypt.org/directory"
  server_url = "https://acme-v02.api.letsencrypt.org/directory"
}

# Output the zone ID
output "route53_zone_id" {
  value = local.is_shared_workspace ? data.terraform_remote_state.shared-route53.outputs.route53_zone_id : null
}

# Output the certificate ARN
output "certificate_arn" {
  value = local.is_shared_workspace ? data.terraform_remote_state.shared-route53.outputs.certificate_arn : null
}

locals {
  dns_servers = jsondecode(file("${path.module}/../shared-route53/dns_servers.json"))
}

resource "namecheap_domain_records" "domain" {
  count  = local.count
  domain = local.root_domain_name
  mode   = "OVERWRITE"

  nameservers = [for ns in local.dns_servers : ns]

  depends_on = []
}

resource "aws_acm_certificate_validation" "website" {
  certificate_arn         = data.terraform_remote_state.shared-route53.outputs.certificate_arn
  validation_record_fqdns = [for record in data.terraform_remote_state.shared-route53.outputs.dns_records : record.fqdn]
  provider                = aws.virginia

  depends_on = [namecheap_domain_records.domain[0]]
  timeouts {
    create = "1m"
  }
}

resource "aws_route53_record" "email" {
  for_each = {
    for record in local.email_dns_records : index(local.email_dns_records, record) => {
      name    = record.name
      records = record.value
      type    = record.type
    }
  }

  allow_overwrite = true
  ttl             = 60
  name            = each.value.name
  type            = each.value.type
  zone_id         = data.terraform_remote_state.shared-route53.outputs.route53_zone_id
  records         = each.value.records
}

provider "digitalocean" {
  token = var.do_api_key
}

resource "digitalocean_ssh_key" "api" {
  count      = local.count
  name       = "Algeria Startup Jobs Terraform Key"
  public_key = var.do_ssh_pub_key
}

output "digitalocean_ssh_key_fingerprint" {
  value = local.is_shared_workspace ? digitalocean_ssh_key.api[0].fingerprint : null
}

resource "digitalocean_project" "api" {
  count = local.count
  name  = "Algeria Startup Jobs"
}

output "digitalocean_project_id" {
  value = local.is_shared_workspace ? digitalocean_project.api[0].id : null
}

resource "tls_private_key" "api" {
  count     = local.count
  algorithm = "RSA"
}

resource "acme_registration" "api" {
  count           = local.count
  account_key_pem = tls_private_key.api[0].private_key_pem
  email_address   = local.contact_email_address
}

resource "acme_certificate" "api" {
  count                     = local.count
  account_key_pem           = acme_registration.api[0].account_key_pem
  common_name               = local.root_domain_name
  subject_alternative_names = ["*.${local.root_domain_name}", "*.${local.api_root_domain_name}"]

  dns_challenge {
    provider = "route53"

    config = {
      AWS_HOSTED_ZONE_ID = data.terraform_remote_state.shared-route53.outputs.route53_zone_id
    }
  }

  depends_on = [acme_registration.api[0]]
}

output "acme_certificate_api_certificate_pem" {
  value     = local.is_shared_workspace ? acme_certificate.api[0].certificate_pem : null
  sensitive = true

  depends_on = [acme_certificate.api[0]]
}

output "acme_certificate_api_private_key_pem" {
  value     = local.is_shared_workspace ? acme_certificate.api[0].private_key_pem : null
  sensitive = true

  depends_on = [acme_certificate.api[0]]
}
