use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum ChessPieceType
{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

// Map every piece as an u8 for a more
// efficient piece checking later on
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ChessState
{
    On = 0,
    Stalemate = 1,
    Checkmate = 2,
    Aborted = 3
}

#[derive(PartialEq, Clone, Copy)]
pub struct ChessPos
{
    x: u8,
    y: u8,
}

impl ChessPos
{
    pub fn from(pos: u8, ignore_bounds: bool) -> ChessPos
    {
        if !ignore_bounds
        {
            assert!(pos>63, "Position outside of bounds");
        }
        
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

    // This function is equivalent to `self ∈ (a,b)`
    pub fn between(&self, a: ChessPos, b: ChessPos, ignore_shape: bool) -> bool
    {
        let dx: u8 = i8::abs(a.x as i8 - b.x as i8) as u8;
        let dy: u8 = i8::abs(a.y as i8 - b.y as i8) as u8;

        //`a` and `b` must be placed either straight or diagonally
        let _diag: bool = dx==dy;
        let _straight: bool = dx==0 && dy>0 || dx>0 && dy==0;
        if !(_diag || _straight) && ignore_shape { return false; }
        assert!(_diag || _straight, "Elements `a` and `b` must be placed diagonally or straight");
    
        // Does not include the start and end points
        if *self==a || *self == b
        {
            return false;
        }

        let h1: i8 = i8::signum(self.x as i8 - a.x as i8);
        let h2: i8 = i8::signum(self.x as i8 - b.x as i8);

        let v1: i8 = i8::signum(self.y as i8 - a.y as i8);
        let v2: i8 = i8::signum(self.y as i8 - b.y as i8);
        if _straight
        {

            // If the block is valid horisontally or vertically
            let _in_h: bool = h1!=h2 && dy==0 && self.y==a.x;
            let _in_v: bool = v1!=v2 && dx==0 && self.x==a.x;

            return _in_h || _in_v;
        }

        // Testing if between `a` and `b` in a diagonal pathway
        let _rel_dx: i8 = self.x as i8 - a.x as i8;
        let _rel_dy: i8 = self.y as i8 - a.y as i8;
        let _in_diag: bool = h1!=h2 && v1!=v2 && _rel_dx == _rel_dy;

        return _in_diag;
    }
}

pub struct ChessBoard
{
    w_lock: bool,
    // Real men do 1d array and interpretate it as a 2d array
    board: [ChessPiece; 64],

    _w_king: u8,
    _b_king: u8,

    _default_promotion: ChessPieceType,

    // Current game state
    _state: ChessState,    
    // Having a count on the chess pieces without having to count
    // 64 elements every time
    _piece_count: HashMap<ChessPiece, u8>,
}

mod chess_logic;
#[cfg(test)]
mod pub_tests;