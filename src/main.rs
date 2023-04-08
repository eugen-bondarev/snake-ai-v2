mod genetic;
mod snake;

use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

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
    canvas.set_pxl((FIELD_WIDTH + 1) as i32 + shift.0, shift.1, border_pixel);
    canvas.set_pxl(
        (FIELD_WIDTH + 1) as i32 + shift.0,
        (FIELD_HEIGHT + 1) as i32 + shift.1,
        border_pixel,
    );
    canvas.set_pxl(shift.0, (FIELD_HEIGHT + 1) as i32 + shift.1, border_pixel);

    for x in 0..FIELD_WIDTH + 1 {
        canvas.set_pxl(x as i32 + shift.0, shift.1, border_pixel);
        canvas.set_pxl(
            x as i32 + shift.0,
            (FIELD_HEIGHT + 1) as i32 + shift.1,
            border_pixel,
        );
    }
    for y in 0..FIELD_HEIGHT + 1 {
        canvas.set_pxl(shift.0, y as i32 + shift.1, border_pixel);
        canvas.set_pxl(
            (FIELD_WIDTH + 1) as i32 + shift.0,
            y as i32 + shift.1,
            border_pixel,
        );
    }
}

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

use rayon::prelude::*;

fn main() {
    let capacity = 5000;
    let mut snakes: Vec<Snake> = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        snakes.push(Snake::new());
    }
    let status_bar_height = 8;
    let mut engine = ConsoleEngine::init(
        (FIELD_WIDTH + 4 + 60).into(),
        (FIELD_HEIGHT + 4 + status_bar_height).into(),
        300,
    )
    .unwrap();

    let mut generation = 0;
    let mut max = 0;
    let mut max_fitness_prev = 0;
    let mut mutation_rate = 0.01;

    let mut draw = true;

    loop {
        engine.wait_frame();
        engine.clear_screen();

        let shift = (0, status_bar_height as i32);

        if draw {
            draw_borders(&mut engine, shift);
        }

        // let mut alive_snakes_num = 0;

        let batch_size = snakes.len() / 8;
        let batches: Vec<_> = snakes.chunks_mut(batch_size).collect();

        let max_fitness_current = Arc::new(Mutex::new(0));

        let alive_snakes_num = Arc::new(Mutex::new(0));

        batches.into_par_iter().for_each(|batch| {
            for item in batch {
                if item.get_score() > *max_fitness_current.lock().unwrap() {
                    *max_fitness_current.lock().unwrap() = item.get_score();
                }
                if !item.get_is_alive() {
                    continue;
                }
                item.tick();
                *alive_snakes_num.lock().unwrap() += 1;
            }
        });

        for snake in &mut snakes {
            if !snake.get_is_alive() {
                continue;
            }

            // snake.tick();

            if draw {
                engine.set_pxl(
                    snake.get_apple().current.0 + 1 + shift.0,
                    snake.get_apple().current.1 + 1 + shift.1,
                    pixel::pxl_bg(' ', Color::Red),
                );
                for cell in snake.get_cells() {
                    engine.set_pxl(
                        cell.current.0 + 1 + shift.0,
                        cell.current.1 + 1 + shift.1,
                        pixel::pxl_bg(' ', Color::Green),
                    );
                }
            }
            // alive_snakes_num += 1;
        }

        if *alive_snakes_num.lock().unwrap() == 0 {
            snakes.sort_by_key(|snake| (snake.get_score() as i32) * -1);
            let mut slice = snakes[0..capacity / 10].to_vec();

            let mut new_population: Vec<Snake> = vec![];

            let progress = *max_fitness_current.lock().unwrap() > max_fitness_prev;
            if progress {
                mutation_rate -= mutation_rate * 0.1;
            } else {
                mutation_rate += mutation_rate * 0.1;
            }
            mutation_rate = f64::clamp(mutation_rate, 0.00005 as f64, 0.05 as f64);

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
                new_population.push(Snake::crossover(&parent_a, &parent_b, mutation_rate));
            }

            snakes.clear();
            for snake in new_population {
                snakes.push(snake);
            }

            for snake in &mut snakes {
                snake.reborn();
            }
            max = slice[0].get_score();
            if max > max_fitness_prev {
                max_fitness_prev = max;
            }
            generation += 1;
        }

        engine.print(
            1,
            0,
            format!("snakes_alive: {}", *alive_snakes_num.lock().unwrap(),).as_str(),
        );

        engine.print(1, 1, format!("generation: {}", generation,).as_str());
        engine.print(1, 2, format!("mutation_rate: {}", mutation_rate).as_str());
        engine.print(
            1,
            3,
            format!(
                "max_fitness_current: {}",
                *max_fitness_current.lock().unwrap()
            )
            .as_str(),
        );
        engine.print(
            1,
            4,
            format!("max_fitness_prev: {}", max_fitness_prev).as_str(),
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

        if engine.is_key_pressed(KeyCode::Char(' ')) {
            draw = !draw;
        }

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        engine.draw();
    }
}
