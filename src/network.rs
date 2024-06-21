use std::fs::read_to_string;
use ureq::{get};
use std::env;
use std::fs;

pub fn get_input(year: usize, day: usize) -> String {
    let session: String = get_session();

    let response = ureq::get("https://adventofcode.com/{year}/{day}/2/input")
    .set("cookie", &session)
    .call()
    .expect("Problem contacting the server. Double check your session value and the day and year are valid. \n\n: {error:?}")
    .into_string()
    .expect("Failed to convert Response to String.");

    response
}


fn get_session() -> String {
    let mut path: std::path::PathBuf = env::current_dir().expect("Problem occured getting the current directory.");
    path = path.join(".session");

    let session: String = fs::read_to_string(path).expect("Failed to read to file. Check to make sure your .session file exists");

    session
}