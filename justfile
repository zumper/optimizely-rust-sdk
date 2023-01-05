docs:
  cargo doc

test:
  cargo test -- --nocapture
  just docs

fmt:
  rustfmt src/**/*.rs tests/*.rs
