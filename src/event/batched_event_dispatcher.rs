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
/// use optimizely::event::BatchedEventDispatcher;
///
/// let dispatcher = BatchedEventDispatcher::new();
/// // TODO: add example of two events being sent
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
