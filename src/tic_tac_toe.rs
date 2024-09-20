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
fn enemy(player: Player) -> Player {
    if player == X {
        return O;
    } else if player == O {
        return X;
    } else {
        return NONE;
    }
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
fn find_winning_moves(pos: &GamePos, player: Player) -> Vec<u8> {
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

#[allow(dead_code)]
fn rate_potential(pos: &GamePos, player: Player, added_rating: u8) -> [u8; 9] {
    let mut rated_moves: [u8; 9] = [0; 9];

    for i in 0..8 {
        let mut player_count: u8 = 0;

        for j in 0..3 {
            if pos[WINNING_PATTERNS[i][j] as usize] == player {
                player_count += 1;
            }

            if pos[WINNING_PATTERNS[i][j] as usize] == enemy(player) {
                player_count = 0;
                break;
            }
        }

        for j in 0..3 {
            rated_moves[WINNING_PATTERNS[i][j] as usize] += added_rating * player_count;
        }
    }

    return rated_moves;
}

#[allow(dead_code)]
pub fn rate_moves(pos: &GamePos, player: Player) -> [u8; 9] {
    let mut rated_moves: [u8; 9] = [2, 1, 2, 1, 3, 1, 2, 1, 2];
    let rate_1 = rate_potential(&pos, player, 2);
    let rate_2 = rate_potential(&pos, enemy(player), 1);

    for i in 0..9 {
        rated_moves[i] += rate_1[i] + rate_2[i];

        if pos[i] != NONE {
            rated_moves[i] = 0;
        }
    }

    return rated_moves;
}
