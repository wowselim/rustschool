mkdir -p target
rustc $1 --out-dir target
./target/`basename $1 .rs`
