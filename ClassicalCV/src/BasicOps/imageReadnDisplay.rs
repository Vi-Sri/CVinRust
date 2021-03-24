extern crate opencv;

use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*
};

fn run() -> opencv::Result<()> {
    let window: &str = "image capture";
    let image_file: &str  = "/assets/dtp3.mp4";
    
    highgui::named_window(window, 1)?;
    
    
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