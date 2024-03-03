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
  root_domain_name        = "algeriastartupjobs.com"
  is_shared_workspace     = terraform.workspace == "shared"
  count                   = local.is_shared_workspace ? 1 : 0
  contact_email_address   = "contact@algeriastartupjobs.com"
  api_root_domain_name    = "api.algeriastartupjobs.com"
  assets_root_domain_name = "assets.algeriastartupjobs.com"
  tags                    = { Project = "algeria-startup-jobs", Environment = terraform.workspace }
  # @TODO-ZM: make this more generic
  email_dns_records = [
    { type : "MX", name : "", value : ["10 mx.zoho.com", "20 mx2.zoho.com", "50 mx3.zoho.com"] },
    { type : "TXT", name : "", value : ["v=spf1 include:zeptomail.net mx include:zoho.com ~all", "google-site-verification=cSr0V7GyACe-BFr9GaKtVFFw6bz_fTuiJ1cJDraevzY"] },
    { type : "TXT", name : "dkim._domainkey", value : ["v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCm72cEJ58s0O+DDFEuYbfofjbNxav37gs4avX784W6s7IOYTdUJcodUCfSUVmb3rPxocVhu5yU3X81BuETG54kg9hgSePx8FANURvkEKbLyyYZZes1g5zhJ1KK7mLkKn5wKnD54WDlrokf6u2TBw9oNU5vlYU1ZHgtxYQ2xmSaQIDAQAB"] },
    { type : "TXT", name : "29611._domainkey", value : ["k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCaK3ZkrLUeTAss2WebIPPDR1hGuDhczxcD0NqGXgSY7ecYO9PRSQaOQh20+rSEIJGhfhDNfYitFcZYolzp64hanJfiBP3JK7yVRrgT1WDa95E6rwZ7syciGRVyWj4wqFK+ys3JHfNguqXyyylktKKhGzMVILrN2xi3OPRc0UDZ7wIDAQAB"] },
    { type : "CNAME", name : "mail", value : ["cluster89.zeptomail.com"] },
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

# Shared Route53 zone configuration
resource "aws_route53_zone" "website" {
  count         = local.count
  name          = local.root_domain_name
  force_destroy = true
  tags          = local.tags
}

# Output the zone ID
output "route53_zone_id" {
  value = local.is_shared_workspace ? aws_route53_zone.website[0].id : null
}

resource "aws_acm_certificate" "website" {
  count                     = local.count
  domain_name               = local.root_domain_name
  validation_method         = "DNS"
  subject_alternative_names = ["*.${local.root_domain_name}"]
  lifecycle {
    create_before_destroy = true
  }
  provider = aws.virginia
  tags     = local.tags
}

# Output the certificate ARN
output "certificate_arn" {
  value = local.is_shared_workspace ? aws_acm_certificate.website[0].arn : null
}

resource "aws_route53_record" "website" {
  for_each = {
    for dvo in aws_acm_certificate.website[0].domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }

  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = aws_route53_zone.website[0].id
}

resource "null_resource" "dns_servers_fetch" {
  count = local.count
  triggers = {
    always_run = timestamp()
  }
  provisioner "local-exec" {
    command = "aws route53 list-resource-record-sets --hosted-zone-id ${aws_route53_zone.website[0].zone_id} --output json > dns_servers.json"
  }
}

resource "null_resource" "dns_servers_prepare" {
  count = local.count
  triggers = {
    always_run = timestamp()
  }
  depends_on = [null_resource.dns_servers_fetch[0]]
  provisioner "local-exec" {
    command = "echo '['$(grep -o '\"ns-[^\"]*\\.\"' dns_servers.json | sed 's/$/,/; $s/,$//')']' > dns_servers.json"
  }
}

resource "null_resource" "dns_servers_clean" {
  count = local.count
  triggers = {
    always_run = timestamp()
  }
  depends_on = [null_resource.dns_servers_prepare[0]]
  provisioner "local-exec" {
    command = "sed -i -e 's/\\.\"/\"/g' dns_servers.json"
  }
}

locals {
  dns_servers = jsondecode(file("${path.module}/dns_servers.json"))
}

resource "namecheap_domain_records" "domain" {
  count  = local.count
  domain = local.root_domain_name
  mode   = "OVERWRITE"

  nameservers = [for ns in local.dns_servers : ns]
}

resource "aws_acm_certificate_validation" "website" {
  depends_on              = [namecheap_domain_records.domain[0]]
  certificate_arn         = aws_acm_certificate.website[0].arn
  validation_record_fqdns = [for record in aws_route53_record.website : record.fqdn]
  provider                = aws.virginia
}

# resource "aws_route53_record" "email" {
#   for_each = {
#     for record in local.email_dns_records : index(local.email_dns_records, record) => {
#       name    = record.name
#       records = record.value
#       type    = record.type
#     }
#   }

#   allow_overwrite = true
#   ttl             = 60
#   name            = each.value.name
#   type            = each.value.type
#   zone_id         = aws_route53_zone.website[0].id
#   records         = each.value.records
# }

# resource "aws_route53_record" "github" {
#   allow_overwrite = true
#   ttl             = 60
#   name            = "_github-challenge-algeriastartupjobs-org.algeriastartupjobs.com"
#   type            = "TXT"
#   zone_id         = aws_route53_zone.website[0].id
#   records         = ["029060ef0f"]
# }

# provider "digitalocean" {
#   token = var.do_api_key
# }

# resource "digitalocean_ssh_key" "api" {
#   count      = local.count
#   name       = "Algeria Startup Jobs Terraform Key"
#   public_key = var.do_ssh_pub_key
# }

# output "digitalocean_ssh_key_fingerprint" {
#   value = local.is_shared_workspace ? digitalocean_ssh_key.api[0].fingerprint : null
# }

# resource "digitalocean_project" "api" {
#   count = local.count
#   name  = "Algeria Startup Jobs"
# }

# output "digitalocean_project_id" {
#   value = local.is_shared_workspace ? digitalocean_project.api[0].id : null
# }

# resource "tls_private_key" "api" {
#   count     = local.count
#   algorithm = "RSA"
# }

# resource "acme_registration" "api" {
#   count           = local.count
#   account_key_pem = tls_private_key.api[0].private_key_pem
#   email_address   = local.contact_email_address
# }

# resource "acme_certificate" "api" {
#   count                     = local.count
#   account_key_pem           = acme_registration.api[0].account_key_pem
#   common_name               = local.root_domain_name
#   subject_alternative_names = ["*.${local.root_domain_name}", "*.${local.api_root_domain_name}"]

#   dns_challenge {
#     provider = "route53"

#     config = {
#       AWS_HOSTED_ZONE_ID = aws_route53_zone.website[0].id
#     }
#   }

#   depends_on = [acme_registration.api[0]]
# }

# output "acme_certificate_api_certificate_pem" {
#   value     = local.is_shared_workspace ? acme_certificate.api[0].certificate_pem : null
#   sensitive = true
# }

# output "acme_certificate_api_private_key_pem" {
#   value     = local.is_shared_workspace ? acme_certificate.api[0].private_key_pem : null
#   sensitive = true
# }
