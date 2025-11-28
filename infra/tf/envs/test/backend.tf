terraform {
  backend "s3" {
    bucket         = "realworld-test-terraform-state"
    key            = "realworld/terraform.tfstate"
    region         = "eu-central-1"
    dynamodb_table = "realworld-test-terraform-lock"
    encrypt        = true
  }
}
