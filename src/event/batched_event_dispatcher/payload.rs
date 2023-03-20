// Imports from crate
use crate::event::Payload;

// Upper limit to number of events in a batch
const DEFAULT_BATCH_THRESHOLD: u16 = 10;

pub(super) struct BatchedPayload {
    counter: u16,
    payload_option: Option<Payload>,
}

impl BatchedPayload {
    pub(super) fn new() -> BatchedPayload {
        let payload_option: Option<Payload> = None;
        let counter = 0;

        BatchedPayload {
            counter,
            payload_option,
        }
    }

    pub(super) fn add_decision(
        &mut self, account_id: String, visitor_id: String, campaign_id: String, experiment_id: String,
        variation_id: String,
    ) {
        log::debug!("Adding decision event to log payload");

        // Add to the existing payload or create a new one
        match self.payload_option.as_mut() {
            None => {
                // Create new payload
                let mut payload = Payload::new(account_id);

                // Add decision
                payload.add_decision(visitor_id, campaign_id, experiment_id, variation_id);

                // Store for next iteration
                self.payload_option = Some(payload);
            }
            Some(payload) => {
                // Add decision
                payload.add_decision(visitor_id, campaign_id, experiment_id, variation_id);
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
                match payload.send() {
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

impl Drop for BatchedPayload {
    fn drop(&mut self) {
        log::debug!("Dropping BatchedPayload");

        // If the BatchedLogPayload is dropped, send one last payload
        self.send()
    }
}
