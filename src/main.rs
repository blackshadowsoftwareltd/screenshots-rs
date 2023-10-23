use screenshots::Screen;
use std::{path::PathBuf, time::Instant};

fn main() {
    let root = PathBuf::new().join("ScreenShots");
    let start = Instant::now();
    let screens = Screen::all().unwrap();

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
