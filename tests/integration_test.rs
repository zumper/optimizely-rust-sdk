use optimizely::Optimizely;
use std::fs::File;
use std::io::Read;

const FILE_PATH: &str = "tests/datafile.example.json";

#[test]
fn client_initialization() {
    // Read datafile from local path
    let mut datafile = String::new();
    File::open(FILE_PATH)
        .expect("should be able to open file")
        .read_to_string(&mut datafile)
        .expect("should be able to read to string");

    // Example datafile is valid
    let result = Optimizely::build(&datafile);
    matches!(result, Ok(_));

    // Can unwrap now, since matches OK(_)
    let client = result.unwrap();

    // Check property on client
    assert!(client.initialized);
}

#[test]
fn empty_datafile() {
    // Empty datafile is invalid
    let result = Optimizely::build(r"");
    matches!(result, Err(_));
}
