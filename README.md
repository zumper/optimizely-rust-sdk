
# Optimizely Feature Experimentation - Rust SDK

An **unofficial** Rust SDK for Optimizely Feature Experimentation.

This SDK is **not** supported by Optimizely!

This SDK only includes a small subset of features compared to supported SDKs. Use at own risk!

## Example

```rust
use optimizely::ClientBuilder;

// Initialize Optimizely client using local datafile
let file_path = "examples/datafiles/sandbox.json";
let optimizely_client = ClientBuilder::new()
    .with_local_datafile(file_path).unwrap()
    .build().unwrap();

// Create user context for current user
let user_id = "123abc789xyz";
let user_context = optimizely_client.create_user_context(user_id);

// Get decision for the Buy Button feature flag
let feature_flag = "buy_button";
let decision = user_context.decide(feature_flag);
```

## Included features

A list of the features that are currently included:

- [x] Initialize client from local datafile
- [x] Initialize client from SDK key
- [ ] Periodically poll latest datafile
- [x] Event dispatcher (synchronous)
- [x] Event dispatcher (batched)
- [ ] Logger
- [ ] Notification listeners
- [X] Decide option (DisableDecisionEvent)
- [ ] Decide options (others)
- [X] Creating an user context
- [X] Decide method consistent with other SDKs
- [ ] Evaluating audience conditions
- [ ] Variation variables
- [ ] Forced decision methods
- [ ] Mutual exclusion groups
