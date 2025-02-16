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
        if &slice[start..start + delimiter_len] == delimiter {
            result.push(&slice[..start]);
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

    let boundary_marker_bytes = boundary_marker.as_bytes();
    let end_marker_bytes = end_marker.as_bytes();

    let parts = split_bytes(form_data, boundary_marker_bytes);
    for part in parts.iter().skip(1) {
        if *part == end_marker_bytes {
            break;
        }

        if let Some((headers, body)) = split_once_bytes(part, b"\r\n\r\n") {
            let mut body = body.to_vec();
            body.truncate(body.len().saturating_sub(2));
            let mut field_name = None;
            let mut filename = None;
            let mut content_type = None;

            let headers_str = String::from_utf8_lossy(headers);
            for header in headers_str.lines() {
                for mut each_header in header.split("; ") {
                    each_header = each_header.trim_matches(&['"', '\''][..]);
                    if let Some(filename_part) = each_header.split("filename=").nth(1) {
                        filename = Some(filename_part.trim_matches(&['"', '\''][..]).to_string());
                        continue;
                    }
                    if let Some(name_part) = each_header.split("name=").nth(1) {
                        field_name = Some(name_part.trim_matches(&['"', '\''][..]).to_string());
                        continue;
                    }
                    if let Some(content_type_part) = each_header.split("filename=").nth(1) {
                        content_type =
                            Some(content_type_part.split_once(": ").unwrap().1.to_string());
                        continue;
                    }
                }
            }

            if let Some(name) = field_name {
                if let Some(file_name) = filename {
                    if body.len() > 0 {
                        files.insert(
                            name,
                            UploadedFile {
                                size: body.len() as u64,
                                extension: file_name
                                    .split(".")
                                    .last()
                                    .unwrap_or_default()
                                    .to_owned(),
                                filename: file_name,
                                content_type,
                                content: body,
                            },
                        );
                    }
                } else {
                    fields.insert(name, String::from_utf8_lossy(&body).to_string());
                }
            }
        }
    }

    (fields, files)
}
