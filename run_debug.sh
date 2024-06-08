cargo build
./target/debug/data-mover  -f postgresql://admin:admin@127.0.0.1:5432/MandC?schema=public -t postgresql://admin:admin@127.0.0.1:5432/MandC?schema=public