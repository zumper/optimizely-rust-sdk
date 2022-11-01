use std::error::Error;

#[derive(Debug)]
pub struct Optimizely {
    pub initialized: bool,
}

impl Optimizely {
    pub fn build(datafile: &str) -> Result<Optimizely, Box<dyn Error>> {
        let parsed = json::parse(datafile)?;

        println!("{:?}", parsed);

        Ok(Optimizely { initialized: true })
    }
}
