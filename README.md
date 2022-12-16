
# Optimizely Feature Experimentation - Rust SDK

An **unofficial** Rust SDK for Optimizely Feature Experimentation.

This SDK is **not** supported by Optimizely!

This SDK only includes a small subset of features compared to supported SDKs. Use at own risk!

## Included features

A list of the features that are currently included:

- [x] Initialize client from local datafile
- [x] Initialize client from SDK key
- [ ] Periodically poll latest datafile
- [ ] Event dispatcher
- [ ] Logger
- [ ] Notification listeners
- [ ] Decide Options
- [X] Creating an user context
- [X] Decide method consistent with other SDKs
- [ ] Evaluating audience conditions
- [ ] Variation variables
- [ ] Forced decision methods

## Performance

Rust offers very high performance. Here is a comparison of this **unofficial** Rust SDK to the official SDKs in other languages.

| Language   | Duration (1M) | Decisions / second |
| :--------- | ------------: | -----------------: |
| Rust       |         0.17s |               5.8M |
| Go         |         2.80s |               357k |
| JavaScript |         3.18s |               314k |
| Python     |        22.57s |                44k |

[Source](/examples/performance-test/)