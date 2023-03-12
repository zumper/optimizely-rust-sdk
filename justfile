docs:
  cargo doc -p optimizely --no-deps

test:
  cargo test -- --nocapture
  just docs

fmt:
  rustfmt src/**/*.rs tests/*.rs
