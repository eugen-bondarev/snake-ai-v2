mod snake;

use console_engine::{pixel, Color, ConsoleEngine, KeyCode};
use dfdx::{
    prelude::{DeviceBuildExt, Linear, Module, Sigmoid},
    shapes::Rank1,
    tensor::{Cpu, Tensor, ZerosTensor},
};
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

fn main() {
    let res = 42;

    type NN = (
        (Linear<20, 8>, Sigmoid),
        (Linear<8, 8>, Sigmoid),
        (Linear<8, 4>, Sigmoid),
    );

    let dev: Cpu = Default::default();
    let mlp = dev.build_module::<NN, f32>();
    let x: Tensor<Rank1<20>, f32, Cpu> = dev.zeros();
    let y: Tensor<Rank1<4>, f32, Cpu> = mlp.forward(x);

    println!("{:?}", y.as_vec());
    return;
    let mut snakes: Vec<Snake> = vec![Snake::new()];
    let status_bar_height = 3;
    let mut engine = ConsoleEngine::init(
        (FIELD_WIDTH + 4).into(),
        (FIELD_HEIGHT + 4 + status_bar_height).into(),
        15,
    )
    .unwrap();

    loop {
        engine.wait_frame();
        engine.clear_screen();

        let shift = (0, status_bar_height as i32);

        draw_borders(&mut engine, shift);

        engine.print(1, 1, format!("Score: {}", snakes[0].get_score()).as_str());

        for snake in &mut snakes {
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
        }

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
