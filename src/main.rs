extern crate sdl2;

use std::time::Duration;
use std::collections::VecDeque;
use rand::{self, Rng};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 650;
const BACKGROUND_COLOR: Color = Color::RGB(252, 244, 233);
const BLOCK_SIZE: u32 = 25;
const APPLE_COLOR: Color = Color::RGB(214, 41, 41);
const SNAKE_COLOR: Color = Color::RGB(123, 145, 123);
const START_POSITION: (i32, i32) = (150, 150);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
        "Snake Game by Regina",
        SCREEN_WIDTH,
        SCREEN_HEIGHT
    )
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut apple = Block {
        rectangle: Rect::new(random_point().0, random_point().1, BLOCK_SIZE, BLOCK_SIZE),
        color: APPLE_COLOR,
    };

    let mut snake = VecDeque::new();

    snake.push_front(
        Block {
            rectangle: Rect::new(START_POSITION.0, START_POSITION.1, BLOCK_SIZE, BLOCK_SIZE),
            color: SNAKE_COLOR,
        },
    );

    snake.push_front(
        Block {
            rectangle: Rect::new(START_POSITION.0 + BLOCK_SIZE as i32, START_POSITION.1, BLOCK_SIZE, BLOCK_SIZE),
            color: SNAKE_COLOR,
        },
    );

    snake.push_front(
        Block {
            rectangle: Rect::new(START_POSITION.0 + 2 * BLOCK_SIZE as i32, START_POSITION.1, BLOCK_SIZE, BLOCK_SIZE),
            color: SNAKE_COLOR,
        },
    );

    let mut direction = Direction::Right;
    let mut scores = 0;

    let mut running = true;

    //GAME LOOP
    while running {
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                    break;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                    break;
                },

                // DIRECTION CHANGING
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    direction = Direction::Up
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    direction = Direction::Down
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    direction = Direction::Left
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    direction = Direction::Right
                },

                // PAUSE

                _ => {}
            }
        }

        apple.draw(&mut canvas);

        for segment in &snake {
            segment.draw(&mut canvas);
        }

        move_player(&mut snake, &direction);

        if collision(&mut snake) {
            running = false;
        }

        if score(&mut snake, &apple) {
            scores += 1;

            apple = Block {
                rectangle: Rect::new(random_point().0, random_point().1, BLOCK_SIZE, BLOCK_SIZE),
                color: APPLE_COLOR,
            };

            // New block that will be added to the snake
            let new_block = add_block(&mut snake, &direction);
            snake.push_back(new_block)
        }

        if running == false {
            break;
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }
}

enum Direction {Up, Down, Left, Right}

struct Block {
    rectangle: Rect,
    color: Color,
}

impl Block {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rectangle).expect("Failed to draw");
    }
}

fn random_point() -> (i32, i32) {
    let random_point = (
        rand::thread_rng().gen_range(BLOCK_SIZE..SCREEN_WIDTH-BLOCK_SIZE) as i32,
        rand::thread_rng().gen_range(BLOCK_SIZE..SCREEN_HEIGHT-BLOCK_SIZE) as i32,
    );

    (
        (random_point.0 / BLOCK_SIZE as i32) * BLOCK_SIZE as i32,
        (random_point.1 / BLOCK_SIZE as i32) * BLOCK_SIZE as i32
    )
}

fn move_player(snake: &mut VecDeque<Block>, direction: &Direction){
    let head = &mut snake[0].rectangle;

    let (mut future_coords_x, mut future_coords_y) = {
        match direction{
            Direction::Up => (head.x(), head.y() - BLOCK_SIZE as i32),
            Direction::Down => (head.x(), head.y() + BLOCK_SIZE as i32),
            Direction::Left => (head.x() - BLOCK_SIZE as i32, head.y()),
            Direction::Right => (head.x() + BLOCK_SIZE as i32, head.y()),
        }
    };

    for i in 0..snake.len(){
        let last_coords_x = snake[i].rectangle.x;
        let last_coords_y = snake[i].rectangle.y;

        snake[i].rectangle.set_x(future_coords_x);
        snake[i].rectangle.set_y(future_coords_y);

        future_coords_x = last_coords_x;
        future_coords_y = last_coords_y;
    }
}

fn collision(snake: &mut VecDeque<Block>) -> bool{
    let mut collision = false;

    // Collision with wall
    if snake[0].rectangle.x() < 0 || snake[0].rectangle.x() > SCREEN_WIDTH as i32 {
        collision = true;
    }
    else if snake[0].rectangle.y() < 0 || snake[0].rectangle.y() > SCREEN_HEIGHT as i32 {
        collision = true;
    }

    // Collision with own body
    for i in 3..snake.len() {
        if snake[0].rectangle == snake[i].rectangle {
            collision = true;
            break;
        }
    }

    collision
}

fn add_block(snake: &mut VecDeque<Block>, direction: &Direction) -> Block {
    let last_block = snake[snake.len() - 1].rectangle;

    match direction {
        Direction::Up => Block {
            rectangle: Rect::new(
                last_block.x(), last_block.y() + BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE
            ),
            color: SNAKE_COLOR
            },
        Direction::Down => Block {
            rectangle: Rect::new(
                last_block.x(), last_block.y() - BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE
            ),
            color: SNAKE_COLOR
        },
        Direction::Left => Block {
            rectangle: Rect::new(
                last_block.x() + BLOCK_SIZE as i32, last_block.y(), BLOCK_SIZE, BLOCK_SIZE
            ),
            color: SNAKE_COLOR
        },
        Direction:: Right => Block {
            rectangle: Rect::new(
                last_block.x() - BLOCK_SIZE as i32, last_block.y(), BLOCK_SIZE, BLOCK_SIZE
            ),
            color: SNAKE_COLOR
        },
    }
}

fn score(snake: &mut VecDeque<Block>, apple: &Block) -> bool{
    let mut score = false;
    if snake[0].rectangle == apple.rectangle {
        score = true;
    }
    score
}