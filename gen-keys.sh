#!/bin/bash

# -f : force

while getopts "f" opt; do
    case $opt in
        f)
            echo "Force mode enabled"
            rm -rf .keys
            rm -rf crates/attendance/keys
            rm -rf crates/http/keys
            mkdir keys
            openssl ecparam -name secp521r1 -genkey -noout -out keys/private.pem
            openssl ec -in keys/private.pem -pubout -out keys/public.pem

            mkdir -p crates/attendance/keys
            mkdir -p crates/http/keys

            cp keys/private.pem crates/attendance/keys/private.pem
            cp keys/public.pem crates/attendance/keys/public.pem
            cp keys/private.pem crates/http/keys/private.pem
            cp keys/public.pem crates/http/keys/public.pem
            rm -rf keys
            touch .keys
            echo "Keys generated successfully !!"
            exit 0
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            ;;
    esac
done

if [ -f ".keys" ]; then
    echo "Keys already exist !!"
    echo "Please remove the keys directory to generate new keys"
    exit 1
else
    mkdir -p keys
    openssl ecparam -name secp521r1 -genkey -noout -out keys/private.pem
    openssl ec -in private.pem -pubout -out keys/public.pem
    mkdir -p crates/attendance/keys
    mkdir -p crates/http/keys

    cp keys/private.pem crates/attendance/keys/private.pem
    cp keys/public.pem crates/attendance/keys/public.pem
    cp keys/private.pem crates/http/keys/private.pem
    cp keys/public.pem crates/http/keys/public.pem
    rm -rf keys
    touch .keys
    echo "Keys generated successfully !!"
fi