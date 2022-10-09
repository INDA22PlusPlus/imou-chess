use super::*;
use std::ops::Range;


#[cfg(test)]
mod tests;

impl ChessBoard
{
    pub fn get_state(&self) -> ChessState
    {
        return self._state;
    }

    // Get the board
    // Public
    pub fn get_board(&self) -> [ChessPiece; 64]
    {
        return self.board;
    }

    // If white's turn
    // Public
    pub fn white_turn(&self) -> bool
    {
        return !self.w_lock;
    }

    // Get the piece from the board based on the coordinates
    // Public
    pub fn get_piece(&self, coords: u8) -> ChessPiece
    {
        return self.board[coords as usize];
    }

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
            _default_promotion: ChessPieceType::Queen, _state: ChessState::On,
            _piece_count: _piece_count }

    }

    // Returns true if promotion is possible. Should be used before `ChessBoard::drag`
    // method when implementing a GUI for this library, to allow the user to choose
    // a promotion chess piece
    pub fn promotion_check(&mut self, from: u8, to: u8) -> bool
    {
        let _from: ChessPos = ChessPos::from(from, false);
        let _to: ChessPos = ChessPos::from(from, false);

        let _from_el: ChessPiece = self.board[from as usize];
        let _to_el: ChessPiece = self.board[to as usize];

        if !_to_el.is_empty()
        {
            return false;
        }

        let case1: bool = _from_el == ChessPiece::WPawn && _to.y == 7;
        let case2: bool = _from_el == ChessPiece::BPawn && _to.y == 0;
  
        return case1 || case2;
    }

    // Main function that realizes the move of one chess piece at `from` coordinates
    // to `to` coordinate block. The function validates if the move to be made is valid
    // and moves the chess piece to the given coordinates if the move is indeed valid.
    // thereafter a lock is being set or unset for the white color indicating that it's
    // black's or white's turn next.
    pub fn drag(&mut self, from: u8, to: u8)
    {
        // If the game has ended do not allow the game to continue;
        // assert!(self._state==ChessState::On);

        // A kinder version of assert
        if self._state != ChessState::On
        {
            return;
        }

        // Checks if the move is allowed, takes in account if for example
        // a pawn can 'eat' a enemy piece by taking a straight step
        if !(self.__verify_move(from, to))
        {
            // Raise error that the move isn't allowed
            // assert!(false, "Illegal move");

            // Just exit if cannot do a move
            return;
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

        // Promotion replacement when a pawn reaches the top/bottom
        // `__verify_move` already checks if the square above or bellow a
        // pawn is empty 
        if from_el==ChessPiece::WPawn && ChessPos::from(to, false).y == 7
        {
            // Dirty lookup and conversion
            match self._default_promotion
            {
                ChessPieceType::King => assert!(false, "Pawn cannot be promoted to a king"),
                ChessPieceType::Bishop =>   self.board[to as usize] = ChessPiece::WBishop,
                ChessPieceType::Knight =>   self.board[to as usize] = ChessPiece::WKnight,
                ChessPieceType::Queen =>    self.board[to as usize] = ChessPiece::WQueen,
                ChessPieceType::Rook =>     self.board[to as usize] = ChessPiece::WRook,
                ChessPieceType::Pawn =>     (),
            }
        }

        if from_el==ChessPiece::BPawn && ChessPos::from(to, false).y == 0
        {
            // Dirty lookup and conversion
            match self._default_promotion
            {
                ChessPieceType::King => assert!(false, "Pawn cannot be promoted to a king"),
                ChessPieceType::Bishop =>   self.board[to as usize] = ChessPiece::BBishop,
                ChessPieceType::Knight =>   self.board[to as usize] = ChessPiece::BKnight,
                ChessPieceType::Queen =>    self.board[to as usize] = ChessPiece::BQueen,
                ChessPieceType::Rook =>     self.board[to as usize] = ChessPiece::BRook,
                ChessPieceType::Pawn =>     (),
            }
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
        let coords: ChessPos = ChessPos::from(coords_raw, false);

        let _pawn: ChessPiece = if _wh {ChessPiece::WPawn} else {ChessPiece::BPawn};
        let _king: ChessPiece = if _wh {ChessPiece::WKing} else {ChessPiece::BKing};
        let _rook: ChessPiece = if _wh {ChessPiece::WRook} else {ChessPiece::BRook};
        let _bishop: ChessPiece = if _wh {ChessPiece::WBishop} else {ChessPiece::BBishop};
        let _knight: ChessPiece = if _wh {ChessPiece::WKnight} else {ChessPiece::BKnight};
        let _queen: ChessPiece = if _wh {ChessPiece::WQueen} else {ChessPiece::BQueen};

        let _straight_attack: Vec<ChessPiece> = vec![_rook, _queen];
        let _diagonal_attack: Vec<ChessPiece> = vec![_bishop, _queen];

        // Check if horisontal line after the block threatens the block
        for i in 1..=(7-coords.x)
        {
            
            let enemy_coords: u8 = ChessPos::conv(coords.x+i, coords.y);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return ChessPos::conv(coords.x+i, coords.y) }
            
            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if horizontal line before the block threatens the block
        for i in 1..=coords.x
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x-i, coords.y);
            let _el: ChessPiece = self.board[enemy_coords as usize];   
            if _el == _king && i == 1 { return enemy_coords; }

            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        /////////////////////////////////////////
        // Check if vertical line before the block threatens the block
        for i in 1..=coords.y
        {
            let enemy_coords: u8 = ChessPos::conv(coords.x, coords.y-i);
            let _el: ChessPiece = self.board[enemy_coords as usize];
            if _el == _king && i == 1 { return enemy_coords; }

            if !_straight_attack.contains(&_el) { break; }

            if !_el.is_empty() { return enemy_coords; }
        }

        // Check if vertical line above the block threatens the block
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



        for _pos in Self::__gen_possible_gamma_moves(coords).iter()
        {
            let enemy_coords: u8 = ChessPos::conv(_pos.x, _pos.y);
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
        let k_coords: ChessPos = ChessPos::from(k_coords_raw, false);
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

        // If not stalemate, but no threat - just exit
        if k_threat_raw == 64 {return;}

        // Check whether it is possible to eliminate the threat to the king
        // if it is possible - just exit
        if self.__is_threatened(k_threat_raw, !self.w_lock) != 64
        {
            return;
        }


        let k_threat_coords: ChessPos = ChessPos::from(k_threat_raw, false);
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


        for i in 0..64 as u8
        {
            // Yeah bad brute force - sorting out all pieces and checking if they
            // can block the attack on the king
            let _el: ChessPiece = self.board[i as usize];
            if !_el.is_enemy_to(_threat)
            {
                continue;
            }

            let _pos: ChessPos = ChessPos::from(i as u8, false);

            match _el
            {
                ChessPiece::Empty | ChessPiece::WKing | ChessPiece::BKing => {continue;},
                ChessPiece::WPawn | ChessPiece::BPawn => {
                    let x: i8 = _pos.x as i8;
                    let mut y: i8 = _pos.y as i8;
                    y += if _el.is_black() { -1 } else { 1 }; 

                    if !r.contains(&y)
                    {
                        continue;
                    }

                    // No move validation is needed because of one step
                    let _coords: ChessPos = ChessPos::from(ChessPos::conv(x as u8,y as u8),
                                                false);
                    let can_block_threat: bool = _coords.between(k_coords, 
                                            ChessPos::from(k_threat_raw, false),
                                            false);
                    
                    if can_block_threat { return; }
                    
                },
                ChessPiece::WRook | ChessPiece::BRook => {
                    let can_block_threat: bool = self.__can_straight_block_attack(
                        k_coords, k_threat_coords, _pos);

                    if can_block_threat { return; }
                },
                ChessPiece::WBishop | ChessPiece::BBishop => {
                    let can_block_threat: bool = self.__can_diag_block_attack(
                        k_coords, k_threat_coords, _pos);
                    
                    if can_block_threat { return; }
                },
                ChessPiece::WQueen | ChessPiece::BQueen => {
                    let can_block_threat: bool = self.__can_straight_block_attack(
                        k_coords, k_threat_coords, _pos) ||
                        self.__can_diag_block_attack(
                            k_coords, k_threat_coords, _pos);

                    if can_block_threat { return; }
                },
                ChessPiece::WKnight | ChessPiece::BKnight => {
                    for _pos in Self::__gen_possible_gamma_moves(_pos).iter()
                    {
                        let can_block_threat = _pos.between(k_coords, 
                                                k_threat_coords, false);

                        if can_block_threat { return; }
                    }
                }
            }
        }
        
        // If the is not able to be blocked - set checkmate
        self._state = ChessState::Checkmate;
    }

    fn __can_straight_block_attack(&mut self, start: ChessPos, end: ChessPos, from: ChessPos) -> bool
    {
        let dx: i8 = end.x as i8 - start.x as i8;
        let dy: i8 = end.y as i8 - start.y as i8;

        // derivative of the diagonal with respect to y, either -1 or +1
        // its going to be used to calculate the collision point for a test object
        let diag_k_y: i8 = dx/dy;
        // same derivative of the diagonal but with respect to x
        let diag_k_x: i8 = dy/dx;

        // Straight linear pathway check
        if (dx==0 && dy!=0) || (dx!=0 && dy==0)
        {
            // Test coordinates to check if the target Rook can move
            // and block the attack; same y-coordinate 
            let test_v: ChessPos = ChessPos::from(ChessPos::conv(
                from.x, end.y
            ), false);

            // Test coordinates to check if the target Rook can move
            // and block the attack; same x-coordinate
            let test_h: ChessPos = ChessPos::from(ChessPos::conv(
                end.x, from.y
            ), false);

            let h_between: bool = test_h.between(end, start, true) &&
                                    self.__verify_move(from.raw(), 
                                        ChessPos::conv(end.x, from.y)
                                    );
            let v_between: bool = test_v.between(end, start, true) &&
                                    self.__verify_move(from.raw(),
                                        ChessPos::conv(from.x, end.x));

            return h_between || v_between;
        }

        // basically calculate the x value from the derivative based linear
        // function, given the value y
        let shift_x: i8 = diag_k_y*(from.y as i8 - start.y as i8);
        // ...and transform it to 'easily' handled coordinates
        let test_h: ChessPos = ChessPos::from(ChessPos::conv(
            (start.x as i8+shift_x) as u8,from.y
        ), true);

        // Same as above but calculate the y value from the derivative based
        // linear function, but given the value x
        let shift_y: i8 = diag_k_x*(from.x as i8 - start.x as i8);
        let test_v: ChessPos = ChessPos::from(ChessPos::conv(
            start.x, (from.y as i8+shift_y) as u8
        ), true);

        // Verify if the target rook is between the elements and that moves
        // are actually possible to make
        let verify_h: bool = test_h.between(start, end, true) &&
                                self.__verify_move(from.raw(), test_h.raw());
        let verify_v: bool = test_v.between(start, end, true) &&
                                self.__verify_move(from.raw(), test_v.raw());

        return verify_v || verify_h;
    }

    fn __can_diag_block_attack(&mut self, start: ChessPos, end: ChessPos, from: ChessPos) -> bool
    {
        let dx: i8 = end.x as i8 - start.x as i8;
        let dy: i8 = end.y as i8 - start.y as i8;
        // Check if the attack pathway is straight
        // aka straight line - diagonal 'collision' check
        if (dx==0 && dy!=0) || (dx!=0 && dy==0)
        {
            // Simple algebraic checking
            // First of all the bishop can in 2 directions
            // Described by the equations:
            // x+y_0-x_0
            // &&
            // -x+y_0+x_0
            // Where x_0 and y_0 are the coordinates of the
            // bishop.

            // If the attack pathway is straight vertical
            // it's pretty simple to check if the bishop
            // can move in a way to block the attack.

            let _y1: u8 = start.x + (from.y as i8 - from.x as i8) as u8;
            let _y2: u8 = (((from.y+from.x) as i8)-(start.x as i8)) as u8;
            let test_v1: ChessPos = ChessPos::from(ChessPos::conv(
                start.x, _y1
            ), true);
            let test_v2: ChessPos = ChessPos::from(ChessPos::conv(
                start.x, _y2
            ), true);

            let _x1: u8 = (start.y as i8 - from.x as i8 + from.y as i8) as u8;
            let _x2: u8 = (-(start.y as i8) + from.x as i8 + from.y as i8) as u8;
            let test_h1: ChessPos = ChessPos::from(ChessPos::conv(
                _x1, start.y
            ), true);
            let test_h2: ChessPos = ChessPos::from(ChessPos::conv(
                _x1, start.y 
            ), true);

            // Check if the function pos, is inside the attack pathway or not
            let mut valid_v: bool = dx==0;
            valid_v &= test_v1.between(start, end, true) ||
                        test_v2.between(start, end, true);
            
            valid_v &= self.__verify_move(from.raw(), test_v1.raw()) ||
                        self.__verify_move(from.raw(), test_v2.raw());

            let mut valid_h: bool = dy==0;
            valid_h &= test_h1.between(start, end, true) ||
                        test_h2.between(start, end, true);
            
            valid_h &= self.__verify_move(from.raw(), test_h1.raw()) ||
                        self.__verify_move(from.raw(), test_h2.raw());

            return valid_v || valid_h;
        }

        // kx+m, m value for the diagonal attack linear equation
        let k_attack: i8 = dy/dx;
        let m_attack: i8 = -(start.x as i8)*k_attack+(start.y as i8);

        let k1: i8 = 1;
        // m value for the bishop diagonal linear equation with a derivative of +1
        let m1: i8 = (from.y as i8)-(from.x as i8);

        let k2: i8 = -1;
        // ... m value for the linear equation with a derivative of -1
        let m2: i8 = (from.y as i8)+(from.x as i8);

        // The determinant between the attack pathway and bishop's diagonal equations
        let det_1_attack: i8 = -k_attack + k1;
        let det_2_attack: i8 = -k_attack + k2;

        // if determinant == 0, lines are parallel, and in the case of this game, this
        // cannot be possible because otherwise it would block the attack per automatica
        let x_intercept1: i8 = if det_1_attack==0{64}else{(m_attack-m1)/det_1_attack};
        let y_intercept1: i8 = if det_1_attack==0{64}else{(-k_attack*m1+k1*m_attack)/det_1_attack};

        let x_intercept2: i8 = if det_2_attack==0{64}else{(m_attack-m2)/det_2_attack};
        let y_intercept2: i8 = if det_2_attack==0{64}else{(-k_attack*m2+k2*m_attack)/det_2_attack};

        let intercept1: ChessPos = ChessPos::from(ChessPos::conv(
            x_intercept1 as u8, y_intercept1 as u8
        ), true);
        let intercept2: ChessPos = ChessPos::from(ChessPos::conv(
            x_intercept2 as u8, y_intercept2 as u8
        ), true);

        // Check if the iterception pos is inside the attack pathway
        let verify_intercept1 = intercept1.between(
            start, end, true
        ) && self.__verify_move(from.raw(), intercept1.raw());
        let verify_intercept2 = intercept2.between(
            start, end, true
        ) && self.__verify_move(from.raw(), intercept2.raw());

        return verify_intercept1 || verify_intercept2;
    }

    // Checks if the given pathway is empty.
    // `include_enemy` indicates if there's an enemy piece on
    // `to` coordinates and if it should be treated as an empty
    // position (can be 'eaten') or not.
    fn __empty_pathway(&mut self, from: u8, to: u8,
        include_enemy: bool, path: ChessPathway) -> bool
    {
        let f: ChessPos = ChessPos::from(from as u8, false);
        let t: ChessPos = ChessPos::from(to as u8, false);

        // I have no energy and time in researching on how custom exceptions
        // are created in Rust
        // assert!(from != to, "Given pathway: from = to");
        if from == to { return false; }

        let __r_dx: i8 = t.x as i8 - f.x as i8;
        let __r_dy: i8 = t.y as i8 - f.y as i8;

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
                // assert!(dx==dy, "Given pathway is not diagonal");
                if dx!=dy { return false; }

                for i in 1..=(dx-1)
                {
                    let __check_x: u8 = ((f.x as i8) + i*dx_sign) as u8;
                    let __check_y: u8 = ((f.y as i8) + i*dy_sign) as u8;
                    let __coords: u8 = ChessPos::conv(__check_x, __check_y);

                    let _el: ChessPiece = self.board[__coords as usize];
                    if !_el.is_empty() { return false; }
                }
            },

            ChessPathway::Straight => {
                let v_case: bool = (dx == 0) && (dy > 0);
                let h_case: bool = (dx > 0) && (dy == 0);
                // assert!(v_case || h_case, "Given pathway is not straight");

                if !(v_case || h_case) { return false; }
            
                // If diagonal, either dx or dy is 0
                for i in 1..=i8::max(dy-1, dx-1)
                {
                    
                    // As you see I love branchless programming
                    let __x: u8 = ((f.x as i8)+i*dx_sign*(h_case as i8)) as u8;
                    let __y: u8 = ((f.y as i8)+i*dy_sign*(v_case as i8)) as u8;

                    let _el: ChessPiece = self.board[ChessPos::conv(__x, __y) as usize];

                    if !_el.is_empty() { return false; }
                }
            },
            ChessPathway::Gamma => {
                let gamma_check: bool = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
                // assert!(gamma_check, "Given pathway is not of type gamma");

                if !gamma_check { return false; }
            }
        }
        
        return valid_le;
    }

    // Verifies if the move is allowed 
    fn __verify_move(&mut self, from: u8, to: u8) -> bool
    {
        // Cannot move a piece to the same location
        // assert!(from != to, "Given pathway: from = to");
        if from == to { return false; }

        // `ChessPos` constructor automatically checks if position
        // is inside the chess board or not. If not, it asserts an
        // error.
        let f: ChessPos = ChessPos::from(from as u8, false);
        let t: ChessPos = ChessPos::from(to as u8, false);

        let fx: i8 = f.x as i8;
        let fy: i8 = f.y as i8;
        let tx: i8 = t.x as i8;
        let ty: i8 = t.y as i8;

        let _abs_dx: u8     = i8::abs( fx - tx ) as u8;
        let _abs_dy: u8     = i8::abs( fy - ty ) as u8;

        let from_el: ChessPiece = self.board[from as usize];

    
        let _wrong_color: bool = from_el.is_black() == self.w_lock;
        // assert!(_wrong_color, "Illegal move, wrong color");
        if !_wrong_color { return false; }

        let to_el: ChessPiece = self.board[to as usize];

        match from_el {
            // Raise error when trying to move an empty piece
            ChessPiece::Empty => {
                // assert!(false, "Illegal move");
                return false;
            },
            ChessPiece::WPawn => {
                let _is_1down: bool = fx==tx && ty-fy==1;
                let _is_2down: bool = fx==tx && ty-fy==2;
                let _is_ldiag: bool = fx-tx==1 && ty-fy==1;
                let _is_rdiag: bool = tx-fx==1 && ty-fy==1;

                // If the given move is not in the list of the allowed onces..
                // assert!(_is_1down || _is_2down || _is_ldiag || _is_rdiag,
                //     "Illegal move for a white pawn");

                if !(_is_1down || _is_2down || _is_ldiag || _is_rdiag) { return false; }

                // Are the moves allowed?
                let _v_1down: bool  = _is_1down && to_el.is_empty();
                let _v_2down: bool  = _is_2down && self.__empty_pathway(from, to, 
                    false,ChessPathway::Straight) && fy==1; 
                let _v_ldiag: bool  = _is_ldiag && to_el.is_enemy_to(from_el);
                let _v_rdiag: bool  = _is_rdiag && to_el.is_enemy_to(from_el);

                return _v_1down || _v_2down || _v_ldiag || _v_rdiag;
            },
            ChessPiece::BPawn => {
                let _is_1up: bool   = fx==tx && fy-ty==1;
                let _is_2up: bool   = fx==tx && fy-ty==2;
                let _is_ldiag: bool = fx-tx==1 && fy-ty==1;
                let _is_rdiag: bool = tx-fx==1 && fy-ty==1;

                // assert!(_is_1up || _is_2up || _is_ldiag || _is_rdiag,
                //     "Illegal move for a black pawn");

                if !(_is_1up || _is_2up || _is_ldiag || _is_rdiag) { return false; }
                
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
                // assert!(_is_h || _is_v, "Illegal move for a rook");

                if !(_is_h || _is_v) { return false; }

                let _v_h: bool      = _is_h && self.__empty_pathway(from, to, 
                    true,ChessPathway::Straight);
                let _v_v: bool      = _is_v && self.__empty_pathway(from, to, 
                    true,ChessPathway::Straight);

                return _v_h || _v_v;
            },
            ChessPiece::BBishop | ChessPiece::WBishop => {

                // If the move is not diagonal, raise an error
                // assert!(_abs_dx==_abs_dy, "Illegal move for a bishop");

                if _abs_dx!=_abs_dy { return false; }

                return self.__empty_pathway(from, to, true, 
                    ChessPathway::Diagonal);
            },
            ChessPiece::BKnight | ChessPiece::WKnight => {
                let _is_hgamma: bool= _abs_dx==2 && _abs_dy==1;
                let _is_vgamma: bool= _abs_dx==1 && _abs_dy==2;

                // assert!(_is_hgamma || _is_vgamma, "Illegal move for a knight");

                if !(_is_hgamma || _is_vgamma) { return false; }
                
                // `ChessBoard::__empty_pathway` in this case is just going to look
                // if `to_el` is an enemy or not, but anyways.. I just follow the
                // same structure..

                return self.__empty_pathway(from, to, true, 
                    ChessPathway::Gamma);
            },
            ChessPiece::BKing | ChessPiece::WKing => {
                // to==from is already checked in the begining of this method
                // assert!(_abs_dx < 2 && _abs_dy < 2, "Illegal move for a king");
                if !(_abs_dx < 2 && _abs_dy < 2) { return false; }
                return !to_el.is_enemy_to(from_el);
            },
            ChessPiece::BQueen | ChessPiece::WQueen => {
                let _is_diag: bool  = _abs_dx==_abs_dy;
                let _is_h: bool     = _abs_dx>0 && _abs_dy==0;
                let _is_v: bool     = _abs_dx==0 && _abs_dy>0;

                // assert!(_is_diag || _is_h || _is_v, "Illegal move for a queen");

                if !(_is_diag || _is_h || _is_v) { return false;}
                
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

    fn __gen_possible_gamma_moves(from: ChessPos) -> Vec<ChessPos>
    {
        let r: Range<i8> = 0..8;
        let _x: i8 = from.x as i8;
        let _y: i8 = from.y as i8;

        let mut pos_vec: Vec<ChessPos> = Vec::new();

        ////////////////////////////////////////////////
        // Check if knight threatens the block
        if r.contains(&(_x+2)) && r.contains(&(_y+1))
        {
            pos_vec.push(ChessPos{x:(_x+2) as u8, y: (_y+1) as u8});
        }

        if r.contains(&(_x+2)) && r.contains(&(_y-1))
        {
            pos_vec.push(ChessPos{x:(_x+2) as u8, y: (_y-1) as u8});
        }

        if r.contains(&(_x-2)) && r.contains(&(_y+1))
        {
            pos_vec.push(ChessPos{x:(_x-2) as u8, y:(_y+1) as u8});
        }

        if r.contains(&(_x-2)) && r.contains(&(_y-1))
        {
            pos_vec.push(ChessPos{x:(_x-2) as u8, y:(_y-1) as u8});
        }

        //////////////////////////////////////
        if r.contains(&(_x+1)) && r.contains(&(_y+2))
        {
            pos_vec.push(ChessPos{x:(_x+1) as u8, y: (_y+2) as u8});
        }

        if r.contains(&(_x-1)) && r.contains(&(_y+2))
        {
            pos_vec.push(ChessPos{x:(_x-1) as u8, y: (_y+2) as u8});
        }

        if r.contains(&(_x+1)) && r.contains(&(_y-2))
        {
            pos_vec.push(ChessPos{x:(_x+1) as u8, y: (_y-2) as u8});
        }

        if r.contains(&(_x-1)) && r.contains(&(_y-2))
        {
            pos_vec.push(ChessPos{x:(_x-1) as u8, y: (_y-2) as u8});
        }

        return pos_vec;
    }

}