use crate::types;

pub fn parse_fen(fen_string: &str) -> Vec<Vec<types::Piece>> {
    let array = fen_string.split("/").collect::<Vec<&str>>();
    let mut board: Vec<Vec<types::Piece>> = vec![];

    for x in 0..8 {
        board.push(vec![]);
        for _ in 0..8 {
            board[x].push(types::Piece::Empty);
        }
    }

    let mut line_offset: usize = 0;
    for line in array {
        let mut offset: usize = 0;
        for piece in line.chars() {
            let piece_color: types::PieceColor = if piece.is_ascii_uppercase() { types::PieceColor::White } else { types::PieceColor::Black };

            match piece.to_ascii_lowercase() {
                'p' => board[offset][line_offset] = types::Piece::Pawn(piece_color),
                'r' => board[offset][line_offset] = types::Piece::Rook(piece_color),
                'n' => board[offset][line_offset] = types::Piece::Knight(piece_color),
                'b' => board[offset][line_offset] = types::Piece::Bishop(piece_color),
                'q' => board[offset][line_offset] = types::Piece::Queen(piece_color),
                'k' => board[offset][line_offset] = types::Piece::King(piece_color),
                x => {
                    if !x.is_numeric() {
                        panic!("Invalid FEN string at: {}", x);
                    }

                    let skip: i32 = x.to_string().parse::<i32>().unwrap();


                    for _ in 0..skip {
                        board[offset][line_offset] = types::Piece::Empty;
                        offset += 1;
                    }
                }
            }

            if ['r', 'n', 'b', 'q', 'k', 'p'].contains(&piece.to_ascii_lowercase()) {
                offset += 1;
            }
        }

        line_offset += 1;
    }

    for x in 0..8 {
        for y in 0..8 {
            match &board[x][y] {
                types::Piece::Empty  => print!("( )"),
                types::Piece::Pawn(color) => print!("({})", match color {
                    types::PieceColor::White => "P",
                    types::PieceColor::Black => "p"
                }),                
                types::Piece::Rook(color) => print!("({})", match color {
                    types::PieceColor::White => "R",
                    types::PieceColor::Black => "r"
                }),                
                types::Piece::Knight(color) => print!("({})", match color {
                    types::PieceColor::White => "N",
                    types::PieceColor::Black => "n"
                }),                
                types::Piece::Bishop(color) => print!("({})", match color {
                    types::PieceColor::White => "B",
                    types::PieceColor::Black => "b"
                }),                
                types::Piece::Queen(color) => print!("({})", match color {
                    types::PieceColor::White => "Q",
                    types::PieceColor::Black => "q"
                }),
                types::Piece::King(color) => print!("({})", match color {
                    types::PieceColor::White => "K",
                    types::PieceColor::Black => "k"
                })
            }
        }
        println!();
    }

    board
}
