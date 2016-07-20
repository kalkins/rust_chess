extern crate chess;
extern crate env_logger;
use chess::*;

fn log() {
    let _ = env_logger::init();
}

fn setup(pos: (usize, usize), piece: &Piece) -> Vec<Vec<((usize, usize), (usize, usize))>> {
    let mut game = Game::new_empty();
    game.ignore_kings(true);
    game.set_at_pos(pos, Some(piece));
    game.valid_moves(pos)
}

#[test]
fn pawn() {
	log();
    let moves = setup((3,3), &WHITE[0]);
    assert_eq!(moves.len(), 1);
}

#[test]
fn rook() {
	log();
    let moves = setup((3,3), &WHITE[1]);
    assert_eq!(moves.len(), 14);
}

#[test]
fn knight() {
	log();
    let moves = setup((3,3), &WHITE[2]);
    assert_eq!(moves.len(), 8);
}

#[test]
fn bishop() {
	log();
    let moves = setup((3,3), &WHITE[3]);
    assert_eq!(moves.len(), 13);
}

#[test]
fn queen() {
	log();
    let moves = setup((3,3), &WHITE[4]);
    assert_eq!(moves.len(), 27);
}

#[test]
fn king() {
	log();
    let moves = setup((3,3), &WHITE[5]);

    assert_eq!(moves.len(), 8);

    let mut game = Game::new_empty();
    game.set_at_pos((1, 2), Some(&WHITE[4]));
    game.set_at_pos((0, 0), Some(&BLACK[5]));
    game.set_at_pos((6, 7), Some(&WHITE[5]));

    assert_eq!(game.valid_moves((0,0)).len(), 0);
}
