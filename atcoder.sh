cargo compete new $1
cd $1

mkdir .vscode
touch .vscode/settings.json 
echo '{
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-A", "clippy::needless_return"]
}
' >> .vscode/settings.json

touch run.sh
echo '
p=`pwd`
dir=${p##*/}
p=${p///c/C:}
p=${p//\//\\}
p=${p}\\Cargo.toml
rustup run 1.42.0 cargo run --bin $dir-$1 --manifest-path $p
' >> run.sh

echo '
[features]
local = []

[profile.dev]
opt-level = 3' >> Cargo.toml

code .