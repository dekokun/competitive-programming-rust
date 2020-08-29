#!/bin/bash

set -e

program_name=$(git rev-parse --abbrev-ref HEAD)

if [ "$program_name" == "master" ]; then
    echo "Current branch is master. This program is merge branch that name is program name to master."
    exit 1;
fi

git add "$program_name"/
git commit -m"$program_name pass" --allow-empty
git checkout master
git merge "$program_name"
git branch -d "$program_name"
