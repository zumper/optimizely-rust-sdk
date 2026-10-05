# GitHub Actions CI

`build` runs the existing `cargo test` command with Rust 1.76.0 on a light GitHub runner. The container isolates the Rust toolchain without changing application dependencies.

CircleCI's entry configuration is removed. Replace required `ci/circleci:`
checks with the equivalent Actions jobs after this PR's checks pass, then disable
the CircleCI project to prevent historical branches from starting old pipelines.
Do not merge until publishing credentials and private dependency access have been
verified by a real run. GitHub-hosted Macs are the approved macOS exception.
