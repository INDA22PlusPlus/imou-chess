use super::*;

#[test]
fn test_chessboard_drag()
{
    let mut board: ChessBoard = ChessBoard::init_position();
    board.drag(0o01, 0o22);
}