use ggez::{
    graphics::{self, ImageFormat},
    Context,
};
use std::{collections::HashMap, fs};

pub struct Sprite {
    axis_x: u16,
    axis_y: u16,
    width: u32,
    height: u32,
    image: graphics::Image,
}

pub struct SpriteSheet {
    map: HashMap<(u16, u16), Sprite>,
}

impl SpriteSheet {
    pub fn new(file_path: &str, ctx: &mut Context) -> SpriteSheet {
        let map = SpriteSheet::convert_sff_to_images(file_path, ctx);
        SpriteSheet { map }
    }

    fn convert_sff_to_images(file_name: &str, ctx: &mut Context) -> HashMap<(u16, u16), Sprite> {
        let sff = fs::read(file_name).expect("Failted to read SFF file");
        let sff = sff_rs::SFF::decode(&sff).unwrap();
        let sff_map: sff_rs::SFFMap = sff_rs::SFFMap::from(sff);
        let mut map: HashMap<(u16, u16), Sprite> = HashMap::new();

        for (id, image) in sff_map {
            let ez_image = graphics::Image::from_pixels(
                ctx,
                image.image_data.as_ref(),
                ImageFormat::Rgba8UnormSrgb,
                image.width,
                image.height,
            );
            let sprite = Sprite {
                image: ez_image,
                width: image.width,
                height: image.height,
                axis_x: image.axis_x,
                axis_y: image.axis_y,
            };
            map.insert(id, sprite);
        }
        map
    }

    pub fn get_axis(&self, group: u16, image: u16) -> (u16, u16) {
        let sprite = self.map.get(&(group, image)).unwrap();
        (sprite.axis_x, sprite.axis_y)
    }
    pub fn get_size(&self, group: u16, image: u16) -> (u32, u32) {
        let sprite = self.map.get(&(group, image)).unwrap();
        (sprite.width, sprite.height)
    }

    pub fn get(&self, group: u16, image: u16) -> &graphics::Image {
        &self.map.get(&(group, image)).unwrap().image
    }
}
