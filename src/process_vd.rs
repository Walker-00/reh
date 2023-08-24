use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use ffmpeg_next::{
    format::{input, Pixel},
    frame::Video,
    media::Type,
    software::scaling::{Context, Flags},
};

pub fn vd_process(dir_name: &PathBuf, file: &String) -> Result<(), ffmpeg_next::Error> {
    ffmpeg_next::init().unwrap();
    println!("Processing Video....\nThis Might Take a several minutes.\ndepending on your video's duration and Your Computer.");
    match input(&file) {
        Ok(mut ictx) => {
            let input = ictx
                .streams()
                .best(Type::Video)
                .ok_or(ffmpeg_next::Error::StreamNotFound)?;
            let video_stream_index = input.index();

            let context_decoder =
                ffmpeg_next::codec::context::Context::from_parameters(input.parameters())?;
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
            let mut frame_index: usize = 0;

            let mut receive_and_process_decoded_frames =
                |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
                    let mut decoded = Video::empty();
                    while decoder.receive_frame(&mut decoded).is_ok() {
                        let mut rgb_frame = Video::empty();
                        scaler.run(&decoded, &mut rgb_frame)?;
                        save_frame(&rgb_frame, frame_index, dir_name).unwrap();
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
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}

fn save_frame(frame: &Video, index: usize, dir_name: &PathBuf) -> std::io::Result<()> {
    create_dir_all(dir_name)?;
    let mut file = File::create(format!("{}/frame{}.png", dir_name.to_str().unwrap(), index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))
}
