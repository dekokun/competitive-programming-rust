RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

runtest() {
  output=$(echo "$1" | time cargo run -q)
  if [ "$?" -ne 0 ]; then
    echo -e "${RED}command fail${NC}"
    echo -e "please run command: echo $1 | time cargo run"
  elif [ "$output" != "$2" ]; then
    echo -e "${RED}fail"
    echo "input: $1"
    echo
    echo -e "$2 expected. but got $output${NC}"
  else
    echo -e "${GREEN}OK${NC}"
  fi
}

t() {
  runtest "$@"
}
