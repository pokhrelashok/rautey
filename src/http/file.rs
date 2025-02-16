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

pub struct Uploader<'a> {
    pub file: &'a UploadedFile,
    pub filename: Option<String>,
    pub path: Option<String>,
}

impl<'a> Uploader<'a> {
    pub fn new(file: &'a UploadedFile) -> Uploader<'a> {
        Uploader {
            file: file,
            filename: None,
            path: None,
        }
    }

    pub fn with_filename<T: Into<String>>(mut self, filename: T) -> Uploader<'a> {
        self.filename = Some(filename.into());
        self
    }
    pub fn with_path<T: Into<String>>(mut self, path: T) -> Uploader<'a> {
        self.path = Some(path.into());
        self
    }
    pub fn upload(&self) -> Result<(), Box<dyn Error>> {
        let default_path = "src/storage/uploads".to_string();
        let base_path = self.path.as_ref().unwrap_or(&default_path);
        let file_name: String = self
            .filename
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
                since_the_epoch.to_string()
            })
            .to_string();
        let file_path = format!("{}/{}.{}", base_path, file_name, self.file.extension);
        upload_file(file_path, &self.file.content)
    }
}

fn upload_file(file_path: String, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let parent_dir = Path::new(&file_path).parent().ok_or("Invalid file path")?;
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_path)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(content)?;
    writer.flush()?;

    Ok(())
}

impl UploadedFile {
    pub fn uploader(&self) -> Uploader {
        return Uploader::new(self);
    }
}
