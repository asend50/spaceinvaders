use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::collision::check_collision;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32,) -> (String, TextureManager, i32 ){
    loop {
        clear_background(DARKGRAY);

        next_frame().await;
    }
}