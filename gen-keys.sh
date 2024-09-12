#!/bin/bash

# -f : force

while getopts "f" opt; do
    case $opt in
        f)
            echo "Force mode enabled"
            rm -rf keys
            mkdir keys
            openssl ecparam -name secp521r1 -genkey -noout -out keys/private.pem
            openssl ec -in keys/private.pem -pubout -out keys/public.pem
            echo "Keys generated successfully !!"
            exit 0
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            ;;
    esac
done

if [ -d "keys" ]; then
    echo "Keys already exist !!"
    echo "Please remove the keys directory to generate new keys"
    exit 1
else
    mkdir keys
    openssl ecparam -name secp521r1 -genkey -noout -out keys/private.pem
    openssl ec -in private.pem -pubout -out keys/public.pem
    echo "Keys generated successfully !!"
fi