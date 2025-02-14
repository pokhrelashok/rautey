use std::collections::HashMap;

use urlencoding::decode;

use super::file::UploadedFile;

pub fn parse_url_encoded(content: &str) -> HashMap<String, String> {
    let content = decode(content).expect("Invalid data");
    let key_val_pairs = content.split("&");
    let mut result = HashMap::new();
    for key_val_pair in key_val_pairs {
        let mut key_val = key_val_pair.split("=");
        let k = key_val.nth(0).unwrap_or_default().to_string();
        let v = key_val.nth(0).unwrap_or_default().to_string();
        if !k.is_empty() && !v.is_empty() {
            result.insert(k, v);
        }
    }

    return result;
}

pub fn parse_multipart_form_data(
    form_data: &str,
    boundary: &str,
) -> (HashMap<String, String>, HashMap<String, UploadedFile>) {
    let mut fields = HashMap::new();
    let mut files = HashMap::new();
    let boundary_marker = format!("--{}", boundary);
    let end_marker = format!("--{}--", boundary);

    for part in form_data.split(&boundary_marker).skip(1) {
        let part = part.trim();
        if part == end_marker {
            break;
        }

        if let Some((headers, body)) = part.split_once("\r\n\r\n") {
            let body = body.trim_end_matches("\r\n").as_bytes().to_vec(); // Handle as bytes for files

            let mut field_name = None;
            let mut filename = None;
            let mut content_type = None;

            for header in headers.lines() {
                if header.starts_with("Content-Disposition: form-data;") {
                    if let Some(name_part) = header.split("name=").nth(1) {
                        field_name = Some(name_part.trim_matches(&['"', '\''][..]).to_string());
                    }
                    if let Some(filename_part) = header.split("filename=").nth(1) {
                        filename = Some(filename_part.trim_matches(&['"', '\''][..]).to_string());
                    }
                } else if header.starts_with("Content-Type:") {
                    content_type = Some(header.split_once(": ").unwrap().1.to_string());
                }
            }

            if let Some(name) = field_name {
                if let Some(file_name) = filename {
                    files.insert(
                        name,
                        UploadedFile {
                            extension: file_name.split(".").last().unwrap_or_default().to_owned(),
                            filename: file_name,
                            content_type,
                            size: body.len() as u64,
                            content: body,
                        },
                    );
                } else {
                    fields.insert(name, String::from_utf8_lossy(&body).to_string());
                }
            }
        }
    }

    (fields, files)
}
