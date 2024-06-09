sudo rm -rf ./dbcTemp
cargo build
./target/debug/data-mover  -f postgresql://user:password@127.0.0.1:5433/test?schema=public -t postgresql://user:password@127.0.0.1:5434/test?schema=public