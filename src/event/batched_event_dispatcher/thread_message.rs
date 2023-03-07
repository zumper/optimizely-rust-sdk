// When sending a message to the thread, it can either be a decision event or conversion event
#[allow(dead_code)]
pub(super) enum ThreadMessage {
    Decision(String, String, String, String, String),
    Conversion(),
}