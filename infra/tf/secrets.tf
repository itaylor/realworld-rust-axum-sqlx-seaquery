resource "random_password" "db_password" {
  length           = 32
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"
}

resource "aws_secretsmanager_secret" "db_password" {
  name_prefix             = "${var.project_name}-${var.environment}-db-password-"
  description             = "RDS PostgreSQL master password"
  recovery_window_in_days = 7

  tags = {
    Name = "${var.project_name}-${var.environment}-db-password"
  }
}

resource "aws_secretsmanager_secret_version" "db_password" {
  secret_id     = aws_secretsmanager_secret.db_password.id
  secret_string = random_password.db_password.result
}

resource "random_password" "jwt_secret" {
  length           = 64
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"
}

resource "aws_secretsmanager_secret" "jwt_secret" {
  name_prefix             = "${var.project_name}-${var.environment}-jwt-secret-"
  description             = "JWT secret key"
  recovery_window_in_days = 7

  tags = {
    Name = "${var.project_name}-${var.environment}-jwt-secret"
  }
}

resource "aws_secretsmanager_secret_version" "jwt_secret" {
  secret_id     = aws_secretsmanager_secret.jwt_secret.id
  secret_string = random_password.jwt_secret.result
}

resource "random_password" "password_pepper" {
  length           = 64
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"

  lifecycle {
    ignore_changes = [
      length,
      special,
      override_special,
    ]
  }
}

resource "aws_secretsmanager_secret" "password_pepper" {
  name_prefix             = "${var.project_name}-${var.environment}-password-pepper-"
  description             = "Password pepper for hashing - NEVER rotate this value"
  recovery_window_in_days = 30

  tags = {
    Name = "${var.project_name}-${var.environment}-password-pepper"
  }
}

resource "aws_secretsmanager_secret_version" "password_pepper" {
  secret_id     = aws_secretsmanager_secret.password_pepper.id
  secret_string = random_password.password_pepper.result

  lifecycle {
    ignore_changes = [secret_string]
  }
}
