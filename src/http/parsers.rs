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

fn split_once_bytes<'a>(slice: &'a [u8], delimiter: &[u8]) -> Option<(&'a [u8], &'a [u8])> {
    let delimiter_len = delimiter.len();
    if delimiter_len == 0 || slice.len() < delimiter_len {
        return None;
    }

    for i in 0..=slice.len() - delimiter_len {
        if &slice[i..i + delimiter_len] == delimiter {
            return Some((&slice[..i], &slice[i + delimiter_len..]));
        }
    }

    None
}

fn split_bytes<'a>(slice: &'a [u8], delimiter: &[u8]) -> Vec<&'a [u8]> {
    let mut slice = slice;
    let delimiter_len = delimiter.len();
    if delimiter_len == 0 {
        return vec![slice];
    }

    let mut result = Vec::new();
    let mut start = 0;

    while start + delimiter_len <= slice.len() {
        // Check if the current window matches the delimiter
        if &slice[start..start + delimiter_len] == delimiter {
            // Push the part before the delimiter
            result.push(&slice[..start]);
            // Move the slice forward, skipping the delimiter
            slice = &slice[start + delimiter_len..];
            start = 0;
        } else {
            start += 1;
        }
    }

    // Push the remaining part of the slice
    result.push(slice);
    result
}

pub fn parse_multipart_form_data(
    form_data: &[u8],
    boundary: &str,
) -> (HashMap<String, String>, HashMap<String, UploadedFile>) {
    let mut fields = HashMap::new();
    let mut files = HashMap::new();
    let boundary_marker = format!("--{}", boundary);
    let end_marker = format!("--{}--", boundary);

    // Convert boundary and end markers to byte slices
    let boundary_marker_bytes = boundary_marker.as_bytes();
    let end_marker_bytes = end_marker.as_bytes();

    // Split the form data by the boundary marker
    let parts = split_bytes(form_data, boundary_marker_bytes);
    for part in parts.iter().skip(1) {
        if *part == end_marker_bytes {
            break;
        }

        if let Some((headers, body)) = split_once_bytes(part, b"\r\n\r\n") {
            let mut body = body.to_vec(); // Handle as bytes for files
            body.truncate(body.len().saturating_sub(2));
            let mut field_name = None;
            let mut filename = None;
            let mut content_type = None;

            // Convert headers to a string for parsing
            let headers_str = String::from_utf8_lossy(headers);
            for header in headers_str.lines() {
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
                            content: body,
                        },
                    );
                } else {
                    // Convert body to string only if it's not a file
                    fields.insert(name, String::from_utf8_lossy(&body).to_string());
                }
            }
        }
    }

    (fields, files)
}
