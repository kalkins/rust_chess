extern crate chess;
extern crate env_logger;
use chess::*;

fn log() {
    let _ = env_logger::init();
}

#[test]
fn victory_bug_03c53f() {
    log();
    let mut game = Game::new();
    game.move_piece((1,0), (0,2));

    assert_eq!(game.check_victory(), None);
}
