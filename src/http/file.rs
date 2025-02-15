use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Debug)]
pub struct UploadedFile {
    pub filename: String,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
    pub extension: String,
}

impl UploadedFile {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file_path = format!("src/storage/uploads/{}", self.filename);
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
