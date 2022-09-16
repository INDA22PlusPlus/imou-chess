use super::*;


impl ChessBoard
{
    // Initialize a chess board with pieces on default positions
    pub fn init_position() -> ChessBoard
    {
        // White start by default
        let w_lock = false;

        let mut _piece_count: HashMap<ChessPiece, u8> = HashMap::new();

        let mut board: [ChessPiece; 64] = [ChessPiece::Empty; 64];
        {
            // Register the initial number of pieces on the board
            _piece_count.insert(ChessPiece::BRook, 2);
            _piece_count.insert(ChessPiece::WRook, 2);
            _piece_count.insert(ChessPiece::BKnight, 2);
            _piece_count.insert(ChessPiece::WKnight, 2);
            _piece_count.insert(ChessPiece::BBishop, 2);
            _piece_count.insert(ChessPiece::WBishop, 2);
            _piece_count.insert(ChessPiece::BPawn, 8);
            _piece_count.insert(ChessPiece::WPawn, 8);
            _piece_count.insert(ChessPiece::BQueen, 1);
            _piece_count.insert(ChessPiece::BQueen, 1);
            _piece_count.insert(ChessPiece::BKing, 1);
            _piece_count.insert(ChessPiece::BKing, 1);

            // Place the chess pieces in the default order
            board[0o00] = ChessPiece::BRook;
            board[0o01] = ChessPiece::BKnight;
            board[0o02] = ChessPiece::BBishop;
            board[0o03] = ChessPiece::BQueen;
            board[0o04] = ChessPiece::BKing;
            board[0o05] = ChessPiece::BBishop;
            board[0o06] = ChessPiece::BKnight;
            board[0o07] = ChessPiece::BRook;
            board[0o10] = ChessPiece::BPawn;
            board[0o11] = ChessPiece::BPawn;
            board[0o12] = ChessPiece::BPawn;
            board[0o13] = ChessPiece::BPawn;
            board[0o14] = ChessPiece::BPawn;
            board[0o15] = ChessPiece::BPawn;
            board[0o16] = ChessPiece::BPawn;
            board[0o17] = ChessPiece::BPawn;

            board[0o60] = ChessPiece::WPawn;
            board[0o61] = ChessPiece::WPawn;
            board[0o62] = ChessPiece::WPawn;
            board[0o63] = ChessPiece::WPawn;
            board[0o64] = ChessPiece::WPawn;
            board[0o65] = ChessPiece::WPawn;
            board[0o66] = ChessPiece::WPawn;
            board[0o67] = ChessPiece::WPawn;
            board[0o70] = ChessPiece::WRook;
            board[0o71] = ChessPiece::WKnight;
            board[0o72] = ChessPiece::WBishop;
            board[0o73] = ChessPiece::WQueen;
            board[0o74] = ChessPiece::WKing;
            board[0o75] = ChessPiece::WBishop;
            board[0o76] = ChessPiece::WKnight;
            board[0o77] = ChessPiece::WRook;
        }

        ChessBoard { w_lock: w_lock, board: board, 
            _state: ChessState::On, _piece_count: _piece_count }

    }

    pub fn drag(&mut self, from: u8, to: u8)
    {
        // Checks if the move is allowed, takes in account if for example
        // a pawn can 'eat' a enemy piece by taking a straight step
        if !(self.__verify_move(from, to))
        {
            // Raise error that the move isn't allowed
            assert!(false, "Illegal move");
        }

        let to_el: ChessPiece = self.board[to as usize];
        let from_el: ChessPiece = self.board[from as usize];

        self.board[from as usize] = ChessPiece::Empty;
        self.board[to as usize] = from_el;

        // Kill the enemy 
        if to_el.is_enemy_to(from_el)
        {
            // A piece of this type was killed => -1 in the piece count map
            *self._piece_count.get_mut(&to_el).unwrap() -= 1;
            
        }
        // Lock/unlock the move for whites
        self.w_lock = !self.w_lock;
    }

    // Changges 
    fn __update_state(&mut self)
    {
        
    }

    // Checks if the given pathway is empty.
    // `include_enemy` indicates if there's an enemy piece on
    // `to` coordinates and if it should be treated as an empty
    // position (can be 'eaten') or not.
    fn __empty_pathway(&mut self, from: u8, to: u8,
        include_enemy: bool, path: ChessPathway) -> bool
    {
        let f: ChessPos = ChessPos::from(from as u8);
        let t: ChessPos = ChessPos::from(to as u8);

        // I have no energy and time in researching on how custom exceptions
        // are created in Rust
        assert!(from != to, "Given pathway: from = to");

        let __r_dx: i8 = (f.x - t.x) as i8;
        let __r_dy: i8 = (f.y - t.y) as i8;

        // |__r_dx|, |__r_dy|
        let dx: i8 = i8::abs( __r_dx );
        let dy: i8 = i8::abs( __r_dy );

        // 1 or -1
        let dx_sign: i8 = i8::signum(__r_dx);
        let dy_sign: i8 = i8::signum(__r_dy);

        //////////////////////////////////////////////////////////////////
        let _el_to: ChessPiece = self.board[to as usize];
        let _el_from: ChessPiece = self.board[from as usize];
        // Check if the `to` position is empty when 'eating'
        // an enemy piece is not allowed
        let __case1: bool = _el_to.is_empty() && !include_enemy;
        
        // Check if the `to` position is empty or an enemy
        // piece is there, when 'eating' an enemy piece is allowed.
        let __case2: bool = _el_from.is_enemy_to(_el_to) && include_enemy;

        // Indicates if the last element is valid given the arguments
        let valid_le: bool = __case1 || __case2;


        // Check if the given `ChessPathway` is right
        match path
        {
            ChessPathway::Diagonal => {
                assert!(dx==dy, "Given pathway is not diagonal");

                for i in 1..(dx-1)
                {
                    let __check_x: u8 = ((f.x as i8) + i*dx_sign) as u8;
                    let __check_y: u8 = ((f.y as i8) + i*dy_sign) as u8;
                    let __coords: u8 = ChessPos::conv(__check_x, __check_y);

                    let _el: ChessPiece = self.board[__coords as usize];
                    if !_el.is_empty()
                    {
                        return false;
                    }
                }
            },

            ChessPathway::Straight => {
                let v_case: bool = (dx == 0) && (dy > 0);
                let h_case: bool = (dx > 0) && (dy == 0);
                assert!(v_case || h_case, "Given pathway is not straight");

                // If diagonal, either dx or dy is 0
                for i in 1..i8::max(dy-1, dx-1)
                {
                    // As you see I love branchless programming
                    let __x: u8 = ((f.x as i8)+i*dx_sign*(h_case as i8)) as u8;
                    let __y: u8 = ((f.y as i8)+i*dy_sign*(v_case as i8)) as u8;
                    let _el: ChessPiece = self.board[ChessPos::conv(__x, __y) as usize];

                    if !_el.is_empty()
                    {
                        return false;
                    }
                }
            },
            ChessPathway::Gamma => {
                let gamma_check: bool = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
                assert!(gamma_check, "Given pathway is not of type gamma");
            }
        }
        return valid_le;
    }

    // Verifies if the move is allowed 
    fn __verify_move(&mut self, from: u8, to: u8) -> bool
    {
        // Cannot move a piece to the same location
        assert!(from != to, "Given pathway: from = to");

        // `ChessPos` constructor automatically checks if position
        // is inside the chess board or not. If not, it asserts an
        // error.
        let f: ChessPos = ChessPos::from(from as u8);
        let t: ChessPos = ChessPos::from(to as u8);

        let _abs_dx: u8     = i8::abs( (f.x as i8) - (t.x as i8) ) as u8;
        let _abs_dy: u8     = i8::abs( (f.y as i8) - (t.y as i8) ) as u8;

        let from_el: ChessPiece = self.board[from as usize];

    
        let _wrong_color: bool = from_el.is_black() == self.w_lock;
        assert!(_wrong_color, "Illegal move, wrong color");

        let to_el: ChessPiece = self.board[to as usize];

        match from_el {
            // Raise error when trying to move an empty piece
            ChessPiece::Empty => {
                assert!(false, "Illegal move");
                return false;
            },
            ChessPiece::BPawn => {
                let _is_1down: bool = f.x==t.x && t.y-f.y==1;
                let _is_2down: bool = f.x==t.x && t.y-f.y==2;
                let _is_ldiag: bool = f.x-t.x==1 && t.y-f.y==1;
                let _is_rdiag: bool = t.x-f.x==1 && t.y-f.y==1;

                // If the given move is not in the list of the allowed onces..
                assert!(_is_1down || _is_2down || _is_ldiag || _is_rdiag,
                    "Illegal move for a black pawn");

                // Are the moves allowed?
                let _v_1down: bool  = _is_1down && to_el.is_empty();
                let _v_2down: bool  = _is_2down && self.__empty_pathway(from, to, 
                    false,ChessPathway::Straight) && f.y==1; 
                let _v_ldiag: bool  = _is_ldiag && to_el.is_enemy_to(from_el);
                let _v_rdiag: bool  = _is_rdiag && to_el.is_enemy_to(from_el);

                return _v_1down || _v_2down || _v_ldiag || _v_rdiag;
            },
            ChessPiece::WPawn => {
                let _is_1up: bool   = f.x==t.x && f.y-t.y==1;
                let _is_2up: bool   = f.x==t.x && f.y-t.y==2;
                let _is_ldiag: bool = f.x-t.x==1 && f.y-t.y==1;
                let _is_rdiag: bool = t.x-f.x==1 && f.y-t.y==1;

                assert!(_is_1up || _is_2up || _is_ldiag || _is_rdiag,
                    "Illegal move for a white pawn");
                
                let _v_1up: bool    = _is_1up && to_el.is_empty();
                let _v_2up: bool    = _is_2up && self.__empty_pathway(from, to, 
                    false,ChessPathway::Straight) && f.y==6;
                let _v_ldiag: bool  = _is_ldiag && to_el.is_enemy_to(from_el);
                let _v_rdiag: bool  = _is_rdiag && to_el.is_enemy_to(from_el);
                
                return _v_1up || _v_2up || _v_ldiag || _v_rdiag;
            },
            ChessPiece::BRook | ChessPiece::WRook => {
                // Check if the move is either a horisontal or vertical drag
                let _is_h: bool     = _abs_dx > 0 && _abs_dy == 0;
                let _is_v: bool     = _abs_dx == 0 && _abs_dy > 0;

                // If not either a horisontal or vertical drag, raise an error
                assert!(_is_h || _is_v, "Illegal move for a rook");

                let _v_h: bool      = _is_h && self.__empty_pathway(from, to, 
                    true,ChessPathway::Straight);
                let _v_v: bool      = _is_v && self.__empty_pathway(from, to, 
                    true,ChessPathway::Straight);

                return _v_h || _v_v;
            },
            ChessPiece::BBishop | ChessPiece::WBishop => {
                let _abs_dx: u8     = i8::abs( (f.x as i8) - (t.x as i8) ) as u8;
                let _abs_dy: u8     = i8::abs( (f.y as i8) - (t.y as i8) ) as u8;

                // If the move is not diagonal, raise an error
                assert!(_abs_dx==_abs_dy, "Illegal move for a bishop");

                return self.__empty_pathway(from, to, true, 
                    ChessPathway::Diagonal);
            },
            ChessPiece::BKnight | ChessPiece::WKnight => {
                let _is_hgamma: bool= _abs_dx==2 && _abs_dy==1;
                let _is_vgamma: bool= _abs_dx==1 && _abs_dy==2;

                assert!(_is_hgamma || _is_vgamma, "Illegal move for a knight");
                
                // `ChessBoard::__empty_pathway` in this case is just going to look
                // if `to_el` is an enemy or not, but anyways.. I just follow the
                // same structure..

                return self.__empty_pathway(from, to, true, 
                    ChessPathway::Gamma);
            },
            ChessPiece::BKing | ChessPiece::WKing => {
                // to==from is already checked in the begining of this method
                assert!(_abs_dx < 2 && _abs_dy < 2, "Illegal move for a king");
                return to_el.is_enemy_to(from_el);
            },
            ChessPiece::BQueen | ChessPiece::WQueen => {
                let _is_diag: bool  = _abs_dx==_abs_dy;
                let _is_h: bool     = _abs_dx>0 && _abs_dy==0;
                let _is_v: bool     = _abs_dx==0 && _abs_dy>0;

                assert!(_is_diag || _is_h || _is_v, "Illegal move for a queen");
                
                // Couldn't escape an if statement ;(
                if _is_diag
                {
                    // Checks if the pathway is empty diagonally
                    return self.__empty_pathway(from, to, true, 
                        ChessPathway::Diagonal);
                }
                // Checks if the pathway is empty vertically and horisontally
                return self.__empty_pathway(from, to, true, 
                    ChessPathway::Straight);
            }
        }
    }

}