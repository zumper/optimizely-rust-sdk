// External imports
use std::sync::mpsc;
use std::thread;

// Imports from super
use crate::event::{Event, EventDispatcher};

// Relative imports of sub modules
use payload::BatchedPayload;

mod payload;

/// Implementation of the EventDisptacher trait that collects multiple events before sending them
///
/// ```
/// use optimizely::event::{BatchedEventDispatcher, Event, EventDispatcher};
///
/// // Create some example IDs
/// let account_id = "21537940595";
/// let user_ids = vec!["user0", "user1", "user2"];
/// let campaign_id = "9300000133039";
/// let experiment_id = "9300000169122";
/// let variation_ids = vec!["87757", "87757", "87755"];
///
/// // Create events from above IDs
/// let events = user_ids.iter()
///     .zip(variation_ids.iter())
///     .map(|(user_id, variation_id)| {
///         Event::decision(account_id, user_id, campaign_id, experiment_id, variation_id)
///     });
///
/// // Create batched event disptacher
/// let dispatcher = BatchedEventDispatcher::new();
///
/// // Send all events
/// for event in events {
///     dispatcher.send_event(event);
/// }
///
/// // Note that only one request will be sent to the Event API
/// ```
///
/// Inspiration from [Spawn threads and join in destructor](https://users.rust-lang.org/t/spawn-threads-and-join-in-destructor/1613/9)
pub struct BatchedEventDispatcher {
    thread_handle: Option<thread::JoinHandle<()>>,
    transmitter: Option<mpsc::Sender<Event>>,
}

impl BatchedEventDispatcher {
    /// Constructor for a new batched event dispatcher
    pub fn new() -> BatchedEventDispatcher {
        let (transmitter, receiver) = mpsc::channel();

        let thread_handle = thread::spawn(move || {
            let mut batched_payload = BatchedPayload::new();

            // Keep receiving new message from the main thread
            for event in receiver.iter() {
                match event {
                    Event::Decision {
                        account_id,
                        user_id,
                        campaign_id,
                        experiment_id,
                        variation_id,
                    } => {
                        batched_payload.add_decision(account_id, user_id, campaign_id, experiment_id, variation_id);
                    }
                    _ => {
                        // TODO
                    }
                }
            }
        });

        BatchedEventDispatcher {
            thread_handle: Some(thread_handle),
            transmitter: Some(transmitter),
        }
    }
}

impl Drop for BatchedEventDispatcher {
    fn drop(&mut self) {
        // Take the transmitter_decision and replace it with None
        match self.transmitter.take() {
            Some(tx) => {
                // Drop the transmitter first, so the receiver in the thread will eventually stop
                drop(tx);
            }
            None => {
                // No transmitter found
            }
        }

        // Take the thread_handle and replace it with None
        match self.thread_handle.take() {
            Some(handle) => {
                // Wait until the thread has send the last batch
                let result = handle.join();
                // Ignore result
                drop(result);
            }
            None => {
                // No thread found
            }
        }
    }
}

impl EventDispatcher for BatchedEventDispatcher {
    fn send_event(&self, event: Event) {
        // Send event to thread
        match &self.transmitter {
            Some(tx) => match tx.send(event) {
                Ok(_) => {}
                Err(_) => {}
            },
            None => {}
        }
    }
}
