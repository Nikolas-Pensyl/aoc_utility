use ureq;
use std::env;
use std::fs;

pub fn get_input(year: usize, day: usize) -> String {
    let session: String = get_session();

    let uri = format!("https://adventofcode.com/{year}/day/{day}/input");

    let response = ureq::get(&uri)
    .set("cookie", &session)
    .call()
    .expect("Problem contacting the server. Double check your session value and the day and year are valid. \n\n: {error:?}")
    .into_string()
    .expect("Failed to convert Response to String.");

    response
}


pub fn submit_ans(year: usize, day: usize, part: &str, answer: &str) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let params = &[("level", part), ("answer", answer)];
    let response = ureq::post(&url).set("cookie", &get_session()).send_form(params)
    .expect("Failed to submit answer.")
    .into_string()
    .expect("Failed to turn response into string.");

    response
}

fn get_session() -> String {
    let mut path: std::path::PathBuf = env::current_dir().expect("Problem occured getting the current directory.");
    path = path.join(".session");

    let session: String = fs::read_to_string(path).expect("Failed to read to file. Check to make sure your .session file exists");

    session
}