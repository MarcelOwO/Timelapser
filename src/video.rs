use std::path::PathBuf;
extern crate ffmpeg_next as ffmpeg;

pub(crate) fn create_video(files: Vec<PathBuf>) {
    ffmpeg::init().unwrap();
}
