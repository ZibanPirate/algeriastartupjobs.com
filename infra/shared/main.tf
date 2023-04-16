locals {
  sharedName        = "shared.algeriastartupjobs.com"
  isSharedWorkspace = terraform.workspace == "shared"
  count             = local.isSharedWorkspace ? 1 : 0
}

provider "aws" {
  region = "eu-west-1"
}

# Shared Route53 zone configuration
resource "aws_route53_zone" "website" {
  count = local.count
  name  = local.sharedName
}

# Output the zone ID
output "route53_zone_id" {
  value = local.isSharedWorkspace ? aws_route53_zone.website[0].id : null
}
