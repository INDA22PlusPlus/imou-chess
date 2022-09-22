use super::*;
use std::ops::Range;


#[cfg(test)]
mod tests;

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
            board[0o00] = ChessPiece::WRook;
            board[0o01] = ChessPiece::WKnight;
            board[0o02] = ChessPiece::WBishop;
            board[0o03] = ChessPiece::WQueen;
            board[0o04] = ChessPiece::WKing;
            board[0o05] = ChessPiece::WBishop;
            board[0o06] = ChessPiece::WKnight;
            board[0o07] = ChessPiece::WRook;
            board[0o10] = ChessPiece::WPawn;
            board[0o11] = ChessPiece::WPawn;
            board[0o12] = ChessPiece::WPawn;
            board[0o13] = ChessPiece::WPawn;
            board[0o14] = ChessPiece::WPawn;
            board[0o15] = ChessPiece::WPawn;
            board[0o16] = ChessPiece::WPawn;
            board[0o17] = ChessPiece::WPawn;

            board[0o60] = ChessPiece::BPawn;
            board[0o61] = ChessPiece::BPawn;
            board[0o62] = ChessPiece::BPawn;
            board[0o63] = ChessPiece::BPawn;
            board[0o64] = ChessPiece::BPawn;
            board[0o65] = ChessPiece::BPawn;
            board[0o66] = ChessPiece::BPawn;
            board[0o67] = ChessPiece::BPawn;
            board[0o70] = ChessPiece::BRook;
            board[0o71] = ChessPiece::BKnight;
            board[0o72] = ChessPiece::BBishop;
            board[0o73] = ChessPiece::BQueen;
            board[0o74] = ChessPiece::BKing;
            board[0o75] = ChessPiece::BBishop;
            board[0o76] = ChessPiece::BKnight;
            board[0o77] = ChessPiece::BRook;
        }

        ChessBoard { w_lock: w_lock, board: board, _w_king: 0o04, _b_king: 0o74,
            _state: ChessState::On, _piece_count: _piece_count }

    }

    // Main function that realizes the move of one chess piece at `from` coordinates
    // to `to` coordinate block. The function validates if the move to be made is valid
    // and moves the chess piece to the given coordinates if the move is indeed valid.
    // thereafter a lock is being set or unset for the white color indicating that it's
    // black's or white's turn next.
    pub fn drag(&mut self, from: u8, to: u8)
    {
        // If the game has ended do not allow the game to continue;
        assert!(self._state==ChessState::On);

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

        // Update the coordinates of the kings when moved
        self._w_king = if from_el==ChessPiece::WKing { to } else {self._w_king};
        self._b_king = if from_el==ChessPiece::BKing { to } else {self._b_king}; 

        // Check if the game has ended
        self.__check_state();

        // Lock/unlock the move for whites
        self.w_lock = !self.w_lock;
    }


    // Checks if the block (piece) is threatened.
    // `_wh` indicates if the piece color is white or not
    // really dirty code - sorry, didn't have so much energy to work with it
    fn __is_threatened(&self, coords_raw: u8, _wh: bool) -> u8
    {
        let coords: ChessPos = ChessPos::from(coords_raw);

        let _pawn: ChessPiece = if _wh {ChessPiece::WPawn} else {ChessPiece::BPawn};
        let _king: ChessPiece = if _wh {ChessPiece::WKing} else {ChessPiece::BKing};
        let _rook: ChessPiece = if _wh {ChessPiece::WRook} else {ChessPiece::BRook};
        let _bishop: ChessPiece = if _wh {ChessPiece::WBishop} else {ChessPiece::BBishop};
        let _knight: ChessPiece = if _wh {ChessPiece::WKnight} else {ChessPiece::BKnight};
        let _queen: ChessPiece = if _wh {ChessPiece::WQueen} else {ChessPiece::BQueen};

        let _straight_attack: Vec<ChessPiece> = vec![_rook, _queen];
        let _diagonal_attack: Vec<ChessPiece> = vec![_bishop, _queen];

        let r: Range<i8> = 0..8;

        // Check if vertical line above the block threatens the block
        for i in 1..=(7-coords.x)
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+i, coords.y);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return ChessPos::conv(coords.x+i, coords.y) }
            
            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if vertical line below the block threatens the block
        for i in 1..=coords.x
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-i, coords.y);
            let _el: ChessPiece = self.board[enemy_coords as usize];   
            if _el == _king && i == 1 { return enemy_coords; }

            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        /////////////////////////////////////////
        // Check if horisontal line before the block threatens the block
        for i in 1..=coords.y
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x, coords.y-i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }

            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if horisontal line after the block threatens the block
        for i in 1..=(7-coords.y)
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x, coords.y+i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }

            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        /////////////////////////////////////////////
        // Check if tl diagonal line threatens the block
        for i in 1..=u8::min(coords.x, 7-coords.y)
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-i, coords.y+i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }

            let _check_pawn: bool = _el==ChessPiece::BPawn && _wh && i == 1;
            if _check_pawn { return enemy_coords; }

            if !_diagonal_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if rb diagonal line threatens the block
        for i in 1..=u8::min(7-coords.x, coords.y)
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+i, coords.y-i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }

            let _check_pawn: bool = _el==ChessPiece::WPawn && !_wh && i == 1;
            if _check_pawn { return enemy_coords; }

            if !_diagonal_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if tr diagonal line threatens the block
        for i in 1..=u8::min(7-coords.x, 7-coords.y)
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+i, coords.y+i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }
            
            let _check_pawn: bool = _el==ChessPiece::BPawn && _wh && i == 1;
            if _check_pawn { return enemy_coords; }

            if !_diagonal_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if bl diagonal line threatens the block
        for i in 1..=u8::min(coords.x, coords.y)
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-i, coords.y-i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }

            let _check_pawn: bool = _el==ChessPiece::WPawn && !_wh && i == 1;
            if _check_pawn { return enemy_coords; }

            if !_diagonal_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }



        let _x: i8 = coords.x as i8;
        let _y: i8 = coords.y as i8;

        ////////////////////////////////////////////////
        // Check if knight threatens the block
        if r.contains(&(_x+2)) && r.contains(&(_y+1))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+2, coords.y+1);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        if r.contains(&(_x+2)) && r.contains(&(_y-1))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+2, coords.y-1);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        if r.contains(&(_x-2)) && r.contains(&(_y+1))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-2, coords.y+1);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        if r.contains(&(_x-2)) && r.contains(&(_y-1))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-2, coords.y-1);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        //////////////////////////////////////
        if r.contains(&(_x+1)) && r.contains(&(_y+2))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+1, coords.y+2);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        if r.contains(&(_x-1)) && r.contains(&(_y+2))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-1, coords.y+2);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        if r.contains(&(_x+1)) && r.contains(&(_y-2))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x+1, coords.y-2);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        if r.contains(&(_x-1)) && r.contains(&(_y-2))
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-1, coords.y-2);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el==_knight { return enemy_coords; }
        }

        // out of bounds if not threatened
        return 64;

    }

    // Checks if the king is stalemate or checkmate.
    // The target king is choosed based on `self.w_lock`
    fn __check_state(&mut self)
    {
        let k_coords_raw: u8 = if self.w_lock {self._b_king} else {self._w_king};
        let k_coords: ChessPos = ChessPos::from(k_coords_raw);
        let k_threat_raw: u8 = self.__is_threatened(k_coords_raw, !self.w_lock);

        let _king: ChessPiece = self.board[k_coords_raw as usize];

        let r: Range<i8> = 0..8;

        let mut no_moves_count: u8 = 0;
        // Loop through the king's all possible moves and check
        // if the king has at least 1 legal move to do
        for i in -1..=1
        {
            for j in -1..=1
            {
                let x: i8 = (k_coords.x as i8)+i;
                let y: i8 = (k_coords.y as i8)+j;
                // Checked box out of chess board bounds - impossible
                // for a king to move there
                if !r.contains(&x) || !r.contains(&y)
                {
                    no_moves_count += 1;
                    continue;
                }

                let __tmp: u8 = ChessPos::conv(x as u8, y as u8);
                let _el: ChessPiece = self.board[__tmp as usize];
                // If the king cannot kill the piece near it to make a move
                if !_king.is_enemy_to(_el) && !_el.is_empty()
                {
                    no_moves_count += 1;
                    continue;
                }

                // Or else after all tests, the block itself is being threatened
                let _el_threat: bool = self.__is_threatened(__tmp, 
                                        !self.w_lock) != 64;
                no_moves_count += _el_threat as u8;
            }
        }

        // If not in check and no legal moves - set game state to stalemate and exit
        if k_threat_raw == 64 && no_moves_count == 8
        {
            self._state = ChessState::Stalemate;
            return;
        }

        // Check whether it is possible to eliminate the threat to the king
        // if it is possible - just exit
        if self.__is_threatened(k_threat_raw, !self.w_lock) != 64
        {
            return;
        }


        let k_threat_coords: ChessPos = ChessPos::from(k_threat_raw);
        let dx: i8 = k_threat_coords.x as i8 - k_coords.x as i8;
        let dy: i8 = k_threat_coords.y as i8 - k_coords.y as i8;

        let _threat: ChessPiece = self.board[k_threat_raw as usize];

        // Attacks by these pieces cannot be blocked in any way - ignore en passant for king
        let ignored_pieces: Vec<ChessPiece> = vec![ChessPiece::WKnight, ChessPiece::BKnight,
                                                    ChessPiece::WPawn, ChessPiece::BPawn];

        // If the attack cannot be blocked - set the game state to checkmate and exit
        if ignored_pieces.contains(&_threat)
        {
            self._state = ChessState::Checkmate;
            return;
        }

        let mut can_block_threat: bool = false;
        for i in 0..64 as u8
        {
            // Yeah bad brute force - sorting out all pieces and checking if they
            // can block the attack on the king
            let _el: ChessPiece = self.board[i as usize];
            if !_el.is_enemy_to(_threat)
            {
                continue;
            }

            let _pos: ChessPos = ChessPos::from(i as u8);

            match _el
            {
                ChessPiece::Empty => {return;},
                ChessPiece::WPawn | ChessPiece::BPawn => {
                    let x: i8 = _pos.x as i8;
                    let mut y: i8 = _pos.y as i8;
                    y += if _el.is_black() { -1 } else { 1 }; 

                    if !r.contains(&x)
                    {
                        return;
                    }

                    // No move validation is needed because of one step
                    let _coords: ChessPos = ChessPos::from(ChessPos::conv(x as u8,y as u8));
                    can_block_threat |= _coords.between(k_coords, 
                                            ChessPos::from(k_threat_raw));
                    
                },
                ChessPiece::WRook | ChessPiece::BRook => {
                    // Straight linear pathway check
                    if (dx==0 && dy!=0) || (dx!=0 && dy==0)
                    {
                        // Test coordinates to check if the target Rook can move
                        // and block the attack; same y-coordinate 
                        let test_v: ChessPos = ChessPos::from(ChessPos::conv(
                            _pos.x, k_threat_coords.y
                        ));

                        // Test coordinates to check if the target Rook can move
                        // and block the attack; same x-coordinate
                        let test_h: ChessPos = ChessPos::from(ChessPos::conv(
                            k_threat_coords.x, _pos.y
                        ));

                        let h_between: bool = test_h.between(k_threat_coords, k_coords) &&
                                                self.__verify_move(i, ChessPos::conv(k_threat_coords.x, _pos.y));
                        let v_between: bool = test_v.between(k_threat_coords, k_coords) &&
                                                self.__verify_move(i, ChessPos::conv(_pos.x, k_threat_coords.x));

                        can_block_threat |= h_between || v_between;
                        return;
                    }

                    // derivative of the diagonal with respect to y, either -1 or +1
                    // its going to be used to calculate the collision point for a test object
                    let diag_k_y: i8 = (k_threat_coords.x as i8-k_coords.x as i8)/(k_threat_coords.y as i8 - k_coords.y as i8);
                    // same derivative of the diagonal but with respect to x
                    let diag_k_x: i8 = 1/diag_k_y;

                    // basically calculate the x value from the derivative based linear
                    // function, given the value y
                    let shift_x: i8 = diag_k_y*(_pos.y as i8 - k_coords.y as i8);
                    // ...and transform it to 'easily' handled coordinates
                    let test_h: ChessPos = ChessPos::from(ChessPos::conv(
                        (k_coords.x as i8+shift_x) as u8,_pos.y
                    ));

                    // Same as above but calculate the y value from the derivative based
                    // linear function, but given the value x
                    let shift_y: i8 = diag_k_x*(_pos.x as i8 - k_coords.x as i8);
                    let test_v: ChessPos = ChessPos::from(ChessPos::conv(
                        _pos.x, (k_coords.y as i8+shift_y) as u8
                    ));

                    // Verify if the target rook is between the elements and that moves
                    // are actually possible to make
                    let verify_h: bool = test_h.between(k_coords, k_threat_coords) &&
                                            self.__verify_move(i, test_h.raw());
                    let verify_v: bool = test_v.between(k_coords, k_threat_coords) &&
                                            self.__verify_move(i, test_v.raw());

                    can_block_threat |= verify_v || verify_h;
                }
            },
            ChessPiece::WBishop | ChessPiece::BBishop => {
                // Check if the attack pathway is straight
                if (dx==0 && dy!=0) || (dx!=0 && dy==0)
                {

                }
            },
        }
        
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
            ChessPiece::WPawn => {
                let _is_1down: bool = f.x==t.x && t.y-f.y==1;
                let _is_2down: bool = f.x==t.x && t.y-f.y==2;
                let _is_ldiag: bool = f.x-t.x==1 && t.y-f.y==1;
                let _is_rdiag: bool = t.x-f.x==1 && t.y-f.y==1;

                // If the given move is not in the list of the allowed onces..
                assert!(_is_1down || _is_2down || _is_ldiag || _is_rdiag,
                    "Illegal move for a white pawn");

                // Are the moves allowed?
                let _v_1down: bool  = _is_1down && to_el.is_empty();
                let _v_2down: bool  = _is_2down && self.__empty_pathway(from, to, 
                    false,ChessPathway::Straight) && f.y==1; 
                let _v_ldiag: bool  = _is_ldiag && to_el.is_enemy_to(from_el);
                let _v_rdiag: bool  = _is_rdiag && to_el.is_enemy_to(from_el);

                return _v_1down || _v_2down || _v_ldiag || _v_rdiag;
            },
            ChessPiece::BPawn => {
                let _is_1up: bool   = f.x==t.x && f.y-t.y==1;
                let _is_2up: bool   = f.x==t.x && f.y-t.y==2;
                let _is_ldiag: bool = f.x-t.x==1 && f.y-t.y==1;
                let _is_rdiag: bool = t.x-f.x==1 && f.y-t.y==1;

                assert!(_is_1up || _is_2up || _is_ldiag || _is_rdiag,
                    "Illegal move for a black pawn");
                
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