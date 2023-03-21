// Imports from crate
use super::super::{request::Payload, Event, EventApiClient};

// Upper limit to number of events in a batch
const DEFAULT_BATCH_THRESHOLD: u16 = 10;

pub(super) struct BatchedPayload<'a> {
    counter: u16,
    payload_option: Option<Payload<'a>>,
}

impl BatchedPayload<'_> {
    pub(super) fn new() -> BatchedPayload<'static> {
        let payload_option: Option<Payload> = None;
        let counter = 0;

        BatchedPayload {
            counter,
            payload_option,
        }
    }

    pub(super) fn add_event(&mut self, event: Event) {
        // Add to the existing payload or create a new one
        match self.payload_option.as_mut() {
            None => {
                // Create new payload
                let mut payload = Payload::new(event.account_id());

                // Add decision
                payload.add_event(event);

                // Store for next iteration
                self.payload_option = Some(payload);
            }
            Some(payload) => {
                // Add decision
                payload.add_event(event);
            }
        };

        // Increment counter
        self.counter += 1;

        if self.counter >= DEFAULT_BATCH_THRESHOLD {
            log::debug!("Reached DEFAULT_BATCH_THRESHOLD");
            self.send();
        }
    }

    fn send(&mut self) {
        // Take ownership of payload and leave behind None (for next iteration)
        match self.payload_option.take() {
            Some(payload) => {
                // Sending payload
                log::debug!("Sending log payload to Event API");

                // Send payload to endpoint
                match EventApiClient::send(payload) {
                    Ok(_) => {
                        log::info!("Successfull request to Event API");
                    }
                    Err(report) => {
                        log::error!("Failed request to Event API");
                        log::error!("\n{report:?}");
                    }
                }

                // Reset counter
                self.counter = 0;
            }
            None => {
                // Nothing to send
                log::debug!("No log payload to send");
            }
        }
    }
}

impl Drop for BatchedPayload<'_> {
    fn drop(&mut self) {
        log::debug!("Dropping BatchedPayload");

        // If the BatchedLogPayload is dropped, send one last payload
        self.send()
    }
}
