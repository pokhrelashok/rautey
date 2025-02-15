use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct UploadedFile {
    pub filename: String,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
    pub extension: String,
}

impl UploadedFile {
    pub fn save(&self, path: Option<&str>, filename: Option<&str>) -> Result<(), Box<dyn Error>> {
        let base_path = path.unwrap_or("src/storage/uploads");
        let file_name: String = filename
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
                since_the_epoch.to_string()
            })
            .to_string();
        let file_path = format!("{}/{}.{}", base_path, file_name, self.extension);
        let parent_dir = Path::new(&file_path).parent().ok_or("Invalid file path")?;
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path)?;

        let mut writer = BufWriter::new(file);
        writer.write_all(&self.content)?;
        writer.flush()?;

        Ok(())
    }
}
