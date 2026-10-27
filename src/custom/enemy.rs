use crate::ui::still_image::StillImage;
use macroquad::prelude::*;

pub struct Enemy {
    pub image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
}
impl Enemy {
    pub async fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_name: &str,
        stretch: bool,
        zoom: f32,
        speed: f32,
    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Enemy {
            image,
            speed: 1.0,
            move_dir: vec2(0.0, 0.0),
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn enemymovement(&mut self) {

        for _ in 0..25 {
            self.image.set_x(self.image.get_x() + 2.0);
        }

        for _ in 0..25 {
            self.image.set_x(self.image.get_x() - 2.0);
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
   
}
