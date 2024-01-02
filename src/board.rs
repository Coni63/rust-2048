use std::fmt;

use rand::seq::SliceRandom;

pub struct Board {
    board: [u8; 16],
    score: u32
}


impl Board {

    pub fn new() -> Board {
        Board {
            board: [0; 16],
            score: 0
        }
    }

    pub fn copy(&self) -> Board {
        Board {
            board: self.board,
            score: self.score
        }
    }

    // pub fn set_by_index(&mut self, idx: usize, value: u8) {
    //     self.board[idx] = value;
    // }

    // pub fn set(&mut self, x: u8, y: u8, value: u8) {
    //     self.board[(y * 4 + x) as usize] = value;
    // }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        self.board[(y * 4 + x) as usize]
    }

    pub fn apply_action(&mut self, action: String) -> bool {
        if action == "up" {
            return self.move_up();
        } else if action == "left" {
            return self.move_left();
        } else if action == "down" {
            return self.move_down();
        } else if action == "right" {
            return self.move_right();
        }
        return false;
    }

    pub fn is_game_over(&self) -> bool {
        // not valid if moved are possible, instead check all move directions
        let mut moved: bool;
        let mut copy: Board;
        for dir in ["up", "left", "down", "right"].iter() {
            copy = self.copy();
            moved = copy.apply_action(dir.to_string());
            if moved {
                return false;
            }
        }
        return true;
    }

    pub fn add_random_tile(&mut self) {
        let mut empty_tiles = Vec::new();
        for i in 0..16 {
            if self.board[i] == 0 {
                empty_tiles.push(i);
            }
        }

        let value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };

        let idx = empty_tiles.choose(&mut rand::thread_rng()).unwrap();
        self.board[*idx] = value;
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: usize;
        let mut ptr2: usize;
        for i in 0..4 {  // for each column
            ptr1 = i;
            ptr2 = i + 4;

            while ptr2 < 16 {
                if self.board[ptr2] != 0 {
                    if self.board[ptr1] == 0 {  // move to empty tile
                        self.board[ptr1] = self.board[ptr2];
                        self.board[ptr2] = 0;
                        ptr2 += 4;
                        moved = true;
                    } else if self.board[ptr1] == self.board[ptr2] {  // 2 same values -> merge
                        self.board[ptr1] *= 2;
                        self.board[ptr2] = 0;
                        ptr1 += 4;
                        ptr2 = ptr1 + 4;
                        moved = true;
                    } else {    // 2 different values
                        ptr1 += 4;
                        ptr2 = ptr1 + 4;
                    }
                }
                else {
                    ptr2 += 4;
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: usize;
        let mut ptr2: usize;

        for i in 0..4 {  // for each row
            ptr1 = 4 * i;
            ptr2 = 4 * i + 1;

            while ptr2 < 4 * (i + 1) {
                if self.board[ptr2] != 0 {
                    if self.board[ptr1] == 0 {  // move to empty tile
                        self.board[ptr1] = self.board[ptr2];
                        self.board[ptr2] = 0;
                        ptr2 += 1;
                        moved = true;
                    } else if self.board[ptr1] == self.board[ptr2] {  // 2 same values -> merge
                        self.board[ptr1] *= 2;
                        self.board[ptr2] = 0;
                        ptr1 += 1;
                        ptr2 = ptr1 + 1;
                        moved = true;
                    } else {    // 2 different values
                        ptr1 += 1;
                        ptr2 = ptr1 + 1;
                    }
                }
                else {
                    ptr2 += 1;
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: i32;
        let mut ptr2: i32;
        for i in 0..4 {  // for each column
            ptr1 = 12 + i; // bottom pointer
            ptr2 = 8 + i; // top pointer

            while ptr2 >= 0 {
                if self.board[ptr2 as usize] != 0 {
                    if self.board[ptr1 as usize] == 0 {  // move to empty tile
                        self.board[ptr1 as usize] = self.board[ptr2 as usize];
                        self.board[ptr2 as usize] = 0;
                        ptr2 -= 4;
                        moved = true;
                    } else if self.board[ptr1 as usize] == self.board[ptr2 as usize] {  // 2 same values -> merge
                        self.board[ptr1 as usize] *= 2;
                        self.board[ptr2 as usize] = 0;
                        ptr1 -= 4;
                        ptr2 = ptr1 - 4;
                        moved = true;
                    } else {    // 2 different values
                        ptr1 -= 4;
                        ptr2 = ptr1 - 4;
                    }
                }
                else {
                    ptr2 -= 4;
                }
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: i32;
        let mut ptr2: i32;
        for i in 0..4 {  // for each row
            ptr1 = 4 * (i + 1) - 1; // right pointer
            ptr2 = 4 * (i + 1) - 2; // left pointer

            while ptr2 >= 4 * i {
                if self.board[ptr2 as usize] != 0 {
                    if self.board[ptr1 as usize] == 0 {  // move to empty tile
                        self.board[ptr1 as usize] = self.board[ptr2 as usize];
                        self.board[ptr2 as usize] = 0;
                        ptr2 -= 1;
                        moved = true;
                    } else if self.board[ptr1 as usize] == self.board[ptr2 as usize] {  // 2 same values -> merge
                        self.board[ptr1 as usize] *= 2;
                        self.board[ptr2 as usize] = 0;
                        ptr1 -= 1;
                        ptr2 = ptr1 - 1;
                        moved = true;
                    } else {    // 2 different values
                        ptr1 -= 1;
                        ptr2 = ptr1 - 1;
                    }
                }
                else {
                    ptr2 -= 1;
                }
            }
        }
        moved
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        for y in 0..4 {
            for x in 0..4 {
                write!(f, "{:2} ", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        for y in 0..4 {
            for x in 0..4 {
                write!(f, "{:2} ", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_move_up() {
        let board_start =   [0, 4, 0, 2,
                                        0, 0, 4, 2,
                                        2, 4, 2, 2,
                                        2, 2, 8, 2];
        let board_end =     [4, 8, 4, 4,
                                        0, 2, 2, 4,
                                        0, 0, 8, 0,
                                        0, 0, 0, 0];
        
        let mut game = Board::new();
        game.board = board_start;

        let moved = game.move_up();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_up() {
        let board_end =     [4, 8, 4, 4,
                                        0, 2, 2, 0,
                                        0, 0, 8, 0,
                                        0, 0, 0, 0];
        
        let mut game = Board::new();
        game.board = board_end;

        let moved = game.move_up();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }

    #[test]
    fn test_move_left() {
        let board_start =   [2, 2, 2, 2,
                                        2, 0, 4, 2,
                                        0, 4, 8, 2,
                                        0, 4, 0, 2];
        let board_end =     [4, 4, 0, 0,
                                        2, 4, 2, 0,
                                        4, 8, 2, 0,
                                        4, 2, 0, 0];
        
        let mut game = Board::new();
        game.board = board_start;

        let moved = game.move_left();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_left() {
        let board_end =     [4, 0, 0, 0,
                                        2, 4, 2, 0,
                                        4, 8, 2, 0,
                                        4, 2, 0, 0];
        
        let mut game = Board::new();
        game.board = board_end;

        let moved = game.move_left();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }
    
    #[test]
    fn test_move_down() {
        let board_start =   [2, 2, 4, 2,
                                        2, 0, 4, 2,
                                        0, 4, 8, 2,
                                        0, 4, 0, 2];
        let board_end =     [0, 0, 0, 0,
                                        0, 0, 0, 0,
                                        0, 2, 8, 4,
                                        4, 8, 8, 4];
        
        let mut game = Board::new();
        game.board = board_start;

        let moved = game.move_down();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_down() {
        let board_end =     [0, 0, 0, 0,
                                        0, 0, 0, 0,
                                        0, 2, 0, 0,
                                        4, 8, 8, 4];
        
        let mut game = Board::new();
        game.board = board_end;

        let moved = game.move_down();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }

    #[test]
    fn test_move_right() {
        let board_start =   [2, 2, 2, 2,
                                        2, 0, 4, 2,
                                        2, 4, 8, 0,
                                        2, 2, 4, 0];
        let board_end =     [0, 0, 4, 4,
                                        0, 2, 4, 2,
                                        0, 2, 4, 8,
                                        0, 0, 4, 4];
        
        let mut game = Board::new();
        game.board = board_start;

        let moved = game.move_right();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_right() {
        let board_end =     [0, 0, 0, 4,
                                        0, 0, 8, 2,
                                        2, 4, 2, 4,
                                        0, 0, 4, 8];
        
        let mut game = Board::new();
        game.board = board_end;

        let moved = game.move_right();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }
}