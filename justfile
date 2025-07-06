set dotenv-load := true

alias c := clippy
alias d := doc
alias r := run

clippy:
    cargo clippy --features all,blocking --fix --allow-dirty

doc:
    cargo doc --features all,blocking
    miniserve target/doc

run name="main":
    cargo r --example {{name}} --features all