use std::time;

use rand::distr::{Alphanumeric, SampleString};

pub fn uuid() -> String {
    let seconds = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    return format!("{}{}", seconds, string);
}
