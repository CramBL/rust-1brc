

[private]
@default:
    just --list

run-seq: build-release-sequential run-release-binary

run-par: build-release-parallel run-release-binary

[private]
run-release-binary: && verify-matching-baseline
    time ./target/release/rust-1brc

[private]
build-release-sequential:
    cargo build --release --no-default-features --features sequential

[private]
build-release-parallel:
    cargo build --release --no-default-features --features parallel

[private]
verify-matching-baseline:
    diff -q baseline.txt out.txt