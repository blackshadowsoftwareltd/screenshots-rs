use anyhow::Result;
use core_graphics::display::CGDisplay;
use core_graphics::image::CGImage;
use image::{ImageBuffer, Rgb};
use std::path::PathBuf;

#[cfg(target_os = "linux")]
fn linux(root: PathBuf) -> Result<()> {
    let start = Instant::now();
    let screens = Screen::all()?;

    for screen in screens {
        println!("capturer {:?}", screen);

        let mut image = screen.capture().unwrap();
        let path = root
            .clone()
            .join(format!("full_screen{}.png", screen.display_info.id));
        image.save(path.clone()).unwrap();

        image = screen.capture_area(300, 300, 300, 300).unwrap();
        let path = root
            .clone()
            .join(format!("capture_area{}.png", screen.display_info.id));
        image.save(path.clone()).unwrap();
    }

    let screen = Screen::from_point(100, 100).unwrap();
    let image = screen.capture_area(300, 300, 300, 300).unwrap();
    let path = root
        .clone()
        .join(format!("100_100_300{}.png", screen.display_info.id));
    image.save(path.clone()).unwrap();

    println!("Total Elapsed Time : {:?}", start.elapsed());
}

#[cfg(target_os = "macos")]
fn macos(root: PathBuf) -> Result<()> {
    use anyhow::bail;

    let display = CGDisplay::main();

    // let image: CGImage = display.image()?;
    match display.image() {
        Some(image) => {
            let width = image.width() as u32;
            let height = image.height() as u32;

            let data = image.data();
            let bytes_per_row = image.bytes_per_row();

            let mut buffer = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let offset = (y as usize * bytes_per_row) + (x as usize * 4);
                    let pixel = &data[offset..offset + 4];
                    buffer.put_pixel(x, y, Rgb([pixel[2], pixel[1], pixel[0]]));
                }
            }
            let path = root.join("Full_Screen.png");
            buffer.save(path.clone())?;

            println!("Screenshot saved to {:?}", path);
            Ok(())
        }
        None => bail!("Failed to init image"),
    }
}

fn main() {
    let root = PathBuf::new().join("ScreenShots");

    // Conditionally call platform-specific functions
    #[cfg(target_os = "linux")]
    linux(root);

    #[cfg(target_os = "macos")]
    if let Err(e) = macos(root) {
        eprintln!("Error on macOS: {}", e);
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    println!("This platform is not supported.");
}
