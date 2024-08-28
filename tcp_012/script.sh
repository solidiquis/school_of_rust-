for _ in {0..10}; do
  nc localhost 3333 <<EOF &
hello
EOF
done
