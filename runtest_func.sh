#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'
TOP='\033[0G'

runtest() {
  exec_file=./target/debug/$(cd "$(dirname "$0")" && basename "$(pwd)")
  if [ ! -e "$exec_file" ]; then
    cargo build
  fi
  echo -en "${RED}testing...${NC}${TOP}"
  if ! output=$(echo "$1" | time $exec_file); then
    echo -e "${RED}command fail${NC}"
    echo -e "please run command: echo $1 | time cargo run"
  elif [ "$output" != "$2" ]; then
    echo -en "${RED}"
    echo "fail"
    echo "expect                          got"
    max_length=0

    # count column length
    while read -r line; do
      if [ ${#line} -gt $max_length ]; then
        max_length=${#line}
      fi
    done < <(echo "$output")
    while read -r line; do
      if [ ${#line} -gt "$max_length" ]; then
        max_length=${#line}
      fi
    done < <(echo "$2")
    # I don't know what count is appropreate
    length=$((2 * max_length + 10))
    diff <(echo "$2") <(echo "$output") -y -W"$length"
    echo "input: $1"
    echo -en "${NC}"
  else
    echo -e "${GREEN}OK${NC}"
  fi
}

t() {
  runtest "$@"
}
