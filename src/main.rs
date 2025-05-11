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

use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::include_aseprite;
use agb::input::Button;
use agb::fixnum::{Vector2D, vec2};

include_aseprite!(
    mod sprites,
    "gfx/sprites.aseprite"
);

struct Paddle {
    pos: Vector2D<i32>,
    hflip: bool,
}

impl Paddle {
    fn new(start_pos: Vector2D<i32>, hflip: bool) -> Self {
        Self {
            pos: start_pos,
            hflip,
        }
    }

    fn set_pos(&mut self, new_pos: Vector2D<i32>) {
        self.pos = new_pos;
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(sprites::PADDLE_END.sprite(0))
            .set_pos(self.pos)
            .set_hflip(self.hflip)
            .show(frame);

        Object::new(sprites::PADDLE_MID.sprite(0))
            .set_pos(self.pos + vec2(0, 16))
            .set_hflip(self.hflip)
            .show(frame);

        Object::new(sprites::PADDLE_END.sprite(0))
            .set_pos(self.pos + vec2(0, 32))
            .set_vflip(true)
            .set_hflip(self.hflip)
            .show(frame)
    }
}

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // agb::no_game(gba);

    let mut gfx = gba.graphics.get();
    let mut ball = Object::new(sprites::BALL.sprite(0));

    ball.set_pos((50, 50));

    let mut frame = gfx.frame();
    ball.show(&mut frame);

    let mut input = agb::input::ButtonController::new();

    let mut ball_pos = vec2(50, 50);
    let mut ball_velocity = vec2(2, 1);

    let mut paddle_l = Paddle::new(vec2(8, 8), false);
    let mut paddle_r = Paddle::new(vec2(240 - 16 - 8, 8), true);

    frame.commit();

    loop {

        ball_pos += ball_velocity;

        if ball_pos.x <= 0 || ball_pos.x >= agb::display::WIDTH - 16 {
            ball_velocity.x *= -1;
        }
        if ball_pos.y <= 0 || ball_pos.y >= agb::display::HEIGHT - 16 {
            ball_velocity.y *= -1;
        }

        if input.is_pressed(Button::A) {
            ball_velocity *= 2;
        }

        ball.set_pos(ball_pos);
        let mut frame = gfx.frame();
        ball.show(&mut frame);

        paddle_l.show(&mut frame);
        paddle_r.show(&mut frame);

        frame.commit();
        input.update();
    }
}
