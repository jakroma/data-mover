sudo rm -rf ./dbcTemp
cargo build
./target/debug/data-mover  -f postgresql://user:password@127.0.0.1:5433/test?schema=public -t mongodb://root:example@127.0.0.1:27019/test