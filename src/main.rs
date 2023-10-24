mod compiler;
mod ini;
mod bytecode; 
mod compiler_functions;
mod utils; 
mod types; 

use air_rs::*;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color, ImageFormat},
    Context, GameResult,
};
use std::collections::HashSet;
use std::{collections::HashMap, env, fs, path};

struct MainState {
    char: Char,
}

struct SpriteSheet {
    map: HashMap<(u16, u16), graphics::Image>,
}

impl SpriteSheet {
    fn new(file_path: &str, ctx: &mut Context) -> SpriteSheet {
        let map = SpriteSheet::convert_sff_to_images(file_path, ctx);
        SpriteSheet { map }
    }

    fn convert_sff_to_images(
        file_name: &str,
        ctx: &mut Context,
    ) -> HashMap<(u16, u16), graphics::Image> {
        let sff = fs::read(file_name).expect("Failted to read SFF file");
        let sff = sff_rs::SFF::decode(&sff).unwrap();
        let sff_map: sff_rs::SFFMap = sff_rs::SFFMap::from(sff);
        let mut map: HashMap<(u16, u16), graphics::Image> = HashMap::new();

        for (id, image) in sff_map {
            let ez_image = graphics::Image::from_pixels(
                ctx,
                image.image_data.as_ref(),
                ImageFormat::Rgba8UnormSrgb,
                image.width,
                image.height,
            );
            map.insert(id, ez_image);
        }
        map
    }

    fn get(&self, group: u16, image: u16) -> &graphics::Image {
        self.map.get(&(group, image)).unwrap()
    }
}

struct Char {
    sprite_sheet: SpriteSheet,
    animator: Animator,
    position: Vec2,
    direction: Vec2,
}

impl Char {
    fn update(&mut self, context: &mut Context) {
        if context
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Right)
        {
            if self.animator.current_action != 20 {
                self.animator.set_action(20);
            }
            self.direction = Vec2::new(1.45, 0.0);
            self.position = self.position + self.direction;
        } else if context
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Left)
        {
            if self.animator.current_action != 21 {
                self.animator.set_action(21);
            }
            self.direction = -Vec2::new(1.40, 0.0);
            self.position = self.position + self.direction;
        } else if context
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Down)
        {
            if self.animator.current_action != 11 {
                self.animator.set_action(11);
            }
            self.direction = Vec2::new(0.0, 0.0);
        } else {
            if self.animator.current_action != 0 {
                self.animator.set_action(0);
            }
            self.direction = Vec2::new(0.0, 0.0);
        }
        self.animator.update();
    }

    fn draw(&mut self, canvas: &mut graphics::Canvas) {
        let (group, image) = self.animator.draw();
        canvas.draw(
            self.sprite_sheet.get(group, image),
            graphics::DrawParam::new().dest(self.position),
        );
    }
}
struct CharBuilder {
    sprite_sheet: Option<SpriteSheet>,
    animator: Option<Animator>,
    position: Vec2,
    direction: Vec2,
}

impl CharBuilder {
    fn new() -> Self {
        Self {
            sprite_sheet: None,
            animator: None,
            position: Vec2::new(20.0, 20.0),
            direction: Vec2::new(0.0, 0.0),
        }
    }

    fn sprite_sheet(mut self, sprite_sheet: SpriteSheet) -> Self {
        self.sprite_sheet = Some(sprite_sheet);
        self
    }
    fn animator(mut self, animator: Animator) -> Self {
        self.animator = Some(animator);
        self
    }

    fn build(self) -> Char {
        Char {
            sprite_sheet: self.sprite_sheet.unwrap(),
            animator: self.animator.unwrap(),
            position: self.position,
            direction: self.direction,
        }
    }
}

struct Animator {
    action_map: HashMap<u64, Action>,
    time: u64,
    frame_time: i64,
    current_action: u64,
    current_element: usize,
    shown_animations: HashSet<String>,
    loop_start: usize,
}

impl Animator {
    fn new(air_file_path: &str) -> Self {
        let action_map = Animator::load_air(air_file_path);
        Self {
            time: 0,
            current_action: 0,
            current_element: 0,
            frame_time: 0,
            shown_animations: HashSet::new(),
            loop_start: 0,
            action_map,
        }
    }

    fn load_air(file_path: &str) -> HashMap<u64, Action> {
        let unparsed_file = fs::read_to_string(file_path).expect("cannot read file");
        air_rs::parse(&unparsed_file).unwrap()
    }

    fn update(&mut self) {
        let action = self.action_map.get(&self.current_action).unwrap();
        let current_frame = action.elements.get(self.current_element).unwrap();

        self.loop_start = action.loop_start;
        self.time += 1;
        self.frame_time += 1;
        //let x = current_frame.x;
        //let y = current_frame.y;
        if self.frame_time > current_frame.time {
            self.current_element = if self.current_element + 1 >= action.elements.len() {
                self.loop_start
            } else {
                self.current_element + 1
            };
            self.frame_time = 0;
        }
    }

    fn set_action(&mut self, action_no: u64) {
        self.current_action = action_no;
        self.current_element = 0;
        self.time = 0;
        self.frame_time = 0;
    }

    fn draw(&self) -> (u16, u16) {
        let action = self.action_map.get(&self.current_action).unwrap();
        let current_frame = action.elements.get(self.current_element);
        (
            current_frame.unwrap().group.try_into().unwrap(),
            current_frame.unwrap().image.try_into().unwrap(),
        )
    }
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sprite_sheet: SpriteSheet = SpriteSheet::new("./resources/kfm720.sff", ctx);
        let animator: Animator = Animator::new("./resources/kfm720.air");
        let char = CharBuilder::new()
            .animator(animator)
            .sprite_sheet(sprite_sheet)
            .build();
        let s = MainState { char };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            //self.rotation += 0.01;
            self.char.update(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw an image.
        self.char.draw(&mut canvas);

        // // Draw an image with some options, and different filter modes.
        // let dst = glam::Vec2::new(200.0, 100.0);
        // let dst2 = glam::Vec2::new(400.0, 400.0);
        // let scale = glam::Vec2::new(10.0, 10.0);

        // canvas.draw(
        //     &self.image2,
        //     graphics::DrawParam::new()
        //         .dest(dst)
        //         .rotation(self.rotation)
        //         .scale(scale),
        // );
        // canvas.set_sampler(graphics::Sampler::nearest_clamp());
        //canvas.set_default_sampler();

        // Finished drawing, show it all on the screen!
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("ik_ggez", "fantasma").add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
