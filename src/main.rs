use options::ProgramOptions;
use game::{Player, TicTacToe};
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

pub mod options;
pub mod game;

fn main() -> Result<(), String> {
    let options = ProgramOptions::get()?;
    let mut game = TicTacToe::init(options.size, options.button_sep, options.button_size);
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
                canvas.set_draw_color(match game.grid[i][j] {
                    Some(Player::X) => options.x_color,
                    Some(Player::O) => options.o_color,
                    None => Color::RGB(255, 255, 255),
                });
                canvas.draw_rect(*button).unwrap(); // UNSAFE
                canvas.fill_rect(*button).unwrap(); // UNSAFE
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    if let Some((i, j)) = game.button_pressed(x, y) {
                        if game.play_turn(i as u32, j as u32) {
                            if game.player_won() {
                                canvas.set_draw_color(match game.grid[i][j] {
                                    Some(Player::X) => options.x_color,
                                    Some(Player::O) => options.o_color,
                                    None => Color::RGB(255, 255, 255),
                                });
                                canvas.clear();
                                canvas.present();
                                loop {
                                    for event in event_pump.poll_iter() {
                                        match event {
                                            Event::Quit { .. } => break 'running,
                                            Event::MouseButtonDown { .. } => {
                                                game = TicTacToe::from_old(game);
                                                continue 'running;
                                            },
                                            _ => {},
                                        }
                                    }
                                }
                            } else if game.is_full() {
                                game = TicTacToe::from_old(game);
                                continue 'running;
                            }
                            game.switch_player();
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
