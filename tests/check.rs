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

#[test]
fn case1() {
    log();
    let mut game = Game::new_empty();
    game.set_at_pos((0,5), Some(&BLACK[5]));
    game.set_at_pos((0,3), Some(&WHITE[0]));
    game.set_at_pos((2,3), Some(&WHITE[0]));
    game.set_at_pos((1,4), Some(&WHITE[2]));
    game.set_at_pos((1,1), Some(&WHITE[5]));

    for v in game.valid_moves((0,5)) {
        assert!(v[0].1 != (1,4));
        assert!(v[0].1 != (0,6));
    }

    game.move_piece((0,5), (1,4));
    assert!(game.in_check(Color::Black));
}
