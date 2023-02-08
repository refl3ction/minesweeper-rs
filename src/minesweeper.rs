use crate::{board::*, Position};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Minesweeper {
    pub can_exit: bool,
    pub is_exiting: bool,
    pub game_difficult: GameDifficult,
    pub board: Board,
    pub show_board: bool,
}

pub enum GameDifficult {
    BEGINNER,
    INTERMEDIATE,
    ADVANCED,
}

pub enum Field {
    Mine,
    NotMine(usize),
}

impl Default for GameDifficult {
    fn default() -> GameDifficult {
        GameDifficult::BEGINNER
    }
}

impl Minesweeper {
    fn create_board(&mut self) {
        match self.game_difficult {
            GameDifficult::BEGINNER => {
                self.board = Board::new(9, 9, 10);
            }
            _ => todo!(),
        }
    }

    pub fn populate_mines(&mut self) {
        let mut rng = thread_rng();
        let min = 0;
        let max = self.board.mines_count;

        while self.board.mine_fields_pos.len() != self.board.mines_count {
            let x = rng.gen_range(min..max);
            let y = rng.gen_range(min..max);
            self.board.add_mine((x, y));
        }
    }

    pub fn open_field(&mut self, pos: Position) -> Field {
        if self.board.is_flag(pos) {
            println!("is flag, do nothing")
            // return None
        }
        if self.board.is_mine(pos) {
            // game is lost
            println!("Found mine {:?}", pos);
            Field::Mine
            // show all mines
        } else {
            let neighbor_fields = self.board.get_neighbors(pos);
            let mines_count = neighbor_fields
                .into_iter()
                .filter(|pos| self.board.is_mine(pos.to_owned()))
                .count();
            println!("Total mines around {:?}: {}", pos, mines_count);
            // If mines_count = 0, call recursive get_neighbords
            // if mines_count == 0 {
            //     for neighbor in neighbor_fields {}
            // }
            self.board.add_open(pos);
            Field::NotMine(mines_count)
        }
    }
}
