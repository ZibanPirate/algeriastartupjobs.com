data "terraform_remote_state" "shared" {
  backend = "local"
  config  = { path = "${path.module}/../shared/terraform.tfstate.d/shared/terraform.tfstate" }
}

locals {
  stage         = terraform.workspace
  subDomainName = local.stage == "production" ? "www" : local.stage
  domainName    = "${local.subDomainName}.algeriastartupjobs.com"
  bucketName    = "${local.subDomainName}.algeriastartupjobs.com"
}

provider "aws" {
  region = "eu-west-1"
}

resource "aws_s3_bucket" "website" {
  bucket        = local.bucketName
  force_destroy = true
}

resource "aws_s3_bucket_website_configuration" "website" {
  bucket = aws_s3_bucket.website.bucket
  index_document {
    suffix = "index.html"
  }
  error_document {
    key = "index.html"
  }
}

resource "aws_s3_bucket_policy" "website" {
  bucket = aws_s3_bucket.website.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        "Sid" : "PublicReadGetObject",
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : "s3:GetObject",
        "Resource" : "arn:aws:s3:::${local.bucketName}/*"
      }
    ]
  })
}

# https://stackoverflow.com/a/57457344
resource "null_resource" "remove_and_upload_website_to_s3" {
  provisioner "local-exec" {
    command = "aws s3 sync ${path.module}/../../web/dist s3://${aws_s3_bucket.website.id}"
  }
}

resource "aws_cloudfront_origin_access_identity" "website" {}

# TODO-ZM: remove PublicReadGetObject from aws_s3_bucket_website_configuration?
resource "aws_cloudfront_distribution" "website" {
  origin {
    domain_name = aws_s3_bucket.website.bucket_regional_domain_name
    origin_id   = "S3-${aws_s3_bucket.website.bucket}"
    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.website.cloudfront_access_identity_path
    }
  }

  default_root_object = "index.html"
  enabled             = true
  is_ipv6_enabled     = true
  # aliases             = [local.domainName]

  # TODO-ZM: remove index_document and error_document from aws_s3_bucket_website_configuration?
  custom_error_response {
    error_caching_min_ttl = 3000
    error_code            = 404
    response_code         = 200
    response_page_path    = "/index.html"
  }

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "S3-${aws_s3_bucket.website.bucket}"

    forwarded_values {
      query_string = true
      cookies {
        forward = "none"
      }
    }
    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  price_class = "PriceClass_100"
  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn = data.terraform_remote_state.shared.outputs.certificate_arn
    ssl_support_method  = "sni-only"
  }
}

resource "aws_route53_record" "website-a" {
  zone_id = data.terraform_remote_state.shared.outputs.route53_zone_id
  name    = local.domainName
  type    = "A"
  alias {
    name                   = aws_cloudfront_distribution.website.domain_name
    zone_id                = aws_cloudfront_distribution.website.hosted_zone_id
    evaluate_target_health = false
  }
}
