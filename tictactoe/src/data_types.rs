use std::fmt::Display;

pub struct Board {
    squares: Vec<Vec<Marker>>,
    pub turn: Marker,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)] //the derive is copied from the solutions
pub enum Marker {
    X,
    O,
    None,
}

impl Board {
    pub fn new(first: Marker) -> Self {//create new board
        Self {
            squares: vec![vec![Marker::None, Marker::None, Marker::None], vec![Marker::None, Marker::None, Marker::None], vec![Marker::None, Marker::None, Marker::None]],
            turn: first,
        }
    }

    pub fn display(b: Self) {//display the board
        println!("-------------");
        for row in b.squares {
            for square in row {
                print!("| {square} ");
            }
            println!("|");
            println!("-------------");
        }
    }

    pub fn get_winner(b: Self) -> Marker {//find if theres a winner
        if (
            (b.squares[0][0].eq(&b.squares[0][1]) & b.squares[0][0].eq(&b.squares[0][2])) |
            (b.squares[1][0].eq(&b.squares[1][1]) & b.squares[1][0].eq(&b.squares[1][2])) |
            (b.squares[2][0].eq(&b.squares[2][1]) & b.squares[2][0].eq(&b.squares[2][2])) |
            (b.squares[0][0].eq(&b.squares[1][0]) & b.squares[0][0].eq(&b.squares[2][0])) |
            (b.squares[0][1].eq(&b.squares[1][1]) & b.squares[0][1].eq(&b.squares[2][1])) |
            (b.squares[0][2].eq(&b.squares[1][2]) & b.squares[0][2].eq(&b.squares[2][2])) |
            (b.squares[0][0].eq(&b.squares[1][1]) & b.squares[0][0].eq(&b.squares[2][2])) |
            (b.squares[0][2].eq(&b.squares[1][1]) & b.squares[0][2].eq(&b.squares[2][0]))
        ) {
            return b.turn;
        }
        Marker::None
    }

    pub fn place(mut b: Self, loc: (usize, usize)) {//attempt to place a marker at the given spot
        if !(b.squares[loc.0][loc.1].eq(&Marker::None)) {
            panic!("This square is already in use!"); //literally just throw an error lol
        }
        b.squares[loc.0][loc.1] = b.turn; //this may be inefficient use of vecs, idk
    }
}

impl Display for Marker { //copied almost directly from Joey's solution lol
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 
            match self {
                Marker::X => "X",
                Marker::O => "O",
                Marker::None => " ",
            }
        )
    }
}