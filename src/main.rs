mod genetic;
mod snake;

use console_engine::{pixel, Color, ConsoleEngine, KeyCode};
use genetic::{organism::Organism, population::Population};
use snake::{Point, Snake, FIELD_HEIGHT, FIELD_WIDTH};

fn draw_borders(canvas: &mut ConsoleEngine, shift: Point) {
    let border_color = Color::DarkRed;
    let border_pixel = pixel::pxl_bg(' ', border_color);

    canvas.set_pxl(shift.x, shift.y, border_pixel);
    canvas.set_pxl((FIELD_WIDTH + 1) as i32 + shift.x, shift.y, border_pixel);
    canvas.set_pxl(
        (FIELD_WIDTH + 1) as i32 + shift.x,
        (FIELD_HEIGHT + 1) as i32 + shift.y,
        border_pixel,
    );
    canvas.set_pxl(shift.x, (FIELD_HEIGHT + 1) as i32 + shift.y, border_pixel);

    for x in 0..FIELD_WIDTH + 1 {
        canvas.set_pxl(x as i32 + shift.x, shift.y, border_pixel);
        canvas.set_pxl(
            x as i32 + shift.x,
            (FIELD_HEIGHT + 1) as i32 + shift.y,
            border_pixel,
        );
    }
    for y in 0..FIELD_HEIGHT + 1 {
        canvas.set_pxl(shift.x, y as i32 + shift.y, border_pixel);
        canvas.set_pxl(
            (FIELD_WIDTH + 1) as i32 + shift.x,
            y as i32 + shift.y,
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

        let shift = Point {
            x: 1,
            y: status_bar_height as i32,
        };

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
                snake.get_apple().x + 1 + shift.x,
                snake.get_apple().y + 1 + shift.y,
                pixel::pxl_bg(' ', Color::Red),
            );
            for cell in snake.get_cells() {
                engine.set_pxl(
                    cell.x + 1 + shift.x,
                    cell.y + 1 + shift.y,
                    pixel::pxl_bg(' ', Color::Green),
                );
            }
        }

        engine.print(
            1,
            0,
            format!("snakes_alive: {}", population.alive_genomes_count,).as_str(),
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
            format!("max_fitness_current: {}", population.max_fitness_current).as_str(),
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
