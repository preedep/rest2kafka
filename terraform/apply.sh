terraform plan -out=dev-plan -var-file="dev-variables.tfvars"
terraform apply dev-plan
