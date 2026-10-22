/*
By: <Your Name Here>
Date: 2026-10-07
Program Details: <Program Description Here>
*/

mod ui;
mod utils;
mod custom;
mod menu;
mod game;
mod win;
mod lose;

use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::collision::check_collision;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "spaceinvaders".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut tm = TextureManager::new();

    tm.preload_with_loading_screen(&["assets/menu.png", "assets/player.png", "assets/enemy.png", "assets/bullet.png", "assets/gamebackground.png", "assets/gamebars.png"], None, None).await;

    let mut score = 0;
    
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, tm, score) = match current_screen.as_str() {
                "menu" => menu::run(tm, score).await,
                "game" => game::run(tm, score).await,
                "win" => win::run(tm, score).await,
                "lose" => lose::run(tm, score).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}