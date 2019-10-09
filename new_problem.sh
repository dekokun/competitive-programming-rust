#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo 'Argument is not exists.'
    exit 1
fi

branch=$(git rev-parse --abbrev-ref HEAD)

if [ "$branch" != "master" ]; then
    echo "Please checkout master branch."
    exit 1;
fi

program_name="$1"
cargo new "$program_name" --bin
git checkout -b "$program_name"
cp -pr template/src/main.rs "$program_name"/src
cp -pr template/runtest "$program_name"/runtest
cp -pr template/rust-toolchain "$program_name"/rust-toolchain.bak
mkdir "$program_name"/.vscode/
cargo snippet snippets -t vscode > "$program_name"/.vscode/rs.code-snippets
git add "$program_name"
git commit -m"$program_name initialize"
code "$program_name"
echo "if you want to test, run below command"
echo "$program_name/runtest"
