
p=`pwd`
dir=${p##*/}
p=${p///c/C:}
p=${p//\//\\}
p=${p}\\Cargo.toml
rustup run 1.42.0 cargo run --bin $dir-$1 --manifest-path $p

