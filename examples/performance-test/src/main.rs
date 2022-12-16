use optimizely::client::Client;
use std::error::Error;

const FILE_PATH: &str = "../datafile.json";

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::build_from_file(FILE_PATH)?;
    let flag_key = "buy_button";
    let decide_options = Vec::new();

    for i in 0..1_000_000 {
        let user_id = format!("user{}", i);
        let user_context = client.create_user_context(&user_id);
        let _decision = user_context.decide(flag_key, &decide_options);
    }

    Ok(())
}
