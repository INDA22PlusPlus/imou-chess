use std::collections::HashMap;

// Map every piece as an u8 for a more
// efficient piece checking later on
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChessPiece
{
    Empty   =   0,
    BKing   =   1,
    BQueen  =   2,
    BRook   =   3,
    BBishop =   4,
    BKnight =   5,
    BPawn   =   6,

    WKing   =   7,
    WQueen  =   8,
    WRook   =   9,
    WBishop =   10,
    WKnight =   11,
    WPawn   =   12,
}

impl ChessPiece
{
    pub fn is_white(self) -> bool
    {
        return ((self as u8) > 6u8) && ((self as u8) < 13u8);
    }

    pub fn is_black(self) -> bool
    {
        return ((self as u8) > 0u8) && ((self as u8) < 7u8);
    }

    pub fn is_empty(self) -> bool
    {
        return self == ChessPiece::Empty;
    }

    pub fn is_enemy_to(self, target: ChessPiece) -> bool
    {
        let case1: bool = target.is_empty();
        let case2: bool = self.is_black() && target.is_white();
        let case3: bool = self.is_white() && target.is_black();

        return case1 || case2 || case3;
    }
}

#[repr(u8)]
pub enum ChessPathway
{
    Diagonal = 0,
    Straight = 1,
    Gamma = 2,
}

#[repr(u8)]
pub enum ChessState
{
    On = 0,
    Stalemate = 1,
    Checkmate = 2,
    Aborted = 3
}


pub struct ChessPos
{
    x: u8,
    y: u8,
}

impl ChessPos
{
    pub fn from(pos: u8) -> ChessPos
    {
        assert!(pos>63, "Position outside of bounds");
        ChessPos{x: pos % 8, y: pos/8}
    }

    pub fn raw(self) -> u8
    {
        return self.y*8+self.x;
    }

    pub fn conv(x: u8, y: u8) -> u8
    {
        assert!(x>7 || y>7, "Position outside of bounds");
        return y*8+x;
    }
}

pub struct ChessBoard
{
    w_lock: bool,
    // Real men do 1d array and interpretate it as a 2d array
    board: [ChessPiece; 64],

    // Current game state
    _state: ChessState,    
    // Having a count on the chess pieces without having to count
    // 64 elements every time
    _piece_count: HashMap<ChessPiece, u8>,
}

mod chess_logic;
#[cfg(test)]
mod global_tests;