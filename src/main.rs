extern crate piston_window;

use piston_window::*;
use std::collections::HashSet;

const HEIGHT: f64 = 640.0;
const WIDTH: f64 = 480.0;

struct Paddle {
    width: f64,
    height: f64,
}

struct Ball {
    width: f64,
    height: f64,
    dx: f64,
    dy: f64,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Pong by Dave", [HEIGHT as u32, WIDTH as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut left_paddle = Paddle { width: 30.0, height: WIDTH / 2.0 - 50.0 };
    let mut right_paddle = Paddle { width: HEIGHT - 40.0, height: WIDTH / 2.0 - 50.0 };
    let mut ball = Ball { width: HEIGHT / 2.0, height: WIDTH / 2.0, dx: 0.8, dy: 0.8};
 
    let mut keys_held = HashSet::new(); 

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
            println!("Game Paused");
            while let Some(e) = window.next() {
                if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
                    println!("Game Resumed");
                    break;
                }
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            keys_held.insert(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            keys_held.remove(&key);
        }

        if keys_held.contains(&Key::W) && left_paddle.height > 0.0 {
            left_paddle.height -= 2.0;
        }
        if keys_held.contains(&Key::S) && left_paddle.height < WIDTH - 100.0 {
            left_paddle.height += 2.0;
        }
        if keys_held.contains(&Key::Up) && right_paddle.height > 0.0 {
            right_paddle.height -= 2.0;
        }
        if keys_held.contains(&Key::Down) && right_paddle.height < WIDTH - 100.0 {
            right_paddle.height += 2.0;
        }

        //Mannually incrementing to change ball position
        ball.width += ball.dx;
        ball.height += ball.dy;

        //Inverting ball direction when it his paddles
        if ball.height <= 0.0 || ball.height >= WIDTH {
            ball.dy = -ball.dy;
        }

        // Ball collision with paddles
        if ball.width <= left_paddle.width + 10.0
            && ball.height >= left_paddle.height
            && ball.height <= left_paddle.height + 100.0
        {
            ball.dx = -ball.dx;
        }

        if ball.width >= right_paddle.width - 10.0
            && ball.height >= right_paddle.height
            && ball.height <= right_paddle.height + 100.0
        {
            ball.dx = -ball.dx;
        }

        //Reset Ball when points scored
        if ball.width < 0.0 || ball.width > HEIGHT {
            ball.width = HEIGHT / 2.0;
            ball.height = WIDTH / 2.0;
            ball.dx = -ball.dx;
            ball.dy = -ball.dy;
        }

        //Drawing paddles and ball
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            rectangle(
                [0.0, 0.0, 1.0, 1.0], // blue color for left paddle
                [left_paddle.width, left_paddle.height, 10.0, 100.0],
                c.transform,
                g,
            );

            rectangle(

                [1.0, 0.0, 0.0, 1.0], // red color for right paddle
                [right_paddle.width, right_paddle.height, 10.0, 100.0],
                c.transform,
                g,
            );

            rectangle(
                [1.0, 1.0, 1.0, 1.0], // white color for ball
                [ball.width - 5.0, ball.height - 5.0, 10.0, 10.0],
                c.transform,
                g,
            );
        });
    }
}
