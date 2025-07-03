set dotenv-load := true

alias c := clippy
alias r := run

clippy:
    cargo clippy --features all --fix --allow-dirty

run name="main":
    cargo r --example {{name}} --features all