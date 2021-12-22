use options::ProgramOptions;
use game::{Player, TicTacToe};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;

pub mod options;
pub mod game;

fn get_color_from_cell(options: &ProgramOptions, cell: Option<Player>) -> Color {
    match cell {
        Some(Player::X) => options.x_color,
        Some(Player::O) => options.o_color,
        None => Color::RGB(255, 255, 255),
    }
}

// returns whether to quit
fn render_endscreen(color: Color, canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> bool {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                Event::MouseButtonDown { .. } => return false,
                _ => {},
            }
        }
    }
}

fn main() -> Result<(), String> {
    let options = ProgramOptions::get()?;
    let mut game = TicTacToe::new_instance(
        options.size,
        options.button_sep,
        options.button_size,
        if options.opponent_ai {
            Some(Player::O)
        } else {
            None
        });
    let screen_size = game.screen_size(options.scale);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(options.title.as_str(), screen_size.0, screen_size.1)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(options.scale, options.scale)?;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for (i, row) in game.buttons.iter().enumerate() {
            for (j, button) in row.iter().enumerate() {
                canvas.set_draw_color(get_color_from_cell(&options, game.grid[i][j]));
                canvas.draw_rect(*button).unwrap(); // UNSAFE
                canvas.fill_rect(*button).unwrap(); // UNSAFE
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    if let Some((i, j)) = game.button_pressed(x, y) {
                        match game.play(i, j) {
//fn render_endscreen(color: Color, canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> bool {
                            Some(Some(player)) => {
                                if render_endscreen(
                                    get_color_from_cell(&options, Some(player)),
                                    &mut canvas,
                                    &mut event_pump,
                                ) {
                                    break 'running;
                                } else {
                                    game.restart();
                                    continue 'running;
                                }
                            },
                            Some(None) => {
                                game.restart();
                                continue 'running;
                            },
                            None => continue 'running,
                        }
                    }
                },
                _ => {},
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
