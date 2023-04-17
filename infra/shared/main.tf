locals {
  rootDomainName    = "algeriastartupjobs.com"
  isSharedWorkspace = terraform.workspace == "shared"
  count             = local.isSharedWorkspace ? 1 : 0
}

provider "aws" {
  region = "eu-west-1"
}

# Needed for creating CloudFront distributions
provider "aws" {
  alias  = "virginia"
  region = "us-east-1"
}

# Shared Route53 zone configuration
resource "aws_route53_zone" "website" {
  count         = local.count
  name          = local.rootDomainName
  force_destroy = true
}

# Output the zone ID
output "route53_zone_id" {
  value = local.isSharedWorkspace ? aws_route53_zone.website[0].id : null
}


resource "aws_acm_certificate" "website" {
  count                     = local.count
  domain_name               = local.rootDomainName
  validation_method         = "DNS"
  subject_alternative_names = ["staging.${local.rootDomainName}", "www.${local.rootDomainName}"]
  lifecycle {
    create_before_destroy = true
  }
  provider = aws.virginia
}

# Output the certificate ARN
output "certificate_arn" {
  value = local.isSharedWorkspace ? aws_acm_certificate.website[0].arn : null
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

resource "aws_acm_certificate_validation" "website" {
  certificate_arn         = aws_acm_certificate.website[0].arn
  validation_record_fqdns = [for record in aws_route53_record.website : record.fqdn]
  provider                = aws.virginia
}

