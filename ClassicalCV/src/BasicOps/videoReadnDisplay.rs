extern crate opencv;

use opencv::{
    core,
    highgui,
    prelude::*,
    videoio,
};

fn run() -> opencv::Result<()> {
    let window = "video capture";
    let video_file  = "../../assets/dtp3.mp4";
    
    highgui::named_window(window, 1)?;
    #[cfg(not(feature = "opencv-4"))]
    let mut videostream = videoio::VideoCapture::from_file(video_file, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&videostream)?;
    if !opened {
        panic!("Unable to read frame from video or end of video!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        videostream.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}