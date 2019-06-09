#!/bin/bash

if [ -z "$1" ]; then
    echo 'Argument is not exists.'
    exit 1
fi

program_name="$1"
cargo new abc127_e --bin
mkdir "$program_name"/src
mkdir "$program_name"/runtest
cp -pr template/src/main.rs "$program_name"/src
cp -pr template/runtest "$program_name"/runtest
