use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::collision::check_collision;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32,) -> (String, TextureManager, i32 ){
    
    let mut menu= StillImage::from_preload(
    tm.get_preload("assets/menu.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

    loop {
        clear_background(WHITE);
        menu.draw();

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, score);
        }

        next_frame().await;
    }
}