use std::io;
use std::io::Read;

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
            println!();
        }
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
    fn move_piece(&mut self, from: i32, to: i32) {
        let from_row: i32 = from / 10;
        let from_col: i32 = from % 10;
        let to_row: i32 = to / 10;
        let to_col: i32 = to % 10;

        self.board[to_row as usize][to_col as usize] = self.board[from_row as usize][from_col as usize];
        self.board[from_row as usize][from_col as usize] = ' ';
        if (from_row - to_row).abs() == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            self.board[mid_row as usize][mid_col as usize] = ' ';
        }
    }
}

struct Game {
    board: GameBoard,
}

fn main() {
    let mut gameboard = GameBoard::create_default_board();
    gameboard.print_board();
    //let isWhiteTurn: bool = true;
    loop {
        let mut from = String::new();
        let mut to = String::new();
        println!("Enter the move: (Format: rowcol)");
        io::stdin()
            .read_line(&mut from)
            .expect("Failed to read line");
        io::stdin().read_line(&mut to).expect("Failed to read line");
        let from: i32 = from.trim().parse().expect("Please type a number!");
        let to: i32 = to.trim().parse().expect("Please type a number!");
        gameboard.move_piece(from, to);
        gameboard.print_board();
    }
}

fn game(gameboard: GameBoard) {
    let mut turn = 1;
    loop {
        
    }
}

fn get_user_input(display_text: &str) -> String {
    println!("{}\n", display_text);

    let mut buffer: String = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid type");
    buffer.trim().to_string()
}
