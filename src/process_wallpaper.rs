use std::{error::Error, fmt::Display, path::Path, thread::sleep, time::Duration};

use wallpaper::{set_from_path, set_mode, Mode};

pub fn set_wallpaper(
    path: &Path,
    total_imgs: usize,
    mode: Mode,
    duration: u64,
    loopit: String,
) -> Result<(), Box<dyn Error>> {
    let times: Result<usize, _> = loopit.parse();
    if let Ok(time) = times {
        for _ in 0..time {
            set(total_imgs, path, duration, &mode)?;
        }
    } else if loopit.as_str().to_lowercase() == "forever" || loopit.is_empty() {
        loop {
            set(total_imgs, path, duration, &mode)?;
        }
    } else {
        return Err(Box::new(RehError::new(
            "Only Loop time number and forever is accept!",
        )));
    }
    Ok(())
}

#[derive(Debug)]
pub struct RehError {
    pub msg: String,
}

impl Display for RehError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl RehError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl Error for RehError {}

fn set(total_imgs: usize, path: &Path, duration: u64, mode: &Mode) -> Result<(), Box<dyn Error>> {
    for i in 0..total_imgs {
        let _ = set_mode(mode.clone());
        set_from_path(path.join(format!("frame{i}.png")).to_str().unwrap())?;
        sleep(Duration::from_millis(duration));
    }
    Ok(())
}
