use sdl2::rect::Rect;
use rand::Rng;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Player {
    O = 0,
    X = 1,
}

impl Player {
    pub fn opposite(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub struct TicTacToe {
    whose_turn: Player,
    size: (u32, u32), // grid size
    pub grid: Vec<Vec<Option<Player>>>,
    pub buttons: Vec<Vec<Rect>>,
    button_sep: u32,
    button_size: u32,
    ai_opponent: Option<Player>,
}

impl TicTacToe {
    pub fn new_instance(
        size: (u32, u32),
        button_sep: u32,
        button_size: u32,
        ai_opponent: Option<Player>,
    ) -> Self {
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
            whose_turn: ai_opponent
                .and_then(|opponent| Some(opponent.opposite()))
                .unwrap_or(Player::O),
            size,
            grid,
            buttons,
            button_sep,
            button_size,
            ai_opponent,
        }
    }
    
    pub fn restart(&mut self) {
        *self = Self::new_instance(self.size, self.button_sep, self.button_size, self.ai_opponent);
    }

    pub fn screen_size(&self, scale: f32) -> (u32, u32) {
        (((self.size.0 * (self.button_size + self.button_sep) + self.button_sep) as f32 * scale) as u32,
        ((self.size.1 * (self.button_size + self.button_sep) + self.button_sep) as f32 * scale) as u32)
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

    fn optimal_move(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        loop {
            let i = rng.gen_range(0..self.size.0) as usize;
            let j = rng.gen_range(0..self.size.1) as usize;
            if self.grid[i][j].is_none() {
                return (i, j);
            }
        }
    }

    // returns Ok() if successful; Err(None) if out of bounds; Err(Some) if cell already occupied
    fn try_place(&mut self, i: usize, j: usize) -> Result<(), Option<Player>> {
        if i >= self.size.0 as usize ||
            j >= self.size.1 as usize {
            Err(None)
        } else if let Some(player) = self.grid[i][j] {
            Err(Some(player))
        } else {
            self.grid[i][j] = Some(self.whose_turn);
            Ok(())
        }
    }

    fn has_won(&self) -> bool {
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

    fn is_full(&self) -> bool {
        for i in 0..self.size.0 as usize {
            for j in 0..self.size.1 as usize {
                if self.grid[i][j] == None {
                    return false;
                }
            }
        }
        true
    }

    fn switch_player(&mut self) {
        self.whose_turn = match self.whose_turn {
            Player::O => Player::X,
            Player::X => Player::O,
        };
    }
    
    // return value:
    // - Some(Some(player)) -> show endscreen with winner being 'player'
    // - Some(None) -> restart game without endscreen
    // - None -> continue game normally
    pub fn play(&mut self, i: usize, j: usize) -> Option<Option<Player>> {
        if self.try_place(i, j).is_ok() {
            if self.has_won() {
                Some(Some(self.whose_turn))
            } else if self.is_full() {
                self.restart();
                Some(None)
            } else {
                self.switch_player();
                if Some(self.whose_turn) == self.ai_opponent {
                    let ai_move = self.optimal_move();
                    self.play(ai_move.0, ai_move.1)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

