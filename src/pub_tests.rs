use super::*;

#[test]
fn test_enum_chesspiece()
{
    // Testing `ChessPiece::is_white`
    assert!(!ChessPiece::Empty.is_white());

    assert!(!ChessPiece::BKing.is_white());
    assert!(!ChessPiece::BQueen.is_white());
    assert!(!ChessPiece::BBishop.is_white());
    assert!(!ChessPiece::BKnight.is_white());
    assert!(!ChessPiece::BRook.is_white());
    assert!(!ChessPiece::BPawn.is_white());

    assert!(ChessPiece::WKing.is_white());
    assert!(ChessPiece::WQueen.is_white());
    assert!(ChessPiece::WBishop.is_white());
    assert!(ChessPiece::WKnight.is_white());
    assert!(ChessPiece::WRook.is_white());
    assert!(ChessPiece::WPawn.is_white());


    // Testing `ChessPiece::is_black`
    assert!(!ChessPiece::Empty.is_black());

    assert!(ChessPiece::BKing.is_black());
    assert!(ChessPiece::BQueen.is_black());
    assert!(ChessPiece::BBishop.is_black());
    assert!(ChessPiece::BKnight.is_black());
    assert!(ChessPiece::BRook.is_black());
    assert!(ChessPiece::BPawn.is_black());

    assert!(!ChessPiece::WKing.is_black());
    assert!(!ChessPiece::WQueen.is_black());
    assert!(!ChessPiece::WBishop.is_black());
    assert!(!ChessPiece::WKnight.is_black());
    assert!(!ChessPiece::WRook.is_black());
    assert!(!ChessPiece::WPawn.is_black());


    // Testing `ChessPiece::is_empty`
    assert!(ChessPiece::Empty.is_empty());

    assert!(!ChessPiece::BKing.is_empty());
    assert!(!ChessPiece::BQueen.is_empty());
    assert!(!ChessPiece::BBishop.is_empty());
    assert!(!ChessPiece::BKnight.is_empty());
    assert!(!ChessPiece::BRook.is_empty());
    assert!(!ChessPiece::BPawn.is_empty());

    assert!(!ChessPiece::WKing.is_empty());
    assert!(!ChessPiece::WQueen.is_empty());
    assert!(!ChessPiece::WBishop.is_empty());
    assert!(!ChessPiece::WKnight.is_empty());
    assert!(!ChessPiece::WRook.is_empty());
    assert!(!ChessPiece::WPawn.is_empty());


    // Testing `ChessPiece::is_enemy_to`
    // testing on 1 black, 1 white and empty piece
    assert!(!ChessPiece::Empty.is_enemy_to(ChessPiece::BBishop));
    assert!(ChessPiece::Empty.is_enemy_to(ChessPiece::Empty));
    assert!(!ChessPiece::Empty.is_enemy_to(ChessPiece::WBishop));

    assert!(!ChessPiece::BBishop.is_enemy_to(ChessPiece::BBishop));
    assert!(ChessPiece::BBishop.is_enemy_to(ChessPiece::Empty));
    assert!(ChessPiece::BBishop.is_enemy_to(ChessPiece::WBishop));

    assert!(ChessPiece::WBishop.is_enemy_to(ChessPiece::BBishop));
    assert!(ChessPiece::WBishop.is_enemy_to(ChessPiece::Empty));
    assert!(!ChessPiece::WBishop.is_enemy_to(ChessPiece::WBishop));
}

#[test]
fn test_struct_chesspos_no_panic()
{
    // Initialize structs for testing
    let a1: ChessPos = ChessPos::from(0o00, false);
    let a2: ChessPos = ChessPos::from(0o63, false);
    let a3: ChessPos = ChessPos::from(0o40, false);
    let a4: ChessPos = ChessPos::from(0o37, false);
    let a5: ChessPos = ChessPos::from(0o77, false);

    // Testing constructor with ignore out of bounds flag on
    let a_out: ChessPos = ChessPos::from(0o104, true);

    // Testing `ChessPos::from`
    assert!(a1.x==0&&a1.y==0);
    assert!(a2.x==3&&a2.y==6);
    assert!(a3.x==0&&a3.y==4);
    assert!(a4.x==7&&a4.y==3);
    assert!(a5.x==7&&a5.y==7);
    assert!(a_out.x==4&&a_out.y==8);

    // Testing `ChessPos::raw`
    assert!(a1.raw() == 0o00);
    assert!(a2.raw() == 0o63);
    assert!(a3.raw() == 0o40);
    assert!(a4.raw() == 0o37);
    assert!(a5.raw() == 0o77);
    assert!(a_out.raw() == 0o104);

    // Testing `ChessPos::conv`
    assert!(ChessPos::conv(0,0)==0o00);
    assert!(ChessPos::conv(7,7)==0o77);
    assert!(ChessPos::conv(2,5)==0o52);
    assert!(ChessPos::conv(4,1)==0o14);
    assert!(ChessPos::conv(6,0)==0o06);

    // Testing `ChessPos::between`
    // Horisontal start and end points
    let h1: ChessPos = ChessPos::from(0o22  , false);
    let h2: ChessPos = ChessPos::from(0o25 , false);

    // Vertical start and end points
    let v1: ChessPos = ChessPos::from(0o13 , false);
    let v2: ChessPos = ChessPos::from(0o53 , false);

    // Diagonal start and end points
    let d1: ChessPos = ChessPos::from(0o23, false);
    let d2: ChessPos = ChessPos::from(0o56 , false);

    // Non diagonal or straight points
    let p1: ChessPos = ChessPos::from(0o23, false);
    let p2: ChessPos = ChessPos::from(0o76 , false);


    // Brutally testing `ChessPos::between` function on a horizontal line
    assert!(ChessPos::from(0o23, false).between(h1, h2, false));
    assert!(ChessPos::from(0o23, false).between(h1, h2, true));
    assert!(ChessPos::from(0o24, false).between(h1, h2, false));
    assert!(ChessPos::from(0o24, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o22, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o22, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o25, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o25, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o12, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o12, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o33, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o33, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o00, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o00, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o20, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o20, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o21, false).between(h1, h2, false));
    assert!(!ChessPos::from(0o21, false).between(h1, h2, true));
    assert!(!ChessPos::from(0o104, true).between(h1, h2, false));
    assert!(!ChessPos::from(0o104, true).between(h1, h2, true));

    assert!(ChessPos::from(0o23, false).between(h2, h1, false));
    assert!(ChessPos::from(0o23, false).between(h2, h1, true));
    assert!(ChessPos::from(0o24, false).between(h2, h1, false));
    assert!(ChessPos::from(0o24, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o22, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o22, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o25, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o25, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o12, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o12, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o33, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o33, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o00, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o00, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o20, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o20, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o21, false).between(h2, h1, false));
    assert!(!ChessPos::from(0o21, false).between(h2, h1, true));
    assert!(!ChessPos::from(0o104, true).between(h2, h1, false));
    assert!(!ChessPos::from(0o104, true).between(h2, h1, true));


    // Brutally testing `ChessPos::between` function on a vertical line
    assert!(ChessPos::from(0o23, false).between(v1, v2, false));
    assert!(ChessPos::from(0o23, false).between(v1, v2, true));
    assert!(ChessPos::from(0o43, false).between(v1, v2, false));
    assert!(ChessPos::from(0o43, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o13, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o13, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o53, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o53, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o00, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o00, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o77, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o77, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o10, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o10, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o17, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o17, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o63, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o63, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o03, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o03, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o14, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o14, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o12, false).between(v1, v2, false));
    assert!(!ChessPos::from(0o12, false).between(v1, v2, true));
    assert!(!ChessPos::from(0o104, true).between(v1, v2, true));
    assert!(!ChessPos::from(0o103, true).between(v1, v2, false));
    assert!(!ChessPos::from(0o103, true).between(v1, v2, true));

    assert!(ChessPos::from(0o23, false).between(v2, v1, false));
    assert!(ChessPos::from(0o23, false).between(v2, v1, true));
    assert!(ChessPos::from(0o43, false).between(v2, v1, false));
    assert!(ChessPos::from(0o43, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o13, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o13, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o53, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o53, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o00, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o00, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o77, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o77, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o10, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o10, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o17, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o17, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o63, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o63, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o03, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o03, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o14, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o14, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o12, false).between(v2, v1, false));
    assert!(!ChessPos::from(0o12, false).between(v2, v1, true));
    assert!(!ChessPos::from(0o104, true).between(v2, v1, true));
    assert!(!ChessPos::from(0o103, true).between(v2, v1, false));
    assert!(!ChessPos::from(0o103, true).between(v2, v1, true));


    // Brutally testing `ChessPos::between` function on a diagonal line
    assert!(ChessPos::from(0o34, false).between(d1, d2, false));
    assert!(ChessPos::from(0o34, false).between(d1, d2, true));
    assert!(ChessPos::from(0o45, false).between(d1, d2, false));
    assert!(ChessPos::from(0o45, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o23, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o23, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o13, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o13, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o56, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o56, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o12, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o12, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o67, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o67, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o35, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o35, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o44, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o44, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o00, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o00, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o77, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o77, false).between(d1, d2, true));
    assert!(!ChessPos::from(0o104, true).between(d1, d2, false));
    assert!(!ChessPos::from(0o104, true).between(d1, d2, true));
    assert!(!ChessPos::from(0o33, false).between(d1, d2, false));
    assert!(!ChessPos::from(0o33, false).between(d1, d2, true));

    assert!(ChessPos::from(0o34, false).between(d2, d1, false));
    assert!(ChessPos::from(0o34, false).between(d2, d1, true));
    assert!(ChessPos::from(0o45, false).between(d2, d1, false));
    assert!(ChessPos::from(0o45, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o23, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o23, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o13, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o13, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o56, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o56, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o12, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o12, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o67, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o67, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o35, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o35, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o44, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o44, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o00, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o00, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o77, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o77, false).between(d2, d1, true));
    assert!(!ChessPos::from(0o104, true).between(d2, d1, false));
    assert!(!ChessPos::from(0o104, true).between(d2, d1, true));
    assert!(!ChessPos::from(0o33, false).between(d2, d1, false));
    assert!(!ChessPos::from(0o33, false).between(d2, d1, true));


    assert!(!ChessPos::from(0o00, false).between(p1,p2, true));
    assert!(!ChessPos::from(0o00, false).between(p2,p1, true));

}

#[test]
#[should_panic]
fn test_struct_chesspos_from_no_ignore_panic()
{
    ChessPos::from(0o104, false);
}

#[test]
#[should_panic]
fn test_struct_chesspos_conv_panic()
{
    ChessPos::conv(10, 5);
}

#[test]
#[should_panic]
fn test_struct_chesspos_between_panic()
{
    // Non straight or diagonal start and end points
    let p1: ChessPos = ChessPos::from(0o23, false);
    let p2: ChessPos = ChessPos::from(0o76 , false);

    ChessPos::from(0o00, false).between(p1,p2,false);
}