
    //let background_color: Color = Color::from_rgba(251, 240, 240, 255);
    //let white_color: Color = Color::from_rgba(251, 240, 240, 255);
    //let black_color: Color = Color::from_rgba(124, 117, 117, 255);
use macroquad::prelude::*;
use crate::types;
use crate::parse;

pub struct GameState {
    checkmate: Option<types::PieceColor>,

    white_color: Color,
    background_color: Color,
    black_color: Color,
    checked_color: Color,
    selected_color: Color,
    white_text_color: Color,
    black_text_color: Color,

    board: Vec<Vec<types::Piece>>,

    wking_tex: Texture2D, 
    wqueen_tex: Texture2D,
    wbishop_tex: Texture2D,
    wknight_tex: Texture2D,
    wrook_tex: Texture2D,
    wpawn_tex: Texture2D,

    bking_tex: Texture2D,
    bqueen_tex: Texture2D,
    bbishop_tex: Texture2D,
    bknight_tex: Texture2D,
    brook_tex: Texture2D,
    bpawn_tex: Texture2D,

    movedfrom_tex: Texture2D,

    selected: Option<Vec2>,
    movedfrom: Option<Vec2>,
    movedto: Option<Vec2>,

    current_color: types::PieceColor,

    fenstring: String
}

impl GameState {
    pub async fn new(fenstring: &str) -> GameState {
        GameState {
            checkmate: None,
            white_color: Color::from_rgba(98, 104, 128, 255),
            background_color: Color::from_rgba(48, 52, 70, 255),
            black_color: Color::from_rgba(48, 52, 70, 255),
            checked_color: Color::from_rgba(255, 89, 123, 120),
            selected_color: Color::from_rgba(163, 187, 152, 120),

            black_text_color: Color::from_rgba(0, 0, 0, 255),
            white_text_color: Color::from_rgba(255, 255, 255, 255),

            fenstring: String::from(fenstring),
            board: parse::parse_fen(fenstring),

            wking_tex: load_texture("res/wking.png").await.unwrap(),
            wqueen_tex: load_texture("res/wqueen.png").await.unwrap(),
            wbishop_tex: load_texture("res/wbishop.png").await.unwrap(),
            wknight_tex: load_texture("res/wknight.png").await.unwrap(),
            wrook_tex: load_texture("res/wrook.png").await.unwrap(),
            wpawn_tex: load_texture("res/wpawn.png").await.unwrap(),

            bking_tex: load_texture("res/bking.png").await.unwrap(),
            bqueen_tex: load_texture("res/bqueen.png").await.unwrap(),
            bbishop_tex: load_texture("res/bbishop.png").await.unwrap(),
            bknight_tex: load_texture("res/bknight.png").await.unwrap(),
            brook_tex: load_texture("res/brook.png").await.unwrap(),
            bpawn_tex: load_texture("res/bpawn.png").await.unwrap(),

            movedfrom_tex: load_texture("res/movedfrom.png").await.unwrap(),

            selected: None,
            movedfrom: None,
            movedto: None,

            current_color: types::PieceColor::White
        }
    }

    pub fn restart(&mut self) {
        self.checkmate = None;
        self.board = parse::parse_fen(self.fenstring.as_ref());
        self.selected = None;
        self.movedfrom = None;
        self.movedto = None;
        self.current_color = types::PieceColor::White;
    }

    pub fn get_check(&self, current_color: types::PieceColor) -> bool {
        for (idx, _) in self.board.iter().enumerate() {
            for (idx2, _) in self.board[idx].iter().enumerate() {
                let moves = self.possible_moves(idx as u8, idx2 as u8, false, &current_color);
    
                for pmove in moves {
                    if pmove.x >= 0. && pmove.x < 8. && pmove.y >= 0. && pmove.y < 8. {
                        if matches!(self.board[pmove.x as usize][pmove.y as usize], types::Piece::King(_)) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    
    pub fn possible_moves(&self, x: u8, y: u8, remove: bool, current_color: &types::PieceColor) -> Vec<Vec2> {
        match &self.board[x as usize][y as usize] {
            types::Piece::Pawn(_color) | types::Piece::Rook(_color) | types::Piece::Knight(_color) | types::Piece::Bishop(_color) | types::Piece::Queen(_color) | types::Piece::King(_color) => {
                if matches!(current_color, types::PieceColor::Black) && matches!(*_color, types::PieceColor::White) {
                    return vec![];
                }
                if matches!(current_color, types::PieceColor::White) && matches!(*_color, types::PieceColor::Black) {
                    return vec![];
                }
            },
            types::Piece::Empty => {}
        }
    
        let mut possible: Vec<Vec2> = vec![];
    
        match &self.board[x as usize][y as usize] {
            types::Piece::Pawn(color) => {
                let y_offset: i8 = if matches!(color, types::PieceColor::Black) {1} else {-1}; 
    
                let is_at_start: bool = (matches!(color, types::PieceColor::White) && y == 6) || (matches!(color, types::PieceColor::Black) && y == 1);
    
                if y as i8 + y_offset * 2 >= 0 && y as i8 + y_offset * 2 < 8 {
                    if matches!(self.board[x as usize][(y as i8 + y_offset * 2) as usize], types::Piece::Empty) && matches!(self.board[x as usize][(y as i8 + y_offset) as usize], types::Piece::Empty) && is_at_start {
                        possible.push(Vec2::new(x as f32, y as f32 + y_offset as f32 * 2.));
                    }
                }
    
                if y as i8 + y_offset >= 0 {
                    if matches!(self.board[x as usize][(y as i8 + y_offset) as usize], types::Piece::Empty) {
                        possible.push(Vec2::new(x as f32, y as f32 + y_offset as f32));
                    }
                }
    
                for x_offset in [-1, 1] {
                    if !(x as i8 + x_offset >= 0 && x as i8 + x_offset < 8 && y as i8 + y_offset >= 0 && y as i8 + y_offset < 8) {
                        continue;
                    }
    
                    if !matches!(self.board[(x as i8 + x_offset) as usize][(y as i8 + y_offset) as usize], types::Piece::Empty) {
    
                        let diff = self.different_color(x, y, (x as i8 + x_offset) as u8, (y as i8 + y_offset) as u8);
                        if diff {
                            possible.push(Vec2::new((x as i8 + x_offset) as f32, (y as i8 + y_offset) as f32));
                        }
                    }
                }
            },
            types::Piece::Knight(_) => {
                if (x as i8 - 2 >= 0) && (y as i8 - 1 >= 0) {
                    if self.different_color(x, y, x - 2, y - 1) {
                    possible.push(Vec2::new(x as f32 - 2., y as f32 - 1.));
                    }
                }
                if (x as i8 - 1 >= 0) && (y as i8 - 2 >= 0) {
                    if self.different_color(x, y, x - 1, y - 2) {
                        possible.push(Vec2::new(x as f32 - 1., y as f32 - 2.));
                    }
                }
    
                if (x + 1 < 8) && (y as i8 - 2 >= 0) {
                    if self.different_color(x, y, x + 1, y - 2) {
                        possible.push(Vec2::new(x as f32 + 1., y as f32 - 2.));
                    }
                }
                if (x + 2 < 8) && (y as i8 - 1 >= 0) {
                    if self.different_color(x, y, x + 2, y - 1) {
                        possible.push(Vec2::new(x as f32 + 2., y as f32 - 1.));
                    }
                }
    
                if (x + 2 < 8) && (y + 1 < 8) {
                    if self.different_color(x, y, x + 2, y + 1) {
                        possible.push(Vec2::new(x as f32 + 2., y as f32 + 1.));
                    }
                }
                if (x + 1 < 8) && (y + 2 < 8) {
                    if self.different_color(x, y, x + 1, y + 2) {
                        possible.push(Vec2::new(x as f32 + 1., y as f32 + 2.));
                    }
                }
    
                if (x as i8 - 1 >= 0) && (y as i8 + 2 >= 0) {
                    if self.different_color(x, y, x - 1, y + 2) {
                        possible.push(Vec2::new(x as f32 - 1., y as f32 + 2.));
                    }
                }
                if (x as i8 - 2 >= 0) && (y + 1 < 8) {
                    if self.different_color(x, y, x - 2, y + 1) {
                        possible.push(Vec2::new(x as f32 - 2., y as f32 + 1.));
                    }
                }
    
            },
            types::Piece::Rook(_) => {
                for i in (0..x).rev() {
                    if self.get_color( i as u8, y).is_some() {
                        if self.different_color(x, y, i, y) {
                            possible.push(Vec2::new(i as f32, y as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(i as f32, y as f32));
                }
    
                for i in (x + 1)..8 {
                    if self.get_color( i as u8, y).is_some() {
                        if self.different_color(x, y, i, y) {
                            possible.push(Vec2::new(i as f32, y as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(i as f32, y as f32));
                }
    
                for i in (0..y).rev() {
                    if self.get_color( x, i as u8).is_some() {
                        if self.different_color(x, y, x, i) {
                            possible.push(Vec2::new(x as f32, i as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(x as f32, i as f32));
                }
    
                for i in (y + 1)..8 {
                    if self.get_color( x, i as u8).is_some() {
                        if self.different_color(x, y, x, i) {
                            possible.push(Vec2::new(x as f32, i as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(x as f32, i as f32));
                }
            },
            types::Piece::Bishop(_) => {
                for i in 1..8 {
                    if !(x + i < 8 && y + i < 8) {
                        continue;
                    }
                    if self.get_color( x + i, y + i).is_some() {
                        if self.different_color(x, y, x + i, y + i) {
                            possible.push(Vec2::new((x + i) as f32, (y + i) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x + i) as f32, (y + i) as f32));
                }
    
                for i in 1..8 {
                    if !(x as i8 - i >= 0 && y as i8 - i >= 0) {
                        continue;
                    }
                    if self.get_color( x - i as u8, y - i as u8).is_some() {
                        if self.different_color(x, y, x - i as u8, y - i as u8) {
                            possible.push(Vec2::new((x - i as u8) as f32, (y - i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x - i as u8) as f32, (y - i as u8) as f32));
                }
    
                for i in 1..8 {
                    if !(x as i8 + i < 8 && y as i8 - i >= 0) {
                        continue;
                    }
                    if self.get_color( x + i as u8, y - i as u8).is_some() {
                        if self.different_color(x, y, x + i as u8, y - i as u8) {
                            possible.push(Vec2::new((x + i as u8) as f32, (y - i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x + i as u8) as f32, (y - i as u8) as f32));
                }
    
                for i in 1..8 {
                    if !(x as i8 - i >= 0 && y as i8 + i < 8) {
                        continue;
                    }
                    if self.get_color( x - i as u8, y + i as u8).is_some() {
                        if self.different_color(x, y, x - i as u8, y + i as u8) {
                            possible.push(Vec2::new((x - i as u8) as f32, (y + i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x - i as u8) as f32, (y + i as u8) as f32));
                }
            },
            types::Piece::King(_) => {
                let i = 1;
                if x + 1 < 8 && y + 1 < 8 {
                    if self.different_color(x, y, x + 1, y + 1) {
                        possible.push(Vec2::new((x + i) as f32, (y + i) as f32));
                    }
                } if y + 1 < 8 {
                    if self.different_color(x, y, x, y + i) {
                        possible.push(Vec2::new((x) as f32, (y + i) as f32));
                    }
                } if x as i8 - 1 >= 0 && y + 1 < 8 {
                    if self.different_color(x, y, x - 1, y + 1) {
                        possible.push(Vec2::new((x - i) as f32, (y + i) as f32));
                    }
                }
    
                if x + 1 < 8 {
                    if self.different_color(x, y, x + 1, y) {
                        possible.push(Vec2::new((x + i) as f32, (y) as f32));
                    }
                } if x as i8 - 1 >= 0 {
                    if self.different_color(x, y, x - 1, y) {
                        possible.push(Vec2::new((x - i) as f32, (y) as f32));
                    }
                }
    
                if x + 1 < 8 && y as i8 - 1 >= 0 {
                    if self.different_color(x, y, x + 1, y - 1) {
                        possible.push(Vec2::new((x + i) as f32, (y - i) as f32));
                    }
                } if y as i8 - 1 >= 0 {
                    if self.different_color(x, y, x, y - 1) {
                        possible.push(Vec2::new((x) as f32, (y - i) as f32));
                    }
                } if x as i8 - 1 >= 0 && y as i8 - 1 >= 0 {
                    if self.different_color(x, y, x - 1, y - 1) {
                        possible.push(Vec2::new((x - i) as f32, (y - i) as f32));
                    }
                }
            },
            types::Piece::Queen(_) => {
                for i in 1..8 {
                    if !(x as i8 + i < 8 && y as i8 + i < 8) {
                        continue;
                    }
                    if self.get_color( x + i as u8, y + i as u8).is_some() {
                        if self.different_color(x, y, x + i as u8, y + i as u8) {
                            possible.push(Vec2::new((x + i as u8) as f32, (y + i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x + i as u8) as f32, (y + i as u8) as f32));
                }
    
                for i in 1..8 {
                    if !(x as i8 - i >= 0 && y as i8 - i >= 0) {
                        continue;
                    }
                    if self.get_color( x - i as u8, y - i as u8).is_some() {
                        if self.different_color(x, y, x - i as u8, y - i as u8) {
                            possible.push(Vec2::new((x - i as u8) as f32, (y - i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x - i as u8) as f32, (y - i as u8) as f32));
                }
    
                for i in 1..8 {
                    if !(x as i8 + i < 8 && y as i8 - i >= 0) {
                        continue;
                    }
                    if self.get_color( x + i as u8, y - i as u8).is_some() {
                        if self.different_color(x, y, x + i as u8, y - i as u8) {
                            possible.push(Vec2::new((x + i as u8) as f32, (y - i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x + i as u8) as f32, (y - i as u8) as f32));
                }
    
                for i in 1..8 {
                    if !(x as i8 - i >= 0 && y as i8 + i < 8) {
                        continue;
                    }
                    if self.get_color( x - i as u8, y + i as u8).is_some() {
                        if self.different_color(x, y, x - i as u8, y + i as u8) {
                            possible.push(Vec2::new((x - i as u8) as f32, (y + i as u8) as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new((x - i as u8) as f32, (y + i as u8) as f32));
                }
    
                for i in (0..x).rev() {
                    if self.get_color( i as u8, y).is_some() {
                        if self.different_color(x, y, i, y) {
                            possible.push(Vec2::new(i as f32, y as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(i as f32, y as f32));
                }
    
                for i in (x + 1)..8 {
                    if self.get_color( i as u8, y).is_some() {
                        if self.different_color(x, y, i, y) {
                            possible.push(Vec2::new(i as f32, y as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(i as f32, y as f32));
                }
    
                for i in (0..y).rev() {
                    if self.get_color( x, i as u8).is_some() {
                        if self.different_color(x, y, x, i) {
                            possible.push(Vec2::new(x as f32, i as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(x as f32, i as f32));
                }
    
                for i in (y + 1)..8 {
                    if self.get_color( x, i as u8).is_some() {
                        if self.different_color(x, y, x, i) {
                            possible.push(Vec2::new(x as f32, i as f32));
                        }
                        break;
                    }
                    possible.push(Vec2::new(x as f32, i as f32));
                }
            },
            _ => {}
        }
        if remove {
            let length = possible.len().clone();
            for idx in (0..length).rev() {
                let mut new_board: Vec<Vec<types::Piece>> = self.board.clone();
        
                if x as i8 >= 0 && x < 8 && y as i8 >= 0 && y < 8 && possible[idx].x >= 0. && possible[idx].x < 8. && possible[idx].y >= 0. && possible[idx].y < 8. {
                    new_board[possible[idx].x as usize][possible[idx].y as usize] = self.board[x as usize][y as usize];
                    new_board[x as usize][y as usize] = types::Piece::Empty;
                }
        
                if self.get_check(*current_color) {
                    possible.remove(idx);
                }
            }
        }
    
        possible
    }
    
    pub fn get_color(&self, x: u8, y: u8) -> Option<types::PieceColor> {
        if !((x as i8 >= 0 && x < 8) && (y as i8 >= 0 && y < 8)) {
            return None;
        }
    
         match self.board[x as usize][y as usize] {
            types::Piece::Pawn(_color) | types::Piece::Rook(_color) | types::Piece::Knight(_color) | types::Piece::Bishop(_color) | types::Piece::Queen(_color) | types::Piece::King(_color) => {
                Some(_color)
            },
            types::Piece::Empty => None
        }
    
    }
    
    pub fn different_color(&self, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
        if self.get_color( x1, y1).is_none() && self.get_color(x2, y2).is_none() {
            return false;
        }
        if self.get_color(x2, y2).is_none() {
            return true;
        }
        if self.get_color(x1, y1).is_none() {
            return true;
        }
    
        match self.get_color(x1, y1).unwrap() {
            types::PieceColor::Black => {
                match self.get_color(x2, y2).unwrap() {
                    types::PieceColor::Black => false,
                    types::PieceColor::White => true
                }
            },
            types::PieceColor::White => {
                match self.get_color(x2, y2).unwrap() {
                    types::PieceColor::Black => true,
                    types::PieceColor::White => false
                }
            }
        }
    }
}

pub fn game_loop(gs: &mut GameState) {
    clear_background(gs.background_color);

    let min_val: f32 = if screen_width() > screen_height() {screen_height()} else {screen_width()};

    let x_offset: f32 = if screen_width() > screen_height() {(screen_width() - min_val) / 2.} else {0.};
    let y_offset: f32 = if screen_height() > screen_width() {(screen_height() - min_val) / 2.} else {0.};

    if is_mouse_button_pressed(MouseButton::Right) && gs.checkmate.is_some() {
        gs.restart();
    }

    if is_mouse_button_pressed(MouseButton::Left) && gs.checkmate.is_none() {
        let mut new_x = mouse_position().0;
        let mut new_y = mouse_position().1;

        new_x -= x_offset;
        new_y -= y_offset;

        new_x = (new_x / (min_val / 8.)).floor() * (min_val / 8.) / (min_val / 8.);
        new_y = (new_y / (min_val / 8.)).floor() * (min_val / 8.) / (min_val / 8.);

        println!("({}, {})", new_x, new_y);

        if gs.selected.is_none()  {
            if new_x >= 0. && new_x < 8. && new_y >= 0. && new_y < 8. {
                gs.selected = Some(Vec2::new(new_x, new_y));
                gs.movedfrom = None;
            }
        } else if !gs.possible_moves(gs.selected.unwrap().x as u8, gs.selected.unwrap().y as u8, true, &gs.current_color).contains(&Vec2::new(new_x, new_y)){
            gs.selected = None;
        } else {
            gs.board[new_x as usize][new_y as usize] = gs.board[gs.selected.unwrap().x as usize][gs.selected.unwrap().y as usize];
            gs.board[gs.selected.unwrap().x as usize][gs.selected.unwrap().y as usize] = types::Piece::Empty;

            if ((new_y == 7. && matches!(gs.current_color, types::PieceColor::Black)) || (new_y == 0. && matches!(gs.current_color, types::PieceColor::White))) && matches!(gs.board[new_x as usize][new_y as usize], types::Piece::Pawn(_)) {
                gs.board[new_x as usize][new_y as usize] = types::Piece::Queen(gs.current_color);
            }

            let mut black_possible = 0;
            let mut white_possible = 0;

            for x in 0..gs.board.len() {
                for y in 0..gs.board[x].len() {
                    match &gs.board[x][y] {
                        types::Piece::Pawn(_color) | types::Piece::Rook(_color) | types::Piece::Knight(_color) | types::Piece::Bishop(_color) | types::Piece::Queen(_color) | types::Piece::King(_color) => {
                            if matches!(_color, types::PieceColor::Black) {
                                black_possible += gs.possible_moves(x as u8, y as u8, true, _color).len();
                            } else {
                                white_possible += gs.possible_moves(x as u8, y as u8, true, _color).len();
                            }
                        },
                        types::Piece::Empty => {}
                    }
                }
            }

            if black_possible == 0 {
                gs.checkmate = Some(types::PieceColor::White);
            }
            if white_possible == 0 {
                gs.checkmate = Some(types::PieceColor::Black);
            }

            gs.current_color = match gs.current_color {
                types::PieceColor::White => types::PieceColor::Black,
                types::PieceColor::Black => types::PieceColor::White
            };

            gs.movedfrom = Some(Vec2::new(gs.selected.unwrap().x, gs.selected.unwrap().y));
            gs.movedto = Some(Vec2::new(new_x, new_y));
            gs.selected = None;
        }
    }

    for x in 0..8 {
        for y in 0..8 {
            draw_rectangle(x as f32 * (min_val / 8.) + x_offset, y as f32 * (min_val / 8.) + y_offset, min_val / 8., min_val / 8., if (x+y)%2 == 0 {gs.white_color} else {gs.black_color});

            if x==0 {
                //draw_text((8 - y).to_string().as_str(), x as f32 * (min_val / 8.) + x_offset, (y + 1) as f32 * (min_val / 8.) - (min_val / 8.) + y_offset + 42.0, 64.0, text_color);
            }

            if y==7 {
                let letter: &str = match x {
                    0 => "a",
                    1 => "b",
                    2 => "c",
                    3 => "d",
                    4 => "e",
                    5 => "f",
                    6 => "g",
                    7 => "h",
                    _ => "a"
                };
                //draw_text(letter, x as f32 * (min_val / 8.) + x_offset + (min_val / 8.) - 28., (y + 1) as f32 * (min_val / 8.) - (min_val / 8.) + y_offset + (min_val / 8.), 64.0, text_color);
            }

            let side_px: f32 = min_val / 8.;
            let dest_size: Vec2 = Vec2::new(side_px, side_px);
            let draw_params: DrawTextureParams = DrawTextureParams {dest_size: Some(dest_size), ..Default::default()};

            let x_pos = x as f32 * (min_val / 8.) + x_offset;
            let y_pos = y as f32 * (min_val / 8.) + y_offset;

            if gs.movedfrom.is_some() {
                if gs.movedfrom.unwrap() == Vec2::new(x as f32, y as f32) {
                    draw_texture_ex(gs.movedfrom_tex, x_pos, y_pos, WHITE, draw_params.clone());
                }
            }

            if gs.movedto.is_some() {
                if gs.movedto.unwrap() == Vec2::new(x as f32, y as f32) {
                    draw_texture_ex(gs.movedfrom_tex, x_pos, y_pos, WHITE, draw_params.clone());
                }
            }

            if gs.selected.is_some() {
                if gs.selected.unwrap() == Vec2::new(x as f32, y as f32) {
                    draw_rectangle(x as f32 * (min_val / 8.) + x_offset, y as f32 * (min_val / 8.) + y_offset, min_val / 8., min_val / 8., gs.selected_color);
                }
            }

            match &gs.board[x][y] {
                types::Piece::Pawn(color) => {
                    draw_texture_ex(match color { types::PieceColor::Black => gs.bpawn_tex, _ => gs.wpawn_tex }, x_pos, y_pos, WHITE, draw_params);

                },
                types::Piece::Rook(color) => {
                    draw_texture_ex(match color { types::PieceColor::Black => gs.brook_tex, _ => gs.wrook_tex }, x_pos, y_pos, WHITE, draw_params);
                },
                types::Piece::Knight(color) => {
                    draw_texture_ex(match color { types::PieceColor::Black => gs.bknight_tex, _ => gs.wknight_tex }, x_pos, y_pos, WHITE, draw_params);
                },
                types::Piece::Bishop(color) => {
                    draw_texture_ex(match color { types::PieceColor::Black => gs.bbishop_tex, _ => gs.wbishop_tex }, x_pos, y_pos, WHITE, draw_params);
                },
                types::Piece::Queen(color) => {
                    draw_texture_ex(match color { types::PieceColor::Black => gs.bqueen_tex, _ => gs.wqueen_tex }, x_pos, y_pos, WHITE, draw_params);
                },
                types::Piece::King(color) => {
                    let current_equals = match color {
                        types::PieceColor::Black => match gs.current_color {
                            types::PieceColor::Black => true,
                            types::PieceColor::White => false
                        },
                        types::PieceColor::White => match gs.current_color {
                            types::PieceColor::Black => false,
                            types::PieceColor::White => true
                        }
                    };

                    if current_equals{
                        if gs.get_check(gs.current_color) {
                            draw_rectangle(x_pos, y_pos, min_val / 8., min_val / 8., gs.checked_color);
                        }
                    }

                    draw_texture_ex(match color { types::PieceColor::Black => gs.bking_tex, _ => gs.wking_tex }, x_pos, y_pos, WHITE, draw_params);
                },
                _ => {}
            }
        }
    }

    if gs.selected.is_some() {
        let available_moves = gs.possible_moves(gs.selected.unwrap().x as u8, gs.selected.unwrap().y as u8, true, &gs.current_color);

        for x in available_moves {
            draw_rectangle(x.x as f32 * (min_val / 8.) + x_offset, x.y as f32 * (min_val / 8.) + y_offset, min_val / 8., min_val / 8., gs.selected_color);
        }
    }

    let render_target = render_target(400, 400);
    render_target.texture.set_filter(FilterMode::Linear);

    set_camera(&Camera2D {
        zoom: vec2(0.005, 0.005),
        target: vec2(0.0, 0.0),
        render_target: Some(render_target),
        ..Default::default()
    });

    if gs.checkmate.is_none() {
        match gs.current_color {
            types::PieceColor::Black => draw_text("Black's turn", -190., -170., 32., gs.black_text_color),
            types::PieceColor::White => draw_text("White's turn", -190., -170., 32., gs.white_text_color),

        };
    }

    if gs.checkmate.is_some() {
        draw_text(match gs.checkmate.unwrap() {
            types::PieceColor::Black => "Black Wins!",
            types::PieceColor::White => "White Wins!"
        }, -190., -160., 64., WHITE);
        draw_text("Press to restart", -190., -130., 32., gs.white_text_color);
    }

    set_default_camera();

    draw_texture_ex(
        render_target.texture,
        0., 0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(400., 400.)),
            ..Default::default()
        }
    );

    draw_texture_ex(
        render_target.texture,
        screen_width(), screen_height(),
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(-400., -400.)),
            ..Default::default()
        }
    );
}
