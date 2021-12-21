use sdl2::rect::Rect;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Player {
    O = 0,
    X = 1,
}

pub struct TicTacToe {
    pub whose_turn: Player,
    size: (u32, u32),
    pub grid: Vec<Vec<Option<Player>>>,
    pub buttons: Vec<Vec<Rect>>,
    button_sep: u32,
    button_size: u32,
    pub ai: Option<Player>,
}

impl TicTacToe {
    pub fn init(size: (u32, u32), button_sep: u32, button_size: u32, ai: Option<Player>) -> Self {
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
            ai,
        }
    }
    
    pub fn into_new(self) -> Self {
        Self::init(self.size, self.button_sep, self.button_size, self.ai)
    }

    pub fn screen_size(&self, scale: f32) -> (u32, u32) {
        (((self.size.0 * 210 + 10) as f32 * scale) as u32,
        ((self.size.1 * 210 + 10) as f32 * scale) as u32)
    }

    pub fn button_pressed(&self, x: i32, y: i32) -> Option<(usize, usize)> {
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

    pub fn get_optimal_move(&self) -> (u32, u32) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.is_none() {
                    println!("{}, {}", i, j);
                    return (i as u32, j as u32);
                }
            }
        }
        (0, 0)
    }

    pub fn play_turn(&mut self, i: u32, j: u32) -> bool {
        if i >= self.size.0 || j >= self.size.1 || matches!(self.grid[i as usize][j as usize], Some(_)) {
            false
        } else {
            self.grid[i as usize][j as usize] = Some(self.whose_turn);
            true
        }
    }

    pub fn switch_player(&mut self) {
        self.whose_turn = match self.whose_turn {
            Player::O => Player::X,
            Player::X => Player::O,
        };
    }

    pub fn player_won(&self) -> bool {
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

    pub fn is_full(&self) -> bool {
        for i in 0..self.size.0 as usize {
            for j in 0..self.size.1 as usize {
                if self.grid[i][j].is_none() {
                    return false;
                }
            }
        }
        true
    }
}


