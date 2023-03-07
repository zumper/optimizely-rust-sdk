// External imports
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

// Imports from crate
use crate::datafile::{Experiment, Variation};
use crate::event::EventDispatcher;
use crate::UserContext;

// Relative imports of sub modules
use payload::BatchedPayload;
use thread_message::ThreadMessage;

mod payload;
mod thread_message;

// https://users.rust-lang.org/t/spawn-threads-and-join-in-destructor/1613/9
pub struct BatchedEventDispatcher {
    thread_handle: Option<thread::JoinHandle<()>>,
    transmitter: Option<mpsc::Sender<ThreadMessage>>,
}

impl BatchedEventDispatcher {
    pub fn new() -> BatchedEventDispatcher {
        let (transmitter, receiver) = mpsc::channel();

        let thread_handle = thread::spawn(move || {
            let mut batched_payload = BatchedPayload::new();

            // Keep receiving new message from the main thread
            for message in receiver.iter() {
                match message {
                    ThreadMessage::Decision(account_id, visitor_id, campaign_id, experiment_id, variation_id) => {
                        batched_payload.add_decision(account_id, visitor_id, campaign_id, experiment_id, variation_id);
                    }
                    ThreadMessage::Conversion() => {
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
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>) {
        // Build thread message by cloning all the Strings
        let message = ThreadMessage::Decision(
            user_context.client().account_id().to_owned(),
            user_context.user_id().to_owned(),
            experiment.campaign_id().to_owned(),
            experiment.id().to_owned(),
            variation.id().to_owned(),
        );

        // Send message to thread
        match &self.transmitter {
            Some(tx) => match tx.send(message) {
                Ok(_) => {}
                Err(_) => {}
            },
            None => {}
        }
    }
}
