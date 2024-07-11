pub type Player = u8;
pub type GamePos = [Player; 9];

pub const NONE: Player = 1;
pub const X: Player = 2;
pub const O: Player = 3;

#[allow(dead_code)]
struct GameReport {
    game_pos: GamePos,
    empty_spaces: Vec<u8>,
    winner: Player,
    x_winning_moves: Vec<u8>,
    o_winning_moves: Vec<u8>,
}

#[allow(dead_code)]
fn create_id(pos: &GamePos) -> u32 {
    let mut id: u32 = 0;

    for i in 0..pos.len() {
        id += (pos[i] as u32) * 10_u32.pow(i as u32);
    }

    return id;
}

#[allow(dead_code)]
fn find_winner(pos: &GamePos) -> Player {
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
    let mut player_count: u8 = 0;

    for i in 0..8 {
        for j in 0..3 {
            player_count += pos[WINNING_PATTERNS[i][j] as usize];
        }

        match player_count {
            6 => return X,
            9 => return O,
            _ => player_count = 0,
        }
    }

    return NONE;
}

#[allow(dead_code)]
fn find_empty_spaces(pos: &GamePos) -> Vec<u8> {
    let mut empty_spaces: Vec<u8> = vec![];

    for i in 0..9 {
        if pos[i] == NONE {
            empty_spaces.push(i as u8);
        }
    }

    return empty_spaces;
}

#[allow(dead_code)]
fn find_player_winning_moves(pos: &GamePos, player: Player) -> Vec<u8> {
    let mut winning_moves: Vec<u8> = vec![];
    let empty_spaces: Vec<u8> = find_empty_spaces(&pos);

    for i in 0..empty_spaces.len() {
        let mut cloned_pos: GamePos = pos.clone();

        cloned_pos[i] = player;

        if find_winner(&cloned_pos) == player {
            winning_moves.push(i as u8);
        }
    }

    return winning_moves;
}

#[allow(dead_code)]
fn build_game_report(pos: &GamePos) -> GameReport {
    return GameReport {
        game_pos: pos.clone(),
        empty_spaces: find_empty_spaces(&pos),
        winner: find_winner(&pos),
        x_winning_moves: find_player_winning_moves(&pos, X),
        o_winning_moves: find_player_winning_moves(&pos, O),
    };
}
