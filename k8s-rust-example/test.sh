#!bin/sh

# cargo install grcov
# rustup component add llvm-tools-preview
RUSTFLAGS="-C instrument-coverage" cargo test --tests
grcov . -s . --binary-path ./target/debug/ -t html --branch -o ./target/debug/coverage/

path=$(pwd)
echo "Openning in browser! `open file://$path/target/debug/coverage/index.html`"
##### Alternative #####
# brew install lcov
# grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/coverage/
# genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend ./target/debug/lcov.info