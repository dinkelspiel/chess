#[derive(Copy, Clone)]
pub enum PieceColor {
    Black,
    White
}

#[derive(Copy, Clone)]
pub enum Piece {
    Empty,
    Pawn(PieceColor),
    Rook(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Queen(PieceColor),
    King(PieceColor)
}
