cargo compete new $1
cp -r .vscode/ $1/.vscode/
cd $1
echo '
[profile.dev]
opt-level = 3' >> Cargo.toml
code .