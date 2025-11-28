terraform {
  backend "s3" {
    bucket         = "realworld-dev-terraform-state"
    key            = "realworld/terraform.tfstate"
    region         = "eu-central-1"
    dynamodb_table = "realworld-dev-terraform-lock"
    encrypt        = true
  }
}
