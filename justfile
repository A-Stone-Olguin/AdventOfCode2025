set shell := ["bash", "-c"]
set dotenv-load

tar-inputs:
  if [[ -z ${IV:-} ]]; then echo "Please add IV=<VALUE> to your .env file"; exit 1; fi
  if [[ -z ${KEY:-} ]]; then echo "Please add KEY=<VALUE> to your .env file"; exit 1; fi
  tar cz input/20* | openssl enc -e -aes-256-cbc -K "$KEY" -iv "$IV" -out ./input/encrypted_inputs_tar -in -

