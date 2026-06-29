default:
    @just --list

test:
    cargo test --locked --all-features --all-targets
    cargo test --locked --all-features --doc
    cargo test --locked --no-default-features --all-targets
    cargo test --locked --no-default-features --doc

lint:
    cargo +nightly fmt --all -- --check
    cargo clippy --all-targets -- -D warnings

check:
    cargo +nightly docs-rs
    cargo hack --feature-powerset check
    cargo semver-checks
