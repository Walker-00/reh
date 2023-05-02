extern crate ffmpeg_next as ffmpeg;

mod process_vd;

use base64::engine::general_purpose;
use base64::prelude::*;
use directories_next::BaseDirs;
use process_vd::vd_process;
use std::path::Path;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    ffmpeg::init().unwrap();

    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir().to_str().unwrap().to_string();

    let dir_name = &format!(
        "{}/walker/{}",
        cache_dir,
        general_purpose::STANDARD_NO_PAD.encode(env::args().nth(1).expect("Can't"))
    );

    if Path::new(dir_name).exists() {
        println!("exists");
        let mut count = 0;
        for entry in fs::read_dir(dir_name)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                count += 1;
            }
        }
        println!("{count}");
        std::process::exit(1);
    } else {
        vd_process(dir_name)?;
    }

    Ok(())
}
