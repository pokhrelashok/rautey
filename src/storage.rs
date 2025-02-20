use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

pub struct Storage {}
impl Storage {
    pub fn upload<T: AsRef<str>>(content: &[u8], file_path: T) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.as_ref();
        let parent_dir = Path::new(file_path).parent().ok_or("Invalid file path")?;
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)?;

        let mut writer = BufWriter::new(file);
        writer.write_all(content)?;
        writer.flush()?;

        Ok(())
    }
}
