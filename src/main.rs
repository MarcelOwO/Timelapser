use rfd::FileDialog;
use std::{env::home_dir, path::PathBuf};

mod video;
mod window;

fn main() {
    let picker = move || {
        let directory = FileDialog::new()
            .set_directory(home_dir().unwrap())
            .pick_folder()
            .unwrap();

        let mut imgs: Vec<PathBuf> = directory
            .read_dir()
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

        video::create_video(imgs);
    };

    window::setup_window(picker);
}
