mod genetic;
mod snake;

use std::thread;

use console_engine::{pixel, Color, ConsoleEngine, KeyCode};
use genetic::{population::Population, traits::HasLife};
use snake::{Snake, FIELD_HEIGHT, FIELD_WIDTH};

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

fn main() {
    let mut population: Population<Snake> = Population::new(2000);

    let status_bar_height = 8;
    let mut engine = ConsoleEngine::init(
        (FIELD_WIDTH + 64).into(),
        (FIELD_HEIGHT + status_bar_height + 4).into(),
        u32::MAX,
    )
    .unwrap();

    loop {
        engine.wait_frame();
        engine.clear_screen();

        let shift = (1, status_bar_height as i32);

        draw_borders(&mut engine, shift);

        population.tick();

        if population.is_dead() {
            population.evolution();
        }

        for snake in &mut population.get_genomes().iter() {
            if !snake.is_alive() {
                continue;
            }

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

        engine.print(
            1,
            0,
            format!(
                "snakes_alive: {}",
                *population.alive_genomes_count.lock().unwrap(),
            )
            .as_str(),
        );

        engine.print(
            1,
            1,
            format!("generation: {}", population.generation,).as_str(),
        );
        engine.print(
            1,
            2,
            format!("mutation_rate: {}", population.mutation_rate).as_str(),
        );
        engine.print(
            1,
            3,
            format!(
                "max_fitness_current: {}",
                *population.max_fitness_current.lock().unwrap()
            )
            .as_str(),
        );
        engine.print(
            1,
            4,
            format!("max_fitness_prev: {}", population.max_fitness_prev).as_str(),
        );

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        if engine.is_key_pressed(KeyCode::Char('q')) {
            population.kill();
        }

        engine.draw();
    }
}
