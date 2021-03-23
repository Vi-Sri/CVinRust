extern crate opencv;

use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    videoio,
};

fn run() -> opencv::Result<()> {
    let window = "video capture";
    let video_file  = "/Users/srinivas_v/Work/Projects/CarCounterYOLOv3/videos/dtp3.mp4";
    
    highgui::named_window(window, 1)?;
    #[cfg(feature = "opencv-4")]
    let mut cam = videoio::VideoCapture::new_default(0)?;  
    #[cfg(not(feature = "opencv-4"))]

    let mut videostream = videoio::VideoCapture::from_file(video_file, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&videostream)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        videostream.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = core::Mat::default()?;
            imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0
            )?;
            highgui::imshow(window, &gray)?;
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