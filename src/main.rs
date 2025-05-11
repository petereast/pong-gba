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

include_aseprite!(
    mod sprites,
    "gfx/sprites.aseprite"
);

struct Paddle {
    x: i32,
    y: i32,
    hflip: bool,
}

impl Paddle {
    fn new(start_x: i32, start_y: i32, hflip: bool) -> Self {
        Self {
            x: start_x,
            y: start_y,
            hflip,
        }
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(sprites::PADDLE_END.sprite(0))
            .set_pos((self.x, self.y))
            .set_hflip(self.hflip)
            .show(frame);

        Object::new(sprites::PADDLE_MID.sprite(0))
            .set_pos((self.x, self.y + 16))
            .set_hflip(self.hflip)
            .show(frame);

        Object::new(sprites::PADDLE_END.sprite(0))
            .set_pos((self.x, self.y + 32))
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

    let mut ball_x = 50;
    let mut ball_y = 50;

    let mut velocity_x = 0;
    let mut velocity_y = 0;

    let mut paddle_l = Paddle::new(8, 8, false);
    let mut paddle_r = Paddle::new(240 - 16 - 8, 8, true);

    frame.commit();

    loop {
        ball_x = (ball_x + velocity_x).clamp(0, agb::display::WIDTH - 16);
        ball_y = (ball_y + velocity_y).clamp(0, agb::display::HEIGHT - 16);

        velocity_x = input.x_tri() as i32;
        velocity_y = input.y_tri() as i32;

        if input.is_pressed(Button::A) {
            velocity_y = velocity_y * 2;
            velocity_x = velocity_x * 2;
        }

        ball.set_pos((ball_x, ball_y));
        let mut frame = gfx.frame();
        ball.show(&mut frame);

        paddle_l.show(&mut frame);
        paddle_r.show(&mut frame);

        frame.commit();
        input.update();
    }
}
