#[derive(Debug)]
pub struct UploadedFile {
    pub filename: String,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
    pub extension: String,
    pub size: u64,
}
