// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        GraphicsFrame, Priority,
        object::Object,
        tiled::{RegularBackgroundSize, RegularBackgroundTiles, TileFormat, VRAM_MANAGER},
    },
    fixnum::{Num, Rect, Vector2D, num, vec2},
    include_aseprite, include_background_gfx,
    input::Button,
};

type Fixed = Num<i32, 8>;

include_aseprite!(
    mod sprites,
    "gfx/sprites.aseprite"
);

include_background_gfx!(mod background, PLAY_FIELD => deduplicate "gfx/play_field.aseprite");

struct Paddle {
    pos: Vector2D<Fixed>,
    hflip: bool,
}

impl Paddle {
    fn new(start_pos: Vector2D<Fixed>, hflip: bool) -> Self {
        Self {
            pos: start_pos,
            hflip,
        }
    }

    fn set_pos(&mut self, new_pos: Vector2D<Fixed>) {
        self.pos = new_pos;
    }

    fn move_by(&mut self, y: Fixed) {
        self.pos += vec2(num!(0), y);
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        let sprite_pos = self.pos.round();
        Object::new(sprites::PADDLE_END.sprite(0))
            .set_pos(sprite_pos)
            .set_hflip(self.hflip)
            .show(frame);

        Object::new(sprites::PADDLE_MID.sprite(0))
            .set_pos(sprite_pos + vec2(0, 16))
            .set_hflip(self.hflip)
            .show(frame);

        Object::new(sprites::PADDLE_END.sprite(0))
            .set_pos(sprite_pos + vec2(0, 32))
            .set_vflip(true)
            .set_hflip(self.hflip)
            .show(frame)
    }

    fn collision_rect(&self) -> Rect<Fixed> {
        Rect::new(self.pos, vec2(num!(16), num!(16 * 3)))
    }
}

struct Ball {
    pos: Vector2D<Fixed>,
    velocity: Vector2D<Fixed>,
}

impl Ball {
    fn new() -> Self {
        Self {
            pos: vec2(num!(50), num!(50)),
            velocity: vec2(num!(2), num!(0.5)),
        }
    }

    fn update(&mut self, paddle_l: &Paddle, paddle_r: &Paddle) {
        let possible_next_ball_pos = self.pos + self.velocity;
        let ball_rect = Rect::new(possible_next_ball_pos, vec2(num!(16), num!(16)));

        if paddle_l.collision_rect().touches(ball_rect) {
            self.velocity.x = self.velocity.x.abs();

            let y_difference = (ball_rect.centre().y - paddle_l.collision_rect().centre().y) / 32;
            self.velocity.y += y_difference
        }

        if paddle_r.collision_rect().touches(ball_rect) {
            self.velocity.x = -self.velocity.x.abs();

            let y_difference = (ball_rect.centre().y - paddle_r.collision_rect().centre().y) / 32;
            self.velocity.y += y_difference

        }

        let rounded_possible_next_pos = possible_next_ball_pos.round();

        if rounded_possible_next_pos.x <= 0 || rounded_possible_next_pos.x >= agb::display::WIDTH - 16 {
            self.velocity.x *= num!(-1);
        }
        if rounded_possible_next_pos.y <= 0 || rounded_possible_next_pos.y >= agb::display::HEIGHT - 16 {
            self.velocity.y *= num!(-1);
        }


        self.pos += self.velocity;
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        let rounded_pos = self.pos.round();
        Object::new(sprites::BALL.sprite(0))
            .set_pos(rounded_pos)
            .show(frame);
    }
}

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();

    VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut bg = RegularBackgroundTiles::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    bg.fill_with(&background::PLAY_FIELD);

    let mut input = agb::input::ButtonController::new();

    let mut ball = Ball::new();
    let mut paddle_l = Paddle::new(vec2(num!(8), num!(8)), false);
    let paddle_r = Paddle::new(vec2(num!(240 - 16 - 8), num!(8)), true);

    loop {
        let mut frame = gfx.frame();

        ball.update(&paddle_l, &paddle_r);
        ball.show(&mut frame);

        paddle_l.show(&mut frame);
        paddle_r.show(&mut frame);

        bg.show(&mut frame);
        frame.commit();
        input.update();
        paddle_l.move_by(Num::from(input.y_tri() as i32));
    }
}
