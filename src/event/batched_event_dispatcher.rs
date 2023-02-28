// External imports
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

// Imports from crate
use crate::datafile::{Experiment, Variation};
use crate::event::{EventDispatcher, LogPayload};
use crate::UserContext;

// https://users.rust-lang.org/t/spawn-threads-and-join-in-destructor/1613/9
pub struct BatchedEventDispatcher {
    thread_handle: Option<thread::JoinHandle<()>>, 
    transmitter_decision: Option<mpsc::Sender<(String, String, String, String, String)>>,
}

impl BatchedEventDispatcher {
    pub fn new() -> BatchedEventDispatcher {
        let (transmitter_decision, receiver_decision) = mpsc::channel();

        let thread_handle = thread::spawn(move || {
            // TODO: write as struct
            let mut payload_option: Option<LogPayload> = None;
            
            for (account_id, visitor_id, campaign_id, experiment_id, variation_id) in receiver_decision.iter() {
                match payload_option.as_mut() {
                    None => {
                        // Create new payload
                        let mut payload = LogPayload::new(account_id);

                        // Add decision
                        payload.add_decision(visitor_id, campaign_id, experiment_id, variation_id);

                        // Store for next iteration
                        payload_option = Some(payload);
                    },
                    Some(payload) => {
                         // Add decision
                        payload.add_decision(visitor_id, campaign_id, experiment_id, variation_id);
                    }
                };
            }

            // Send payload to endpoint
            match payload_option {
                Some(payload) => {
                    payload.send();
                },
                None => {
                    // Nothing to send
                }
            }
        });

        BatchedEventDispatcher {
            thread_handle: Some(thread_handle),
            transmitter_decision: Some(transmitter_decision),
        }
    }
}

impl Drop for BatchedEventDispatcher {
    fn drop(&mut self) {
        // Take the transmitter_decision and replace it with None
        match self.transmitter_decision.take() {
            Some(tx) => {
                // Drop the transmitter first, so the receiver in the thread will eventually stop
                drop(tx);
            },
            None => {
                // No transmitter found
            }
        }

        // Take the thread_handle and replace it with None
        match self.thread_handle.take(){
            Some(handle) => {
                // Wait until the thread has send the last batch
                let result = handle.join();
                // Ignore result
                drop(result);
            },
            None => {
                // No thread found
            }
        }
    }
}

impl EventDispatcher for BatchedEventDispatcher {
    fn send_decision(&self, user_context: &UserContext, experiment: &Experiment, variation: Rc<Variation>) {
        let tuple = (
            user_context.client().account_id().to_owned(),
            user_context.user_id().to_owned(),
            experiment.campaign_id().to_owned(),
            experiment.id().to_owned(),
            variation.id().to_owned(),
        );

        // Send data to thread
        match &self.transmitter_decision {
            Some(tx) => {
                match tx.send(tuple) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            },
            None => {},
        }

    }
}
