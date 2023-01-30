
# Optimizely Feature Experimentation - Rust SDK

An **unofficial** Rust SDK for Optimizely Feature Experimentation.

This SDK is **not** supported by Optimizely!

This SDK only includes a small subset of features compared to supported SDKs. Use at own risk!

## Included features

A list of the features that are currently included:

- [x] Initialize client from local datafile
- [x] Initialize client from SDK key
- [ ] Periodically poll latest datafile
- [x] Event dispatcher (synchronous)
- [ ] Event dispatcher (batched)
- [ ] Logger
- [ ] Notification listeners
- [X] Decide option (DisableDecisionEvent)
- [ ] Decide options (others)
- [X] Creating an user context
- [X] Decide method consistent with other SDKs
- [ ] Evaluating audience conditions
- [ ] Variation variables
- [ ] Forced decision methods

## Performance

Rust offers very high performance. Here is a comparison of this **unofficial** Rust SDK to the official SDKs in other languages.

### Sandbox account

These are the results for my personal sandbox account containing 7 feature flags and 4 experiments.

| Language   | Duration (1M) | Average latency / decision | Decisions / second |
| :--------- | ------------: | -------------------------: | -----------------: |
| Rust       |        0.17 s |                    0.17 μs |               5.8M |
| Go         |        2.80 s |                    2.80 μs |               357k |
| JavaScript |        3.18 s |                    3.18 μs |               314k |
| Python     |       22.57 s |                   22.57 μs |                44k |

### Real customer account

These are the results for a real customer account. Let's call them `Hello, World!`. Their account contains 741 feature flags and 349 experiments.

| Language   | Duration (1M) | Average latency / decision | Decisions / second |
| :--------- | ------------: | -------------------------: | -----------------: |
| Rust       |        0.17 s |                    0.17 μs |               5.8M |
| Go         |       13.52 s |                   13.52 μs |                74k |
| JavaScript |        9.71 s |                    9.71 μs |               103k |
| Python     |       67.87 s |                   67.87 μs |                15k |


[Source](/examples/performance-test/)