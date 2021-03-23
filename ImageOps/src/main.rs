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
    highgui::named_window(window, 1)?;
    #[cfg(feature = "opencv-4")]
    let mut cam = videoio::VideoCapture::new_default(0)?;  
    #[cfg(not(feature = "opencv-4"))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; 
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
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