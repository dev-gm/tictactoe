use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::time::Duration;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Player {
    O = 0,
    X = 1,
}

impl Player {
    fn color(player: Option<Self>) -> Color {
        match player {
            Some(Self::O) => Color::RGB(50, 200, 150),
            Some(Self::X) => Color::RGB(255, 50, 50),
            None => Color::RGB(255, 255, 255),
        }
    }
}

struct TicTacToe {
    whose_turn: Player,
    size: (u32, u32),
    grid: Vec<Vec<Option<Player>>>,
    buttons: Vec<Vec<Rect>>,
    button_sep: u32,
    button_size: u32,
}

impl TicTacToe {
    fn init(size: (u32, u32), button_sep: u32, button_size: u32) -> Self {
        let mut grid = Vec::with_capacity(size.0 as usize);
        let mut buttons = Vec::with_capacity(size.0 as usize);
        for i in 0..size.0 {
            grid.push(Vec::new());
            buttons.push(Vec::new());
            for j in 0..size.1 {
                grid[i as usize].push(None);
                buttons[i as usize].push(Rect::new(
                    (button_size * j + button_sep * (j + 1)) as i32,
                    (button_size * i + button_sep * (i + 1)) as i32,
                    button_size, button_size,
                ));
            }
        }
        Self {
            whose_turn: Player::O,
            size,
            grid,
            buttons,
            button_sep,
            button_size,
        }
    }
    
    fn from_old(old: Self) -> Self {
        Self::init(old.size, old.button_sep, old.button_size)
    }

    fn screen_size(&self) -> (u32, u32) {
        (self.size.0 * 210 + 10, self.size.1 * 210 + 10)
    }

    fn button_pressed(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        for (i, row) in self.buttons.iter().enumerate() {
            for (j, button) in row.iter().enumerate() {
                if x >= button.x() &&
                    y >= button.y() &&
                    x <= button.x() + button.width() as i32 &&
                    y <= button.y() + button.height() as i32 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn play_turn(&mut self, i: u32, j: u32) -> bool {
        if i >= self.size.0 || j >= self.size.1 || matches!(self.grid[i as usize][j as usize], Some(_)) {
            false
        } else {
            self.grid[i as usize][j as usize] = Some(self.whose_turn);
            true
        }
    }

    fn switch_player(&mut self) {
        self.whose_turn = match self.whose_turn {
            Player::O => Player::X,
            Player::X => Player::O,
        };
    }

    fn player_won(&self) -> bool {
        let mut line_perfect = [true; 4];
        for i in 0..self.size.0 {
            line_perfect[0] = true;
            line_perfect[1] = true;
            for j in 0..self.size.1 {
                for (k, (r, c)) in [(i, j), (j, i)].into_iter().enumerate() {
                    if line_perfect[k] && self.grid[r as usize][c as usize] != Some(self.whose_turn) {
                        line_perfect[k] = false;
                    }
                }
            }
            if line_perfect[0] || line_perfect[1] {
                return true;
            }
            for (k, j) in [i, self.size.1-1-i].into_iter().enumerate() {
                if line_perfect[k+2] && self.grid[i as usize][j as usize] != Some(self.whose_turn) {
                    line_perfect[k+2] = false;
                }
            }
        }
        line_perfect[2] || line_perfect[3]
    }
}

fn main() -> Result<(), ()> {
    let mut game = TicTacToe::init((4, 4), 10, 200);
    let screen_size = game.screen_size();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("TicTacToe", screen_size.0, screen_size.1)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(50, 200, 150));
        for (i, row) in game.buttons.iter().enumerate() {
            for (j, button) in row.iter().enumerate() {
                canvas.set_draw_color(Player::color(game.grid[i][j]));
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
                                canvas.set_draw_color(Player::color(game.grid[i][j]));
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
