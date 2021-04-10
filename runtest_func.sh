RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

runtest() {
  exec_file=./target/debug/$(cd "$(dirname "$0")" && basename "$(pwd)")
  cargo build

  if ! output=$(echo "$1" | time cargo run); then
    echo -e "${RED}command fail${NC}"
    echo -e "please run command: echo $1 | time cargo run"
  elif [ "$output" != "$2" ]; then
    echo -en "${RED}"
    echo "fail"
    echo "expect                          got"
    diff <(echo "$2") <(echo "$output") -y -W 60
    echo "input: $1"
    echo -en "${NC}"
  else
    echo -e "${GREEN}OK${NC}"
  fi
}

t() {
  runtest "$@"
}
