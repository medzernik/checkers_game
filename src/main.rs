struct GameBoard {
    board: [[char; 8]; 8],
}

impl GameBoard {
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
                    | (2, 7) => 'w',
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
                    | (7, 6) => 'b',
                    _ => ' ',
                }
            }
        }
        return new_board;
    }
}

struct Game {
    board: GameBoard,
}

fn main() {
    let gameboard = GameBoard::create_default_board();
    gameboard.print_board();
}

fn game(gameboard: GameBoard) {
    let mut turn = 1;
    loop {
        
    }
}
