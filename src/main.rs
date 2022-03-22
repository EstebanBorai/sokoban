use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Sokoban"),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    loop {}
}
