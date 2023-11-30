use debug::DebugSystem;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color, ImageFormat},
    Context, GameResult,
};
use spec::{
    cmd::{self, CmdFile, CommandList},
    cns::CNSFile,
    constants::char_constants::{parse_char_constants, CharConstants},
    triggers::ExpressionContext,
};
use std::{collections::HashMap, env, fs, path};
mod game;
mod spec;
mod utils;
mod debug; 

use game::animation::Animator;
use game::{
    char::*,
    state_manager::{self, StateManager},
};
use utils::sprite_sheet::SpriteSheet;

struct MainState {
    char: CharState,
    char_sys: CharSystem,
    state_manager: StateManager,
    expression_context: ExpressionContext,
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sprite_sheet: SpriteSheet = SpriteSheet::new("./resources/kfm720.sff", ctx);
        let animator: Animator = Animator::new("./resources/kfm720.air");
        let cmd_file = CmdFile::new("./resources/kfm720.cmd");
        let command_list = CommandList::new(&cmd_file);
        let anim_no_set = animator.get_anim_action_no_set(); 
        let mut expression_context: ExpressionContext = ExpressionContext::new(ctx.gfx.size().0, anim_no_set);
        let input_states = cmd_file.parse_states();
        let command_states = CNSFile::new("./resources/common1.cns").get_states();
        let (char_cns, constants) = CNSFile::new("./resources/kfm720.cns").get_char_constants();
        expression_context.set_char_constants(&constants);
        let mut char_states = char_cns.get_states();
        char_states.extend(input_states);
        char_states.extend(command_states);
        let state_manager = StateManager::new(char_states);

        let char = CharBuilder::new()
            .animator(animator)
            .command_list(command_list)
            .build();
        let char_sys = CharSystem::new(sprite_sheet, constants);
        let s = MainState {
            char,
            char_sys,
            state_manager,
            expression_context,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            //self.rotation += 0.01;
            // self.char.update(ctx);
            let frame = self.char_sys.update(ctx);
            self.char
                .update(ctx.time.ticks() as i32, frame, &self.char_sys.constants);
            self.expression_context.update(&self.char);
            self.state_manager
                .update(&mut self.char, &mut self.expression_context);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw an image.

        let (a, b) = self.char.draw();
        let pos = self.char.position;
        let draw_pos = self.char.draw_position;
        let size = ctx.gfx.size();
        self.char_sys.draw(size, &mut canvas, draw_pos, pos, a, b);
        
        let debug_text = debug::char_debug(&self.char); 
        let debug_pos = Vec2::new(0.0, size.1-160.0); 
        DebugSystem::draw( debug_text, &mut canvas, debug_pos); 
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
