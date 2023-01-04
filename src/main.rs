use macroquad::prelude::*;

mod types;
mod parse;

fn get_check(board: &Vec<Vec<types::Piece>>, current: &types::PieceColor) -> bool {
    for (idx, _) in board.iter().enumerate() {
        for (idx2, _) in board[idx].iter().enumerate() {
            let moves = possible_moves(board, idx as u8, idx2 as u8, match current {
                types::PieceColor::Black => &types::PieceColor::White,
                _ => &types::PieceColor::Black
            }, false);

            for pmove in moves {
                if pmove.x >= 0. && pmove.x < 8. && pmove.y >= 0. && pmove.y < 8. {
                    if matches!(board[pmove.x as usize][pmove.y as usize], types::Piece::King(_)) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn get_color(board: &Vec<Vec<types::Piece>>, x: u8, y: u8) -> Option<&types::PieceColor> {
    if !((x as i8 >= 0 && x < 8) && (y as i8 >= 0 && y < 8)) {
        return None;
    }

     match &board[x as usize][y as usize] {
        types::Piece::Pawn(_color) | types::Piece::Rook(_color) | types::Piece::Knight(_color) | types::Piece::Bishop(_color) | types::Piece::Queen(_color) | types::Piece::King(_color) => {
            Some(_color)
        },
        types::Piece::Empty => None
    }

}

fn different_color(board: &Vec<Vec<types::Piece>>, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
    if get_color(&board, x1, y1).is_none() && get_color(&board, x2, y2).is_none() {
        return false;
    }
    if get_color(&board, x2, y2).is_none() {
        return true;
    }
    if get_color(&board, x1, y1).is_none() {
        return true;
    }

    match get_color(&board, x1, y1).unwrap() {
        types::PieceColor::Black => {
            match get_color(&board, x2, y2).unwrap() {
                types::PieceColor::Black => false,
                types::PieceColor::White => true
            }
        },
        types::PieceColor::White => {
            match get_color(&board, x2, y2).unwrap() {
                types::PieceColor::Black => true,
                types::PieceColor::White => false
            }
        }
    }
}

fn possible_moves(board: &Vec<Vec<types::Piece>>, x: u8, y: u8, current: &types::PieceColor, remove: bool) -> Vec<Vec2> {
    match &board[x as usize][y as usize] {
        types::Piece::Pawn(_color) | types::Piece::Rook(_color) | types::Piece::Knight(_color) | types::Piece::Bishop(_color) | types::Piece::Queen(_color) | types::Piece::King(_color) => {
            if matches!(*current, types::PieceColor::Black) && matches!(*_color, types::PieceColor::White) {
                return vec![];
            }
            if matches!(*current, types::PieceColor::White) && matches!(*_color, types::PieceColor::Black) {
                return vec![];
            }
        },
        types::Piece::Empty => {}
    }

    let mut possible: Vec<Vec2> = vec![];

    match &board[x as usize][y as usize] {
        types::Piece::Pawn(color) => {
            let y_offset: i8 = if matches!(color, types::PieceColor::Black) {1} else {-1}; 

            let is_at_start: bool = (matches!(color, types::PieceColor::White) && y == 6) || (matches!(color, types::PieceColor::Black) && y == 1);

            if y as i8 + y_offset * 2 >= 0 && y as i8 + y_offset * 2 < 8 {
                if matches!(&board[x as usize][(y as i8 + y_offset * 2) as usize], types::Piece::Empty) && matches!(&board[x as usize][(y as i8 + y_offset) as usize], types::Piece::Empty) && is_at_start {
                    possible.push(Vec2::new(x as f32, y as f32 + y_offset as f32 * 2.));
                }
            }

            if y as i8 + y_offset >= 0 {
                if matches!(&board[x as usize][(y as i8 + y_offset) as usize], types::Piece::Empty) {
                    possible.push(Vec2::new(x as f32, y as f32 + y_offset as f32));
                }
            }

            for x_offset in [-1, 1] {
                if !(x as i8 + x_offset >= 0 && x as i8 + x_offset < 8 && y as i8 + y_offset >= 0 && y as i8 + y_offset < 8) {
                    continue;
                }

                if !matches!(&board[(x as i8 + x_offset) as usize][(y as i8 + y_offset) as usize], types::Piece::Empty) {

                    let diff = different_color(&board, x, y, (x as i8 + x_offset) as u8, (y as i8 + y_offset) as u8);
                    if diff {
                        possible.push(Vec2::new((x as i8 + x_offset) as f32, (y as i8 + y_offset) as f32));
                    }
                }
            }
        },
        types::Piece::Knight(_) => {
            if (x as i8 - 2 >= 0) && (y as i8 - 1 >= 0) {
                if different_color(&board, x, y, x - 2, y - 1) {
                possible.push(Vec2::new(x as f32 - 2., y as f32 - 1.));
                }
            }
            if (x as i8 - 1 >= 0) && (y as i8 - 2 >= 0) {
                if different_color(&board, x, y, x - 1, y - 2) {
                    possible.push(Vec2::new(x as f32 - 1., y as f32 - 2.));
                }
            }

            if (x + 1 < 8) && (y as i8 - 2 >= 0) {
                if different_color(&board, x, y, x + 1, y - 2) {
                    possible.push(Vec2::new(x as f32 + 1., y as f32 - 2.));
                }
            }
            if (x + 2 < 8) && (y as i8 - 1 >= 0) {
                if different_color(&board, x, y, x + 2, y - 1) {
                    possible.push(Vec2::new(x as f32 + 2., y as f32 - 1.));
                }
            }

            if (x + 2 < 8) && (y + 1 < 8) {
                if different_color(&board, x, y, x + 2, y + 1) {
                    possible.push(Vec2::new(x as f32 + 2., y as f32 + 1.));
                }
            }
            if (x + 1 < 8) && (y + 2 < 8) {
                if different_color(&board, x, y, x + 1, y + 2) {
                    possible.push(Vec2::new(x as f32 + 1., y as f32 + 2.));
                }
            }

            if (x as i8 - 1 >= 0) && (y as i8 + 2 >= 0) {
                if different_color(&board, x, y, x - 1, y + 2) {
                    possible.push(Vec2::new(x as f32 - 1., y as f32 + 2.));
                }
            }
            if (x as i8 - 2 >= 0) && (y + 1 < 8) {
                if different_color(&board, x, y, x - 2, y + 1) {
                    possible.push(Vec2::new(x as f32 - 2., y as f32 + 1.));
                }
            }

        },
        types::Piece::Rook(_) => {
            for i in (0..x).rev() {
                if get_color(&board, i as u8, y).is_some() {
                    if different_color(&board, x, y, i, y) {
                        possible.push(Vec2::new(i as f32, y as f32));
                    }
                    break;
                }
                possible.push(Vec2::new(i as f32, y as f32));
            }

            for i in (x + 1)..8 {
                if get_color(&board, i as u8, y).is_some() {
                    if different_color(&board, x, y, i, y) {
                        possible.push(Vec2::new(i as f32, y as f32));
                    }
                    break;
                }
                possible.push(Vec2::new(i as f32, y as f32));
            }

            for i in (0..y).rev() {
                if get_color(&board, x, i as u8).is_some() {
                    if different_color(&board, x, y, x, i) {
                        possible.push(Vec2::new(x as f32, i as f32));
                    }
                    break;
                }
                possible.push(Vec2::new(x as f32, i as f32));
            }

            for i in (y + 1)..8 {
                if get_color(&board, x, i as u8).is_some() {
                    if different_color(&board, x, y, x, i) {
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
                if get_color(&board, x + i, y + i).is_some() {
                    if different_color(&board, x, y, x + i, y + i) {
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
                if get_color(&board, x - i as u8, y - i as u8).is_some() {
                    if different_color(&board, x, y, x - i as u8, y - i as u8) {
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
                if get_color(&board, x + i as u8, y - i as u8).is_some() {
                    if different_color(&board, x, y, x + i as u8, y - i as u8) {
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
                if get_color(&board, x - i as u8, y + i as u8).is_some() {
                    if different_color(&board, x, y, x - i as u8, y + i as u8) {
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
                if different_color(&board, x, y, x + 1, y + 1) {
                    possible.push(Vec2::new((x + i) as f32, (y + i) as f32));
                }
            } if y + 1 < 8 {
                if different_color(&board, x, y, x, y + i) {
                    possible.push(Vec2::new((x) as f32, (y + i) as f32));
                }
            } if x as i8 - 1 >= 0 && y + 1 < 8 {
                if different_color(&board, x, y, x - 1, y + 1) {
                    possible.push(Vec2::new((x - i) as f32, (y + i) as f32));
                }
            }

            if x + 1 < 8 {
                if different_color(&board, x, y, x + 1, y) {
                    possible.push(Vec2::new((x + i) as f32, (y) as f32));
                }
            } if x as i8 - 1 >= 0 {
                if different_color(&board, x, y, x - 1, y) {
                    possible.push(Vec2::new((x - i) as f32, (y) as f32));
                }
            }

            if x + 1 < 8 && y as i8 - 1 >= 0 {
                if different_color(&board, x, y, x + 1, y - 1) {
                    possible.push(Vec2::new((x + i) as f32, (y - i) as f32));
                }
            } if y as i8 - 1 >= 0 {
                if different_color(&board, x, y, x, y - 1) {
                    possible.push(Vec2::new((x) as f32, (y - i) as f32));
                }
            } if x as i8 - 1 >= 0 && y as i8 - 1 >= 0 {
                if different_color(&board, x, y, x - 1, y - 1) {
                    possible.push(Vec2::new((x - i) as f32, (y - i) as f32));
                }
            }
        },
        types::Piece::Queen(_) => {
            for i in 1..8 {
                if !(x as i8 + i < 8 && y as i8 + i < 8) {
                    continue;
                }
                if get_color(&board, x + i as u8, y + i as u8).is_some() {
                    if different_color(&board, x, y, x + i as u8, y + i as u8) {
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
                if get_color(&board, x - i as u8, y - i as u8).is_some() {
                    if different_color(&board, x, y, x - i as u8, y - i as u8) {
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
                if get_color(&board, x + i as u8, y - i as u8).is_some() {
                    if different_color(&board, x, y, x + i as u8, y - i as u8) {
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
                if get_color(&board, x - i as u8, y + i as u8).is_some() {
                    if different_color(&board, x, y, x - i as u8, y + i as u8) {
                        possible.push(Vec2::new((x - i as u8) as f32, (y + i as u8) as f32));
                    }
                    break;
                }
                possible.push(Vec2::new((x - i as u8) as f32, (y + i as u8) as f32));
            }

            for i in (0..x).rev() {
                if get_color(&board, i as u8, y).is_some() {
                    if different_color(&board, x, y, i, y) {
                        possible.push(Vec2::new(i as f32, y as f32));
                    }
                    break;
                }
                possible.push(Vec2::new(i as f32, y as f32));
            }

            for i in (x + 1)..8 {
                if get_color(&board, i as u8, y).is_some() {
                    if different_color(&board, x, y, i, y) {
                        possible.push(Vec2::new(i as f32, y as f32));
                    }
                    break;
                }
                possible.push(Vec2::new(i as f32, y as f32));
            }

            for i in (0..y).rev() {
                if get_color(&board, x, i as u8).is_some() {
                    if different_color(&board, x, y, x, i) {
                        possible.push(Vec2::new(x as f32, i as f32));
                    }
                    break;
                }
                possible.push(Vec2::new(x as f32, i as f32));
            }

            for i in (y + 1)..8 {
                if get_color(&board, x, i as u8).is_some() {
                    if different_color(&board, x, y, x, i) {
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
            let mut new_board: Vec<Vec<types::Piece>> = board.clone();
    
            if x as i8 >= 0 && x < 8 && y as i8 >= 0 && y < 8 && possible[idx].x >= 0. && possible[idx].x < 8. && possible[idx].y >= 0. && possible[idx].y < 8. {
                new_board[possible[idx].x as usize][possible[idx].y as usize] = board[x as usize][y as usize];
                new_board[x as usize][y as usize] = types::Piece::Empty;
            }
    
            if get_check(&new_board, current) {
                possible.remove(idx);
            }
        }
    }

    possible
}

#[macroquad::main("Chess")]
async fn main() {

    //let background_color: Color = Color::from_rgba(251, 240, 240, 255);
    //let white_color: Color = Color::from_rgba(251, 240, 240, 255);
    //let black_color: Color = Color::from_rgba(124, 117, 117, 255);

    let mut checkmate: Option<types::PieceColor> = None;

    let white_color: Color = Color::from_rgba(98, 104, 128, 255);
    let background_color: Color = Color::from_rgba(48, 52, 70, 255);
    let black_color: Color = Color::from_rgba(48, 52, 70, 255);
    let checked_color: Color = Color::from_rgba(255, 89, 123, 120); 
    let selected_color: Color = Color::from_rgba(163, 187, 152, 120);

    let text_color: Color = Color::from_rgba(0, 0, 0, 60);

    let fenstring_start: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    //let fenstring_start: &str = "4k3/8/8/8/8/8/8/QQQQKQQQ";

    let arg = std::env::args().collect::<Vec<String>>();
    let mut board: Vec<Vec<types::Piece>> = parse::parse_fen(if std::env::args().len() >= 2 {arg[1].as_str()} else {fenstring_start});

    let wking_tex = load_texture("res/wking.png").await.unwrap();
    let wqueen_tex = load_texture("res/wqueen.png").await.unwrap();
    let wbishop_tex = load_texture("res/wbishop.png").await.unwrap();
    let wknight_tex = load_texture("res/wknight.png").await.unwrap();
    let wrook_tex = load_texture("res/wrook.png").await.unwrap();
    let wpawn_tex = load_texture("res/wpawn.png").await.unwrap();

    let bking_tex = load_texture("res/bking.png").await.unwrap();
    let bqueen_tex = load_texture("res/bqueen.png").await.unwrap();
    let bbishop_tex = load_texture("res/bbishop.png").await.unwrap();
    let bknight_tex = load_texture("res/bknight.png").await.unwrap();
    let brook_tex = load_texture("res/brook.png").await.unwrap();
    let bpawn_tex = load_texture("res/bpawn.png").await.unwrap();

    let movedfrom_tex = load_texture("res/movedfrom.png").await.unwrap();

    let mut selected: Option<Vec2> = None;
    let mut last_selected: Option<Vec2> = None;

    let mut current_color: types::PieceColor = types::PieceColor::White;

    loop {
        clear_background(background_color);

        let min_val: f32 = if screen_width() > screen_height() {screen_height()} else {screen_width()};

        let x_offset: f32 = if screen_width() > screen_height() {(screen_width() - min_val) / 2.} else {0.};
        let y_offset: f32 = if screen_height() > screen_width() {(screen_height() - min_val) / 2.} else {0.};

        if is_mouse_button_pressed(MouseButton::Right) && checkmate.is_some() {
            checkmate = None;
            board = parse::parse_fen(fenstring_start);
            selected = None;
            last_selected = None;
            current_color = types::PieceColor::White;
        }

        if is_mouse_button_pressed(MouseButton::Left) && checkmate.is_none() {
            let mut new_x = mouse_position().0;
            let mut new_y = mouse_position().1;

            new_x -= x_offset;
            new_y -= y_offset;

            new_x = (new_x / (min_val / 8.)).floor() * (min_val / 8.) / (min_val / 8.);
            new_y = (new_y / (min_val / 8.)).floor() * (min_val / 8.) / (min_val / 8.);

            println!("({}, {})", new_x, new_y);

            if selected.is_none()  {
                if new_x >= 0. && new_x < 8. && new_y >= 0. && new_y < 8. {
                    selected = Some(Vec2::new(new_x, new_y));
                    last_selected = None;
                }
            } else {
                if !possible_moves(&board, selected.unwrap().x as u8, selected.unwrap().y as u8, &current_color, true).contains(&Vec2::new(new_x, new_y)) {
                    selected = None;
                    continue;
                }
                
                board[new_x as usize][new_y as usize] = board[selected.unwrap().x as usize][selected.unwrap().y as usize];
                board[selected.unwrap().x as usize][selected.unwrap().y as usize] = types::Piece::Empty;

                if ((new_y == 7. && matches!(current_color, types::PieceColor::Black)) || (new_y == 0. && matches!(current_color, types::PieceColor::White))) && matches!(board[new_x as usize][new_y as usize], types::Piece::Pawn(_)) {
                    board[new_x as usize][new_y as usize] = types::Piece::Queen(current_color);
                }

                let mut black_possible = 0;
                let mut white_possible = 0;

                for x in 0..board.len() {
                    for y in 0..board[x].len() {
                        match &board[x][y] {
                            types::Piece::Pawn(_color) | types::Piece::Rook(_color) | types::Piece::Knight(_color) | types::Piece::Bishop(_color) | types::Piece::Queen(_color) | types::Piece::King(_color) => {
                                if matches!(_color, types::PieceColor::Black) {
                                    let col = types::PieceColor::Black;
                                    black_possible += possible_moves(&board, x as u8, y as u8, &col, true).len();
                                } else {
                                    let col = types::PieceColor::White;
                                    white_possible += possible_moves(&board, x as u8, y as u8, &col, true).len();
                                }
                            },
                            types::Piece::Empty => {}
                        }
                    }
                }

                if black_possible == 0 {
                    checkmate = Some(types::PieceColor::White);
                }
                if white_possible == 0 {
                    checkmate = Some(types::PieceColor::Black);
                }

                current_color = match current_color {
                    types::PieceColor::White => types::PieceColor::Black,
                    types::PieceColor::Black => types::PieceColor::White
                };

                last_selected = Some(Vec2::new(selected.unwrap().x, selected.unwrap().y));
                selected = Some(Vec2::new(new_x, new_y));
            }
        }

        for x in 0..8 {
            for y in 0..8 {
                let selected_unwrap = if selected.is_some() {selected.unwrap()} else {Vec2::new(-1., -1.)};
                draw_rectangle(x as f32 * (min_val / 8.) + x_offset, y as f32 * (min_val / 8.) + y_offset, min_val / 8., min_val / 8., if (x+y)%2 == 0 {white_color} else {black_color});

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

                if last_selected.is_some() {
                    if last_selected.unwrap() == Vec2::new(x as f32, y as f32) {
                        draw_texture_ex(movedfrom_tex, x_pos, y_pos, WHITE, draw_params.clone());
                    }
                }

                if selected.is_some() {
                    if selected.unwrap() == Vec2::new(x as f32, y as f32) {
                        draw_rectangle(x as f32 * (min_val / 8.) + x_offset, y as f32 * (min_val / 8.) + y_offset, min_val / 8., min_val / 8., selected_color);
                    }
                }

                match &board[x][y] {
                    types::Piece::Pawn(color) => {
                        draw_texture_ex(match color { types::PieceColor::Black => bpawn_tex, _ => wpawn_tex }, x_pos, y_pos, WHITE, draw_params);

                    },
                    types::Piece::Rook(color) => {
                        draw_texture_ex(match color { types::PieceColor::Black => brook_tex, _ => wrook_tex }, x_pos, y_pos, WHITE, draw_params);
                    },
                    types::Piece::Knight(color) => {
                        draw_texture_ex(match color { types::PieceColor::Black => bknight_tex, _ => wknight_tex }, x_pos, y_pos, WHITE, draw_params);
                    },
                    types::Piece::Bishop(color) => {
                        draw_texture_ex(match color { types::PieceColor::Black => bbishop_tex, _ => wbishop_tex }, x_pos, y_pos, WHITE, draw_params);
                    },
                    types::Piece::Queen(color) => {
                        draw_texture_ex(match color { types::PieceColor::Black => bqueen_tex, _ => wqueen_tex }, x_pos, y_pos, WHITE, draw_params);
                    },
                    types::Piece::King(color) => {
                        let current_equals = match color {
                            types::PieceColor::Black => match current_color {
                                types::PieceColor::Black => true,
                                types::PieceColor::White => false
                            },
                            types::PieceColor::White => match current_color {
                                types::PieceColor::Black => false,
                                types::PieceColor::White => true
                            }
                        };

                        if current_equals{
                            if get_check(&board, &current_color) {
                                draw_rectangle(x_pos, y_pos, min_val / 8., min_val / 8., checked_color);
                            }
                        }

                        draw_texture_ex(match color { types::PieceColor::Black => bking_tex, _ => wking_tex }, x_pos, y_pos, WHITE, draw_params);
                    },
                    _ => {}
                }
            }
        }

        if selected.is_some() {
            let available_moves = possible_moves(&board, selected.unwrap().x as u8, selected.unwrap().y as u8, &current_color, true);

            for x in available_moves {
                draw_rectangle(x.x as f32 * (min_val / 8.) + x_offset, x.y as f32 * (min_val / 8.) + y_offset, min_val / 8., min_val / 8., selected_color);
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
    
        if checkmate.is_none() {
            match current_color {
                types::PieceColor::Black => draw_text("Black's turn", -190., -170., 32., BLACK),
                types::PieceColor::White => draw_text("White's turn", -190., -170., 32., WHITE),

            };
        }

        if checkmate.is_some() {
            draw_text(match checkmate.unwrap() {
                types::PieceColor::Black => "Black Wins!",
                types::PieceColor::White => "White Wins!"
            }, -190., -160., 64., WHITE);
            draw_text("Press to restart", -190., -130., 32., WHITE);
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

        next_frame().await
    }
}
