mod tic_tac_toe;
#[allow(unused_imports)]
use tic_tac_toe::{Player, GamePos, NONE, O, X, rate_moves};

fn main() {
    let mut game_pos: GamePos = [X, NONE, NONE, NONE, O, NONE, X, NONE, O];
    println!("{:?}", rate_moves(&game_pos, X));
}   
