use std::{fs::read_dir, path::PathBuf};

use mime::IMAGE;
use mime_guess::from_path;

pub fn exists_count(path: &PathBuf) -> Option<usize> {
    if path.exists() {
        let mut count: usize = 0;
        for entry in read_dir(path).unwrap() {
            let file_path = entry.unwrap().path();
            let is_image = from_path(file_path).first();

            match is_image {
                Some(mime_type) => {
                    if mime_type.type_() == IMAGE {
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
