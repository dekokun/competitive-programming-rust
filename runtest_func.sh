RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

runtest() {
  exec_file=./target/debug/$(cd "$(dirname "$0")" && basename "$(pwd)")
  if [ ! -e "$exec_file" ]; then
    cargo build
  fi
  output=$(echo "$1" | time $exec_file)
  if [ "$?" -ne 0 ]; then
    echo -e "${RED}command fail${NC}"
    echo -e "please run command: echo $1 | time cargo run"
  elif [ "$output" != "$2" ]; then
    echo -e "${RED}fail"
    echo "input: $1"
    echo
    echo -e "$2 \nexpected. but got \n$output${NC}"
  else
    echo -e "${GREEN}OK${NC}"
  fi
}

t() {
  runtest "$@"
}
