use std::{fs::read_dir, path::Path};

use mime_guess::from_path;

pub fn exists_count(path: &str) -> Option<usize> {
    if Path::new(path).exists() {
        let mut count: usize = 0;
        for entry in read_dir(path).unwrap() {
            let file_path = entry.unwrap().path();
            let mime_read = from_path(file_path.to_str().unwrap());
            let is_image = mime_read.first();
            match is_image {
                Some(mime_type) => {
                    if mime_type.type_() == mime::IMAGE {
                        count += 1;
                    }
                }
                None => return None,
            }
        }
        Some(count)
    } else {
        None
    }
}
