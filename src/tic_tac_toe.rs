use std::collections::HashMap;

pub type Player = u8;
pub type GamePos = [Player; 9];

pub const NONE: Player = 0;
pub const X: Player = 1;
pub const O: Player = 2;
const WINNING_PATTERNS: [[u8; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[allow(dead_code)]
fn find_winner(pos: &GamePos) -> Player {
    let mut player_count: u8 = 0;

    for i in 0..8 {
        for j in 0..3 {
            player_count += pos[WINNING_PATTERNS[i][j] as usize];
        }

        if player_count == X * 3 {
            return X;
        } else if player_count == O * 3 {
            return O;
        } else {
            player_count = 0;
        }
    }

    return NONE;
}

#[allow(dead_code)]
fn find_empty_spaces(pos: &GamePos) -> Vec<u8> {
    let mut empty_spaces: Vec<u8> = Vec::new();

    for i in 0..9 {
        if pos[i] == NONE {
            empty_spaces.push(i as u8);
        }
    }

    return empty_spaces;
}

#[allow(dead_code)]
fn find_player_winning_moves(pos: &GamePos, player: Player) -> Vec<u8> {
    let mut winning_moves: Vec<u8> = Vec::new();
    let mut player_count: u8 = 0;

    for i in 0..8 {
        for j in 0..3 {
            player_count += pos[WINNING_PATTERNS[i][j] as usize];
        }

        if player * 2 == player_count {
            for j in 0..3 {
                if pos[WINNING_PATTERNS[i][j] as usize] == NONE {
                    winning_moves.push(WINNING_PATTERNS[i][j] as u8);
                }
            }
        }

        player_count = 0;
    }

    return winning_moves;
}

// #[allow(dead_code)]
// fn rating_all_moves(pos: &GamePos, player: Player) -> HashMap<GamePos, u32> {
//     let mut rated_moves: HashMap<GamePos, u32> = HashMap::new();
//     let mut waiting_stack: Vec<GamePos> = vec![pos.clone()];
//     let in_prosess_pos: &GamePos = &waiting_stack[waiting_stack.len() - 1];

//     while !waiting_stack.is_empty() {

//     }

//     return rated_moves;
// }
