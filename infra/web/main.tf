data "terraform_remote_state" "shared" {
  backend = "local"
  config  = { path = "${path.module}/../shared/terraform.tfstate.d/shared/terraform.tfstate" }
}

locals {
  stage      = terraform.workspace
  bucketName = "${local.stage}.algeriastartupjobs.com"
  domainName = "${local.stage == "production" ? "www" : local.stage}.algeriastartupjobs.com"
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

resource "aws_route53_record" "website-a" {
  zone_id = data.terraform_remote_state.shared.outputs.route53_zone_id
  name    = local.domainName
  type    = "A"
  alias {
    name                   = aws_s3_bucket.website.website_endpoint
    zone_id                = aws_s3_bucket.website.hosted_zone_id
    evaluate_target_health = true
  }
}
