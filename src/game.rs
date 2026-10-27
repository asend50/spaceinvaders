use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::collision::check_collision;
use crate::custom::player::Player;
use crate::custom::bullet::Bullet;
use crate::custom::enemy::Enemy;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32,) -> (String, TextureManager, i32 ){

    let mut player = Player::new(
    500.0,                   // X position
    680.0,                   // Y position
    screen_width() * 0.07,  // Width
    screen_height() * 0.08, // Height
    "assets/player.png",    // Image name
    true,                   // Enable stretching
    1.0,                    // Zoom level (100%)
    3.0,                    // Speed
    vec2(35.0, 15.0),       // Old position
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

let mut bullets: Vec<Bullet> = Vec::new();
let mut enemies: Vec<Enemy> = Vec::new();
let columns = 6;
let rows = 3;
let spacing_x = screen_width() * 0.08;
let spacing_y = screen_height() * 0.09;
let start_x = 240.0;
let start_y = 120.0;

for row in 0..rows {
    for col in 0..columns {
        let mut enemy = Enemy::new(
            start_x + col as f32 * spacing_x,
            start_y + row as f32 * spacing_y,
            screen_width() * 0.055,
            screen_height() * 0.065,
            "assets/enemy.png",
            true,
            1.0,
            3.0,
        ).await;
        enemies.push(enemy);
    }
}

let mut shoot_cooldown = 0.0;


    loop {
        clear_background(WHITE);
        background.draw();
        for enemy in &enemies {
            enemy.get_image().draw();
        }
        player.get_image().draw();


        player.keypress();
        player.move_player();
        for enemy in &mut enemies {
            enemy.enemymovement();
        }


        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &gamebars, 1) {
                player.move_back_x();
            }
        }

        let time = get_frame_time();
        shoot_cooldown -= time;

         if is_key_down(KeyCode::Space) && shoot_cooldown <= 0.0 {
        shoot_cooldown = 0.75;

        let mut bullet = Bullet::new(
            player.image.get_x() + 25.0,
            player.image.get_y(),
            screen_width() * 0.02,
            screen_height() * 0.05,
            "assets/bullet.png",
            true,
            1.0,
            3.0,
        ).await;

        bullets.push(bullet);
    }

    for bullet_index in (0..bullets.len()).rev() {
        let bullet = &mut bullets[bullet_index];
        bullet.shoot();
        bullet.get_image().draw();

        if bullet.image.get_y() < -100.0 {
            bullet.image.clear();
            bullet.move_dir = vec2(0.0, 0.0);
            bullets.remove(bullet_index);
            continue;
        }

        for enemy_index in (0..enemies.len()).rev() {
            if check_collision(bullet.get_image(), enemies[enemy_index].get_image(), 1) {
                enemies.remove(enemy_index);
                bullets.remove(bullet_index);
                break;
            }
        }
    }


    let mut last_shot = 0.0;

if is_key_down(KeyCode::Space) && get_time() - last_shot > 1.0 {
    last_shot = get_time();
}



    gamebars.draw();

        next_frame().await;
    }
}