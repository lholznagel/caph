resource "aws_vpc" "self" {
  cidr_block = "10.10.0.0/16"

  enable_dns_support   = true
  enable_dns_hostnames = true

  tags = {
    Name = "acrux"
  }
}

resource "aws_default_security_group" "self" {
  vpc_id = aws_vpc.self.id

  tags = {
    Name = "default"
  }
}

resource "aws_vpc_dhcp_options" "self" {
  domain_name         = var.domain
  domain_name_servers = ["AmazonProvidedDNS"]

  # NTP-Server from AWS
  ntp_servers = ["169.254.169.123"]

  tags = {
    Name = var.domain
  }
}

resource "aws_vpc_dhcp_options_association" "self" {
  vpc_id          = aws_vpc.self.id
  dhcp_options_id = aws_vpc_dhcp_options.self.id
}

resource "aws_default_network_acl" "self" {
  default_network_acl_id = aws_vpc.self.default_network_acl_id

  ingress {
    protocol   = -1
    rule_no    = 100
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
  }

  egress {
    protocol   = -1
    rule_no    = 100
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
  }

  tags = {
    Name = "default"
  }
}

resource "aws_iam_role" "self" {
  name = "VpcFlow"

  assume_role_policy = <<POLICY
{
  "Version": "2012-10-17",
  "Statement": [{
    "Effect": "Allow",
    "Principal": {
      "Service": "vpc-flow-logs.amazonaws.com"
    },
    "Action": "sts:AssumeRole"
  }]
}
POLICY
}

resource "aws_iam_role_policy" "self" {
  name = "cloudwatch-flow-logs"
  role = aws_iam_role.self.id

  policy = <<POLICY
{
  "Version": "2012-10-17",
  "Statement": [{
    "Action": [
      "logs:CreateLogStream",
      "logs:DescribeLogGroups",
      "logs:DescribeLogStreams",
      "logs:PutLogEvents"
    ],
    "Effect": "Allow",
    "Resource": "arn:aws:logs:${var.region}:${data.aws_caller_identity.self.account_id}:log-group:flowlogs:*"
  }]
}
POLICY
}

resource "aws_cloudwatch_log_group" "self" {
  name = "/vpc/flowlog"

  retention_in_days = 7
}

resource "aws_flow_log" "self" {
  vpc_id          = aws_vpc.self.id
  iam_role_arn    = aws_iam_role.self.arn
  log_destination = aws_cloudwatch_log_group.self.arn

  traffic_type = "ALL"
}

resource "aws_internet_gateway" "self" {
  vpc_id = aws_vpc.self.id

  tags = {
    Name = "Internet"
  }
}

resource "aws_default_route_table" "self" {
  default_route_table_id = aws_vpc.self.main_route_table_id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.self.id
  }

  tags = {
    name = "default"
  }
}

resource "aws_subnet" "dmz" {
  vpc_id = aws_vpc.self.id

  cidr_block        = "10.10.0.0/24"
  availability_zone = format("%sa", var.region)

  map_public_ip_on_launch = true

  tags = {
    Name = "dmz-a"
  }
}

resource "aws_subnet" "infra" {
  vpc_id = aws_vpc.self.id

  cidr_block        = "10.10.1.0/24"
  availability_zone = format("%sa", var.region)

  tags = {
    Name = "infra-a"
  }
}
