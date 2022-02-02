terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.14.0"
    }
  }
}

locals {
  region = "eu-central-1"
  domain = "acrux.space"

  nat_instance_id = "ami-0ab5c3367027f792e"
}

provider "aws" {
  region  = local.region
  profile = "caph"
}

module "vpc" {
  source = "./modules/vpc"

  region = local.region
  domain = local.domain
}
