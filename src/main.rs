mod genetic;
mod snake;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use console_engine::{pixel, Color, ConsoleEngine, KeyCode};
use dfdx::{
    prelude::{DeviceBuildExt, Linear, Module, ModuleMut, Sigmoid},
    shapes::Rank1,
    tensor::{Cpu, Tensor, ZerosTensor},
};
use rand::{thread_rng, Rng};
use snake::{Direction, Snake, FIELD_HEIGHT, FIELD_WIDTH};

fn draw_borders(canvas: &mut ConsoleEngine, shift: (i32, i32)) {
    let border_color = Color::DarkRed;
    let border_pixel = pixel::pxl_bg(' ', border_color);

    canvas.set_pxl(shift.0, shift.1, border_pixel);
    canvas.set_pxl((FIELD_WIDTH + 3) as i32 + shift.0, shift.1, border_pixel);
    canvas.set_pxl(
        (FIELD_WIDTH + 3) as i32 + shift.0,
        (FIELD_HEIGHT + 3) as i32 + shift.1,
        border_pixel,
    );
    canvas.set_pxl(shift.0, (FIELD_HEIGHT + 3) as i32 + shift.1, border_pixel);

    for x in 0..FIELD_WIDTH + 3 {
        canvas.set_pxl(x as i32 + shift.0, shift.1, border_pixel);
        canvas.set_pxl(
            x as i32 + shift.0,
            (FIELD_HEIGHT + 3) as i32 + shift.1,
            border_pixel,
        );
    }
    for y in 0..FIELD_HEIGHT + 3 {
        canvas.set_pxl(shift.0, y as i32 + shift.1, border_pixel);
        canvas.set_pxl(
            (FIELD_WIDTH + 3) as i32 + shift.0,
            y as i32 + shift.1,
            border_pixel,
        );
    }
}

fn create_bit_mask(intersections: u8) -> u32 {
    // return u32::from_str_radix("00000000000111110000000011111111", 2).unwrap();
    let mut remaining_capacity = 32;
    let mut partitions: Vec<u8> = vec![0; (intersections).into()]
        .iter()
        .map(|_| {
            let result = thread_rng().gen_range(0..remaining_capacity);
            remaining_capacity -= result;
            result
        })
        .collect();

    // partitions.push(remaining_capacity);

    let mut result = String::from("");
    let mut starting_bit = "0";
    for i in partitions {
        for _ in 0..i {
            result += starting_bit;
            if result.len() == 32 {
                break;
            }
        }
        starting_bit = if starting_bit == "0" { "1" } else { "0" };
    }

    u32::from_str_radix(result.as_str(), 2).unwrap()
}

use rand::prelude::*;
use rand_distr::{Distribution, Normal};

fn generate_random_number_tending_towards_smaller(n: u32, m: u32, small_likelihood: f64) -> u32 {
    let mean = (n + m) / 2;
    let std_dev = (m - n) / 4;
    let normal = Normal::new(mean as f64, std_dev as f64).unwrap();

    let mut rng = thread_rng();
    let mut num;
    loop {
        num = normal.sample(&mut rng) as u32;
        if num >= n && num <= m {
            break;
        }
    }

    let rand_num = rng.gen_range(0.0..1.0);
    if rand_num <= small_likelihood {
        n + rng.gen_range(0..(num - n).max(1))
    } else {
        num + rng.gen_range(0..(m - num + 1).max(1))
    }
}

fn test() {
    let total = 100;
    let mut acc = 0;
    for _ in 0..100 {
        let num = generate_random_number_tending_towards_smaller(0, 25, 0.0);
        acc += num;

        // let formatted = format!("{:032b}", u);
        // if formatted.len() != 32 {
        //     println!("{}", formatted);
        //     println!("{}", formatted.len());
        // }
    }
    let avg = acc as f32 / total as f32;
    println!("Avg: {}", avg);
}

fn main() {
    // test();
    // return;
    // let mut snakes: Vec<Snake> = vec![Snake::new()];
    let capacity = 2000;
    let mut snakes: Vec<Snake> = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        snakes.push(Snake::new());
    }
    let status_bar_height = 3;
    let mut engine = ConsoleEngine::init(
        (FIELD_WIDTH + 4 + 60).into(),
        (FIELD_HEIGHT + 4 + status_bar_height).into(),
        300,
    )
    .unwrap();

    let mut generations = 0;
    let mut max = 0;
    let mut total_max = 0;

    loop {
        engine.wait_frame();
        engine.clear_screen();

        let shift = (0, status_bar_height as i32);

        draw_borders(&mut engine, shift);

        let mut alive_snakes_num = 0;

        for snake in &mut snakes {
            if !snake.get_is_alive() {
                continue;
            }

            snake.tick();
            engine.set_pxl(
                snake.get_apple().current.0 + 2 + shift.0,
                snake.get_apple().current.1 + 2 + shift.1,
                pixel::pxl_bg(' ', Color::Red),
            );
            for cell in snake.get_cells() {
                engine.set_pxl(
                    cell.current.0 + 2 + shift.0,
                    cell.current.1 + 2 + shift.1,
                    pixel::pxl_bg(' ', Color::Green),
                );
            }
            alive_snakes_num += 1;
        }

        if alive_snakes_num == 0 {
            snakes.sort_by_key(|snake| (snake.get_score() as i32) * -1);
            let mut slice = snakes[0..100].to_vec();

            let mut new_population: Vec<Snake> = vec![];

            for i in (0..capacity).step_by(2) {
                let parent_a = &slice[generate_random_number_tending_towards_smaller(
                    0,
                    slice.len() as u32 - 1,
                    0.9,
                ) as usize];
                let parent_b = &slice[generate_random_number_tending_towards_smaller(
                    0,
                    slice.len() as u32 - 1,
                    0.9,
                ) as usize];
                new_population.push(Snake::crossover(&parent_a, &parent_b));
            }

            snakes.clear();
            for snake in new_population {
                snakes.push(snake);
            }

            for snake in &mut snakes {
                snake.reborn();
            }
            max = slice[0].get_score();
            if max > total_max {
                total_max = max;
            }
            generations += 1;
        }

        engine.print(
            1,
            0,
            format!(
                "Score: {}, snakes alive: {}, max fitness: {}, max max: {}, generations: {}",
                snakes[0].get_score(),
                alive_snakes_num,
                max,
                total_max,
                generations
            )
            .as_str(),
        );
        engine.print(
            1,
            1,
            &format!("Prediction: {:?}", snakes[0].get_nn_prediction()),
        );

        if engine.is_key_pressed(KeyCode::Char('d')) {
            snakes[0].set_direction(Direction::Right);
        }

        if engine.is_key_pressed(KeyCode::Char('a')) {
            snakes[0].set_direction(Direction::Left);
        }

        if engine.is_key_pressed(KeyCode::Char('w')) {
            snakes[0].set_direction(Direction::Up);
        }

        if engine.is_key_pressed(KeyCode::Char('s')) {
            snakes[0].set_direction(Direction::Down);
        }

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        engine.draw();
    }
}
