#[derive(Debug, PartialEq, Clone, Copy)]
enum BoardStatus {
    EMPTY,
    X,
    O
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<BoardStatus>>,
}

impl Board {
    fn new() -> Board {
        return Board{
            board: vec![
                    vec![BoardStatus::EMPTY, BoardStatus::EMPTY, BoardStatus::EMPTY],
                    vec![BoardStatus::EMPTY, BoardStatus::EMPTY, BoardStatus::EMPTY],
                    vec![BoardStatus::EMPTY, BoardStatus::EMPTY, BoardStatus::EMPTY],
                ]
        };
    }

    fn next_player(turn_of: BoardStatus) -> BoardStatus {
        match turn_of {
            BoardStatus::X => BoardStatus::O,
            BoardStatus::O => BoardStatus::X,                        
            _ => BoardStatus::EMPTY,
        }
    }

    fn print_board(&self) -> (){
        for (_,row) in self.board.iter().enumerate() {
            for i in row {
                match i {
                    BoardStatus::X => print!(" X "),
                    BoardStatus::O => print!(" O "),
                    BoardStatus::EMPTY => print!(" - "),
                }
            }
            println!("\n");
        }
    }

    fn fill(&mut self, x: usize, y: usize, turn_of: BoardStatus) -> () {
        match self.board[x][y] {
            BoardStatus::EMPTY => {
                print_line();
                self.board[x][y] = turn_of;
                self.print_board();
            },
            _ => {
                println!("Already filled at that location.");
                let (x,y) = get_input();
                self.fill(x, y, turn_of);
            }
        }
    }

    fn is_over(&self) -> BoardStatus {

        // horizontal win
        for i in 0..3 {
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] && self.board[i][0] != BoardStatus::EMPTY {
                    return self.board[i][0];
            }
        }

        // vertical win
        for i in 0..3 {
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] && self.board[2][i] != BoardStatus::EMPTY {
                return self.board[0][i]
            }
        }

        // diagonal 1 win
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != BoardStatus::EMPTY {
                return self.board[0][0];
        } 

        // diagonal 2 win        
        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] && self.board[2][0] != BoardStatus::EMPTY {
                return self.board[2][0];
        } 
        
        return BoardStatus::EMPTY;
    }

    fn all_filled(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == BoardStatus::EMPTY {
                    return false;
                }
            }
        }
        return true;
    }

}

fn print_line() -> () {
    println!("*************************");
    println!("\n");
}

fn get_input() -> (usize, usize) {
    let mut input = String::new();
    print_line();
    println!("Enter the position (x,y): ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<u8> = input.split(",").map(|x| x.trim().parse().expect("Not an integer!")).collect();
    let x: usize = usize::from(inputs[0]);
    let y: usize = usize::from(inputs[1]);
    if x > 2 || y > 2 {
        println!("Input again. Supported values:> 0,1,2");
        get_input();
    }
    (x,y)
}

fn main() {
    let mut game_over: bool = false;
    let mut board: Board = Board::new();
    Board::print_board(&board);
    let mut next_player: BoardStatus = BoardStatus::X;
    while !game_over {
        next_player = Board::next_player(next_player);
        print_line();
        println!("Turn of: {:?}", next_player);
        let (x,y) = get_input();        
        Board::fill(&mut board, x, y, next_player);

        match Board::is_over(&board) {
            BoardStatus::EMPTY => game_over = false,
            BoardStatus::X => {
                game_over = true;
                println!("X wins!!");
            },
            BoardStatus::O => {
                game_over = true;
                println!("O wins!!");
            },
        }
        match Board::all_filled(&board) {
            true => {
                println!("It's a tie!");
                game_over = true;
            },
            false => {},
        }
    }
}
