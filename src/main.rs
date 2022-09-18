use std::io;
use std::process;

/// Creates the struct for the game board
struct GameBoard {
    board: [[char; 8]; 8],
}

/// Implementing methods
impl GameBoard {
    /// Prints the entire board. Usage:
    /// ```
    /// let gameboard = GameBoard::create_default_board();
    /// board_variable.print_board();
    /// ```
    fn print_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ", self.board[i][j]);
            }
            print!("{}\n", i);
        }
        println!("0 1 2 3 4 5 6 7");
    }

    fn create_default_board() -> GameBoard {
        let mut new_board = GameBoard {
            board: [[' '; 8]; 8],
        };
        for i in 0..8 {
            for j in 0..8 {
                new_board.board[i][j] = match (i, j) {
                    (0, 1)
                    | (0, 3)
                    | (0, 5)
                    | (0, 7)
                    | (1, 0)
                    | (1, 2)
                    | (1, 4)
                    | (1, 6)
                    | (2, 1)
                    | (2, 3)
                    | (2, 5)
                    | (2, 7) => 'b',
                    (5, 0)
                    | (5, 2)
                    | (5, 4)
                    | (5, 6)
                    | (6, 1)
                    | (6, 3)
                    | (6, 5)
                    | (6, 7)
                    | (7, 0)
                    | (7, 2)
                    | (7, 4)
                    | (7, 6) => 'w',
                    _ => ' ',
                }
            }
        }
        return new_board;
    }

    /// Moves a piece from one point to the next. Uses the if_valid_move() function to check move validity.
    /// Usage:
    /// ```
    /// gameboard.move_piece(from, to);
    /// ```
    fn move_piece(&mut self, from: i32, to: i32, white_turn: bool) {
        let from_row: i32 = from / 10;
        let from_col: i32 = from % 10;
        let to_row: i32 = to / 10;
        let to_col: i32 = to % 10;

        if white_turn && to_row == 0{
            self.board[to_row as usize][to_col as usize] = 'W';
        }else if !white_turn && to_row == 7{
            self.board[to_row as usize][to_col as usize] = 'B';
        }else {
            self.board[to_row as usize][to_col as usize] = self.board[from_row as usize][from_col as usize];
        }
        self.board[from_row as usize][from_col as usize] = ' ';
        if (from_row - to_row).abs() == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            self.board[mid_row as usize][mid_col as usize] = ' ';
        }

    }

    /// Checks whether the move is valid
    ///
    /// # Arguments
    ///
    /// * `from`: i32 coordinates (e.g. '40')
    /// * `to`: i32 coordinates (e.g. '63')
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// let mut gameboard = GameBoard::create_default_board();
    /// let valid: bool = gameboard.is_valid_move(30,52);
    /// ```
    fn is_valid_move(&mut self, from: i32, to: i32, white_turn: bool) -> bool {
        let from_row: i32 = from / 10;
        let from_col: i32 = from % 10;
        let to_row: i32 = to / 10;
        let to_col: i32 = to % 10;
        if !(0..=7).contains(&to_row) || !(0..7).contains(&to_) {
            false
        }
        else {
            match self.board[from_row as usize][from_col as usize] {
                'b' => {
                    if !white_turn {
                        if (from_row - to_row).abs() == 1 && (from_col - to_col).abs() == 1 {
                            true
                        } else if (from_row - to_row).abs() == 2 && (from_col - to_col).abs() == 2 {
                            let mid_row = (from_row + to_row) / 2;
                            let mid_col = (from_col + to_col) / 2;

                            self.board[mid_row as usize][mid_col as usize] == 'w'
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                'B' => {
                    if !white_turn && (from_row - to_row).abs() == (to_col - from_col).abs(){
                        true
                    }else {
                        false
                    }
                }
                'w' => {
                    if white_turn {
                        if (from_row - to_row).abs() == 1 && (from_col - to_col).abs() == 1 {
                            true
                        } else if (from_row - to_row).abs() == 2 && (from_col - to_col).abs() == 2 {
                            let mid_row = (from_row + to_row) / 2;
                            let mid_col = (from_col + to_col) / 2;
                            self.board[mid_row as usize][mid_col as usize] == 'b'
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                'W' => {
                    if white_turn && (from_row - to_row).abs() == (to_col - from_col).abs(){
                        true
                    }else {
                        false
                    }
                }
                _ => false,
            }
        }
    }
}

struct Game {
    board: GameBoard,
}

impl Game {
    /// Checks the winner and if so, ends the program
    ///
    /// # Examples
    ///
    /// ```
    /// loop{
    ///     let mut gameboard = GameBoard::create_default_board();
    ///     let mut game: Game = Game { board: gameboard };
    ///     game.check_winner()
    /// }
    /// ```
    fn check_winner(&self) {}
}

fn main() {
    //initialize the variables
    let mut gameboard = GameBoard::create_default_board();
    //let mut game: Game = Game { board: gameboard };

    gameboard.print_board();
    let mut is_white_turn: bool = true;

    //main data loop
    loop {
        let mut from = String::new();
        let mut to = String::new();
        match is_white_turn {
            true => println!("White move"),
            false => println!("Black move"),
        }
        println!("Enter the move: (Format: rowcol)");

        //get user input
        io::stdin()
            .read_line(&mut from)
            .expect("Failed to read line");
        io::stdin().read_line(&mut to).expect("Failed to read line");

        //parse correctly into a number

        let from: i32 = match from.trim().parse() {
            Ok(T) => T,
            Err(E) => {
                println!("{}", E);
                continue;
            }
        };

        let to: i32 = match to.trim().parse() {
            Ok(T) => T,
            Err(E) => {
                println!("{}", E);
                continue;
            }
        };

        if gameboard.is_valid_move(from, to, is_white_turn) {
            gameboard.move_piece(from, to, is_white_turn);
            is_white_turn = !is_white_turn;
            gameboard.print_board();
        } else {
            println!("invalid move");
        }

        //check the winner
        //game.check_winner()
    }
}

// fn get_user_input(display_text: &str) -> String {
//     println!("{}\n", display_text);
//
//     let mut buffer: String = String::new();
//
//     std::io::stdin()
//         .read_line(&mut buffer)
//         .expect("Invalid type");
//     buffer.trim().to_string()
// }

#[cfg(test)]
mod tests {
    use crate::GameBoard;

    #[test]
    ///Tests the functionality of moving
    fn is_valid_move() {
        //Tests an invalid white move
        let mut gameboard = GameBoard::create_default_board();
        let valid: bool = gameboard.is_valid_move(50, 30, true);

        assert_eq!(valid, false);

        //Tests a valid white move
        let mut gameboard = GameBoard::create_default_board();
        let valid: bool = gameboard.is_valid_move(50, 41, true);

        assert_eq!(valid, true);

        //Tests a valid white move while a black turn (should be false)
        let mut gameboard = GameBoard::create_default_board();
        let valid: bool = gameboard.is_valid_move(50, 41, false);

        assert_eq!(valid, false);
    }
}
