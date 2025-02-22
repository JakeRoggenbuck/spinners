use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;
use strum::IntoEnumIterator;

fn main() {
    // loop through each spinner and display them during 2 seconds
    for spinner in Spinners::iter() {
        print!("{}", ansi_escapes::EraseLines(10000));
        let sp = Spinner::new(&spinner, format!("{:?}", spinner));
        sleep(Duration::from_secs(2));
        sp.stop();
    }
}
