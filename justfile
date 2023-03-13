docs:
  cargo doc -p optimizely --no-deps

release-test:
  cargo test --release

quick-test:
  cargo test --lib -- --nocapture

test:
  just release-test
  just docs

fmt:
  rustfmt src/**/*.rs tests/*.rs
