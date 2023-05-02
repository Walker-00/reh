extern crate ffmpeg_next as ffmpeg;

use base64::engine::general_purpose;
use base64::prelude::*;
use directories_next::BaseDirs;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
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

fn save_file(
    frame: &Video,
    index: usize,
    dir_name: &String,
) -> std::result::Result<(), std::io::Error> {
    create_dir_all(dir_name)?;
    let mut file = File::create(format!("{}/frame{}.png", dir_name, index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    Ok(())
}

fn vd_process(dir_name: &String) -> Result<(), ffmpeg::Error> {
    if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    save_file(&rgb_frame, frame_index, dir_name).unwrap();
                    frame_index += 1;
                }
                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }
    Ok(())
}
