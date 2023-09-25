use optimizely::{event_api::BatchedEventDispatcher, Client};
use rand::random;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use uuid::Uuid;

// Production SDK key
const SDK_KEY: &str = "K2rKjU83y2YZcZWnvR35A";

// Flag for which to generate data
const FLAG_KEY: &str = "navbar";

// Event for which to generate conversions
const EVENT_KEY: &str = "purchase";

/// Whether a random event does or doesn't happen
fn random_event_does_happen(chance: f32) -> bool {
    random::<f32>() < chance
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initiate client using SDK key and batched event dispatcher
    let client = Client::from_sdk_key(SDK_KEY)?
        .with_event_dispatcher(BatchedEventDispatcher::default())
        .initialize();

    // Not super accurate interval, but fine for this example
    let interval = Duration::from_millis(166);
    let mut next_iteration = Instant::now();
    loop {
        // There are 10 users in a batch, so it makes sense to sleep between each 10
        for _ in 0..10 {
            // Generate a random user
            let user_id = Uuid::new_v4().as_hyphenated().to_string();
            let user_context = client.create_user_context(&user_id);

            // Get the Optimizely decision for that user
            let decision = user_context.decide(FLAG_KEY);

            // Different conversion rate depending on variation
            let conversion_rate = match decision.variation_key() {
                "complete" => 0.116,
                "simplified" => 0.125,
                "hidden" => 0.117,
                _ => 0.0,
            };

            // Random chance that user makes a purchase
            if random_event_does_happen(conversion_rate) {
                user_context.track_event(EVENT_KEY);
            }
        }

        // Wait a bit until next batch of users
        next_iteration += interval;
        sleep(next_iteration - Instant::now());
    }
}
