use std::{env::var, error::Error, fs::OpenOptions, io::Write, path::Path};

use chrono::{Datelike, Timelike};
pub fn log<T: AsRef<str>>(info: T) -> Result<(), Box<dyn Error>> {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let (_, hour) = current_date.hour12();
    let min = current_date.minute();
    let seconds = current_date.second();

    let final_string = format!("{}:{}:{} {}\n", hour, min, seconds, info.as_ref());
    let file_name = format!(
        "{}/{}-{}-{}",
        var("APP_LOGS_DIR").unwrap_or("logs".to_string()),
        year,
        month,
        day
    );
    let file_path = Path::new(&file_name);

    let parent_dir = Path::new(file_path).parent().ok_or("Invalid file path")?;
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    file.write_all(final_string.as_bytes())?;
    Ok(())
}
