extern crate ffmpeg_next as ffmpeg;

mod process_file;
mod process_vd;
mod process_wallpaper;

use base64::engine::general_purpose;
use base64::prelude::*;
use clap::{Parser, ValueEnum};
use directories_next::BaseDirs;
use process_file::exists_count;
use process_vd::vd_process;
use process_wallpaper::set_wallpaper;
use std::path::PathBuf;
use wallpaper::Mode;

#[derive(ValueEnum, Debug, Clone, Copy, Default)]
pub enum ModeConf {
    #[default]
    /// Fit Mode [Default]
    Fit,
    /// Crop Mode
    Crop,
    /// Span Mode
    Span,
    /// Center Mode
    Center,
    /// Tile Mode
    Tile,
    /// Stretch Mode
    Stretch,
}

#[derive(Parser, Debug)]
#[command(
    author = "Linus Walker",
    version = "0.1.0",
    long_about = "Video to live wallpaper for x86_64 linux"
)]
pub struct Config {
    #[arg(short, long, help = "Video or gif file path")]
    file: String,
    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "sleep time per one frame as millisecond to set frame wallpaper"
    )]
    sleep: u64,
    #[arg(short, long = "loop", help = "loop the wallpaper video")]
    loopit: bool,
    /// Choose Mode to set wallpaper
    #[arg(value_enum)]
    mode: ModeConf,
}

fn main() -> std::io::Result<()> {
    let args = Config::parse();
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();
    let match_modes = match args.mode {
        ModeConf::Fit => Mode::Fit,
        ModeConf::Crop => Mode::Crop,
        ModeConf::Span => Mode::Span,
        ModeConf::Tile => Mode::Tile,
        ModeConf::Center => Mode::Center,
        ModeConf::Stretch => Mode::Stretch,
    };

    let dir_name_path = cache_dir.join(PathBuf::from(format!(
        "reh/{}",
        general_purpose::STANDARD_NO_PAD.encode(args.file)
    )));
    let dir_name: &str = dir_name_path.to_str().unwrap();

    match exists_count(dir_name) {
        Some(total_imgs) => set_wallpaper(
            &dir_name_path,
            total_imgs,
            match_modes,
            args.sleep,
            args.loopit,
        )
        .unwrap(),
        None => {
            vd_process(dir_name).unwrap();
            match exists_count(dir_name) {
                Some(total_imgs) => set_wallpaper(
                    &dir_name_path,
                    total_imgs,
                    match_modes,
                    args.duration,
                    args.loopit,
                )
                .unwrap(),
                None => {
                    print!("Someting went wrong!\nError code: 87");
                    std::process::exit(87);
                }
            }
        }
    };

    Ok(())
}
