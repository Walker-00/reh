use std::{
    path::{Path, PathBuf},
    process::exit,
};

mod process_file;
mod process_vd;
mod process_wallpaper;

use base64::{engine::general_purpose, Engine};
use clap::{Parser, ValueEnum};
use directories_next::BaseDirs;
use process_file::exists_count;
use process_vd::vd_process;
use process_wallpaper::set_wallpaper;
use wallpaper::Mode;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ModeConf {
    /// Fit Mode {Default}
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
    #[arg(short, long, help = "Video or animated image's file path")]
    file: String,
    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "sleep time per one frame as millisecond to set frame wallpaper"
    )]
    sleep: u64,
    #[arg(short, long = "loop", help = "loop the wallpaper video, forever?")]
    loopit: String,
    /// Choose Mode to set wallpaper
    #[arg(value_enum, default_value_t = ModeConf::Fit)]
    mode: ModeConf,
}

impl From<ModeConf> for Mode {
    fn from(value: ModeConf) -> Self {
        match value {
            ModeConf::Fit => Mode::Fit,
            ModeConf::Crop => Mode::Crop,
            ModeConf::Span => Mode::Span,
            ModeConf::Tile => Mode::Tile,
            ModeConf::Center => Mode::Center,
            ModeConf::Stretch => Mode::Stretch,
        }
    }
}

fn main() {
    let args = Config::parse();
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();
    let match_modes: Mode = args.mode.into();

    let dir_name_path = cache_dir.join(PathBuf::from(format!(
        "reh/{}",
        general_purpose::STANDARD_NO_PAD.encode(&args.file)
    )));

    if !Path::new(&args.file).exists() {
        println!("Error due to: Video file {} is not exists.", &args.file);
        exit(1);
    }

    match exists_count(&dir_name_path) {
        Some(total_imgs) => set_wallpaper(
            &dir_name_path,
            total_imgs,
            match_modes,
            args.sleep,
            args.loopit,
        )
        .unwrap(),
        None => {
            vd_process(&dir_name_path, &args.file).unwrap();
            match exists_count(&dir_name_path) {
                Some(total_imgs) => set_wallpaper(
                    &dir_name_path,
                    total_imgs,
                    match_modes,
                    args.sleep,
                    args.loopit,
                )
                .unwrap(),
                None => {
                    println!("Something went wrong while processing video!\nReport error to developer.\n:\")");
                    exit(87);
                }
            }
        }
    }
}
