
p=`pwd`
dir=${p##*/}
p=${p///c/C:}
p=${p//\//\\}
p=${p}\\Cargo.toml
rustup run 1.42.0 cargo run --bin $dir-$1 --manifest-path C:\\Users\\06106\\src\\git\\atcoder\\code-festival-2015-morning-easy\\Cargo.toml

