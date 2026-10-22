use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::collision::check_collision;
use crate::custom::player::Player;
use crate::custom::bullet::Bullet;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32,) -> (String, TextureManager, i32 ){

    let mut player = Player::new(
    500.0,                   // X position
    500.0,                   // Y position
    screen_width() * 0.07,  // Width
    screen_height() * 0.08, // Height
    "assets/player.png",    // Image name
    true,                   // Enable stretching
    1.0,                    // Zoom level (100%)
    3.0,                    // Speed
    vec2(35.0, 15.0),       // Old position
).await;

let mut bullet = Bullet::new(
    500.0,                   // X position
    500.0,                   // Y position
    screen_width() * 0.02,  // Width
    screen_height() * 0.05, // Height
    "assets/bullet.png",    // Image name
    true,                   // Enable stretching
    1.0,                    // Zoom level (100%)
    3.0,                    // Speed
).await;

    let mut background= StillImage::from_preload(
    tm.get_preload("assets/gamebackground.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut gamebars= StillImage::from_preload(
    tm.get_preload("assets/gamebars.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut bullets = vec![];

    loop {
        clear_background(WHITE);
        background.draw();
        gamebars.draw();        
        bullet.get_image().draw();
        player.get_image().draw();


        player.keypress();
        player.move_player();


        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &gamebars, 1) {
                player.move_back_x();
            }
        }

        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &gamebars, 1) {
                player.move_back_y();
            }
        }

        if is_key_pressed(KeyCode::Space) {
            bullets.push(1);
            bullet.image.set_x(player.image.get_x() + 25.0);
            bullet.image.set_y(player.image.get_y());
        }

        next_frame().await;
    }
}