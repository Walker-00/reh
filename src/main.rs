extern crate ffmpeg_next as ffmpeg;

mod process_file;
mod process_vd;

use base64::engine::general_purpose;
use base64::prelude::*;
use directories_next::BaseDirs;
use process_file::exists_count;
use process_vd::vd_process;
use std::env;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();

    let dir_name = cache_dir.join(PathBuf::from(format!(
        "walker/{}",
        general_purpose::STANDARD_NO_PAD.encode(env::args().nth(1).expect("Can't"))
    )));
    let dir_name: &str = dir_name.to_str().unwrap();

    match exists_count(dir_name) {
        Some(total_imgs) => println!("{total_imgs}"),
        None => vd_process(dir_name).unwrap(),
    }

    Ok(())
}
