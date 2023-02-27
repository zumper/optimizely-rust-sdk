use optimizely::{ClientBuilder, Datafile, SimpleEventDispatcher};
use std::error::Error;

const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

fn main() -> Result<(), Box<dyn Error>> {
    let datafile = Datafile::build_from_sdk_key(SDK_KEY)?;
    let event_dispatcher = SimpleEventDispatcher::new();

    let client = ClientBuilder::new()
        .with_datafile(datafile)
        .with_event_dispatcher(event_dispatcher)
        .build()?;
    
    let flag_key = "buy_button";
    let decide_options = vec![];

    for i in 0..10 {
        let user_id = format!("user{}", i);
        let user_context = client.create_user_context(&user_id);
        let _decision = user_context.decide(flag_key, &decide_options);
    }

    Ok(())
}
