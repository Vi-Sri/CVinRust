extern crate opencv;

use opencv::{
    highgui,
    imgcodecs,
};

fn run() -> opencv::Result<()> {
    let window: &str = "image capture";
    let image = imgcodecs::imread("../../assets/lena.jpg", imgcodecs::IMREAD_COLOR)?;
    highgui::named_window(window, 1)?;
    highgui::imshow(window, &image)?;
	highgui::wait_key(10000)?;
	Ok(())
}

fn main() {
    run().unwrap()
}