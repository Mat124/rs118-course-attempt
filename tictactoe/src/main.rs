mod data_types;
use std::io::stdin;
use std::io::stdout;

fn main() {
    let mut turn = String::new();
    while !turn.eq("X") & !turn.eq("O") {
        //get first marker to go
        turn = String::from("");
        println!("Enter the marker to go first (X or O):");
        stdin().read_line(&mut turn);
        turn.pop();
    }

    let mut board = data_types::Board::new(match turn.as_str() {
        //swapped in loop so set it to the other
        "X" => data_types::Marker::O,
        "O" => data_types::Marker::X,
        _ => panic!("This is impossible!"),
    }); //create game board

    while data_types::Board::get_winner(&board) == data_types::Marker::None {
        //main game loop, go until winner exists
        if (board.turn == data_types::Marker::X) {
            board.turn = data_types::Marker::O;
        } else {
            board.turn = data_types::Marker::X;
        }
        println!("Enter player {}'s turn:", board.turn);

        let mut p1 = String::new(); //get index 1
        stdin().read_line(&mut p1);
        let p1_num = p1.trim();

        let mut p2 = String::new(); //get index 2
        stdin().read_line(&mut p2);
        let p2_num = p2.trim();

        match (p1_num.parse::<usize>(), p2_num.parse::<usize>()) {
            //parse and attempt to place
            (Ok(place1), Ok(place2)) => {
                data_types::Board::place(&mut board, (place1 - 1, place2 - 1))
            }
            _ => panic!(
                "bad placement inputs, should be index 1 then index 2 with values between 1 and 3"
            ),
        }

        data_types::Board::display(&board); //display board
    }

    println!("{} wins!", board.turn);
}
