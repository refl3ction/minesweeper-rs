use std::{collections::HashSet, fmt::Display};

use crate::{minesweeper::Field, Position};

#[derive(Default, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub mines_count: usize,
    pub open_fields_pos: HashSet<Position>,
    pub flag_fields_pos: HashSet<Position>,
    pub mine_fields_pos: HashSet<Position>,
}

impl Board {
    // Move this method to a BoardFactory, change params to width, height and mines
    pub fn new(width: usize, height: usize, mines_count: usize) -> Self {
        Self {
            width,
            height,
            mines_count,
            open_fields_pos: HashSet::new(),
            flag_fields_pos: HashSet::new(),
            mine_fields_pos: HashSet::new(),
        }
    }

    pub fn add_mine(&mut self, pos: Position) {
        self.mine_fields_pos.insert(pos);
    }

    pub fn add_open(&mut self, pos: Position) {
        self.open_fields_pos.insert(pos);
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.flag_fields_pos.contains(&pos) {
            self.flag_fields_pos.remove(&pos);
        } else {
            self.flag_fields_pos.insert(pos);
        }
    }

    pub fn is_mine(&self, pos: Position) -> bool {
        self.mine_fields_pos.contains(&pos)
    }

    pub fn is_open(&self, pos: Position) -> bool {
        self.open_fields_pos.contains(&pos)
    }

    pub fn is_flag(&self, pos: Position) -> bool {
        self.flag_fields_pos.contains(&pos)
    }

    pub fn get_neighbors(&self, (x, y): Position) -> Vec<Position> {
        let min_x = x.max(1) - 1;
        let max_x = (x + 1).min(self.width - 1);
        let min_y = y.max(1) - 1;
        let max_y = (y + 1).min(self.height - 1);

        let mut positions: Vec<Position> = vec![];
        for i in min_x..=max_x {
            for j in min_y..=max_y {
                // Ignore self position
                if x != i || y != j {
                    positions.push((i, j));
                }
            }
        }
        positions
    }

    pub fn count_neighboring_mines(&self, pos: Position) -> i32 {
        let neighbors = self.get_neighbors(pos);
        let mut mines_count = 0;
        for neighbor in neighbors {
            if self.mine_fields_pos.contains(&neighbor) {
                mines_count += 1
            }
        }
        mines_count
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.mine_fields_pos.contains(&(x, y)) {
                    f.write_str(" 💣 ")?
                } else if self.flag_fields_pos.contains(&(x, y)) {
                    f.write_str(" 🚩 ")?
                } else {
                    f.write_str(" 🟧 ")?
                }
            }
            f.write_str("\n\n")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, minesweeper::*, Position};

    // #[test]
    // fn test() {
    //     let mut board = Board::new(9, 9, 10);
    //     board.generate_fields();

    //     board.toggle_flag((0, 0));
    //     board.toggle_flag((0, 1));
    //     board.open_field((1, 1));
    //     board.open_field((2, 2));
    //     board.open_field((3, 3));
    //     println!("{}", board);

    //     // let mut ms = Minesweeper::new(10, 10, 5);
    //     // ms.open((5, 5));
    //     // ms.toggle_flag((6, 6));
    //     // ms.open((6, 6));

    //     // println!("{}", ms);
    // }
    #[test]
    fn test_toggle_flag() {
        let mut board = Board::new(9, 9, 10);

        let pos = (0, 1);
        board.toggle_flag(pos);
        assert_eq!(board.is_flag(pos), true);
        board.toggle_flag(pos);
        assert_eq!(board.is_flag(pos), false);
    }

    #[test]
    fn test_count_neighboring_mines() {
        let mut board = Board::new(9, 9, 10);

        board.add_mine((0, 1));
        board.add_mine((1, 0));
        board.add_mine((1, 1));
        let pos: Position = (0, 0);
        let count = board.count_neighboring_mines(pos);
        assert_eq!(count, 3);
    }
}
