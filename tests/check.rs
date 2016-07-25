extern crate chess;
extern crate env_logger;
use chess::*;

fn log() {
    let _ = env_logger::init();
}

#[test]
fn basic() {
    log();
    let mut game = Game::new_empty();
    game.set_at_pos((0,0), Some(&WHITE[5]));
    game.set_at_pos((1,1), Some(&BLACK[4]));

    assert!(game.in_check(Color::White));
}
