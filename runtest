# compile check
cargo build || exit 1

runtest() {
  output=$(echo "$1" | cargo run -q)
  if [ "$output" != "$2" ]; then
    echo fail
    echo "input: $1"
    echo
    echo "$2 expected. but got $output"
  else
    echo OK
  fi
}

