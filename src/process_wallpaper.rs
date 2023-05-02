use std::{error::Error, path::PathBuf, thread::sleep, time::Duration};
use wallpaper::{set_from_path, set_mode, Mode};

pub fn set_wallpaper(
    path: &PathBuf,
    total_imgs: usize,
    mode: Mode,
    duration: u64,
    loopit: bool,
) -> Result<(), Box<dyn Error>> {
    if loopit {
        loop {
            for v in 0..total_imgs {
                set_from_path(
                    path.join(PathBuf::from(format!("frame{v}.ppm")))
                        .to_str()
                        .unwrap(),
                )?;
                let _ = set_mode(mode.clone());
                sleep(Duration::from_millis(duration));
            }
        }
    } else {
        for v in 0..total_imgs {
            set_from_path(
                path.join(PathBuf::from(format!("frame{v}.ppm")))
                    .to_str()
                    .unwrap(),
            )?;
            let _ = set_mode(mode.clone());
            sleep(Duration::from_millis(duration));
        }
    }
    Ok(())
}
