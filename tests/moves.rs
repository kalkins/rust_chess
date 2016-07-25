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

    let mut game = Game::new_empty();
    game.ignore_kings(true);
    game.set_at_pos((4, 6), Some(&WHITE[0]));
    game.move_piece((4, 6), (4, 7));
    assert_eq!(game.get_from_pos((4, 7)), Some(&WHITE[4]));

    game.set_at_pos((2, 1), Some(&BLACK[0]));
    game.move_piece((2, 1), (2, 0));
    assert_eq!(game.get_from_pos((2, 0)), Some(&BLACK[4]));

    // En passant
    game.clear();
    game.set_at_pos((4, 6), Some(&BLACK[0]));
    game.set_at_pos((5, 4), Some(&WHITE[0]));
    for v in game.valid_moves((4, 6)) {
        if let Some(m) = v.last() {
            if (m.1).0 == 4 && (m.1).1 == 4 {
                game.move_pieces(&v);
                break;
            }
        }
    }
    for v in game.valid_moves((5, 4)) {
        if let Some(m) = v.last() {
            if (m.1).0 == 4 && (m.1).1 == 5 {
                assert_eq!(game.move_pieces(&v), Some(&BLACK[0]));
                break;
            }
        }
    }
    assert_eq!(game.get_from_pos((4, 4)), None);
    assert_eq!(game.get_from_pos((4, 5)), Some(&WHITE[0]));

    game.clear();
    game.set_at_pos((4, 4), Some(&BLACK[0]));
    game.set_at_pos((5, 4), Some(&WHITE[0]));
    for v in game.valid_moves((5, 4)) {
        if let Some(m) = v.last() {
            if (m.1).0 == 4 && (m.1).1 == 5 {
                assert!(false);
            }
        }
    }
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

    let mut game = Game::new();
    assert_eq!(game.valid_moves((3,0)).len(), 0);
    game.set_at_pos((1,0), None);
    game.set_at_pos((2,0), None);
    assert_eq!(game.valid_moves((3,0)).len(), 2);

    game.set_at_pos((3,1), None);
    assert_eq!(game.valid_moves((3,0)).len(), 8);
    game.move_piece((4,0), (3,1));
    assert_eq!(game.valid_moves((3,0)).len(), 3);
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

    let mut found = false;
    game = Game::new();
    game.set_at_pos((5, 0), None);

    for v in game.valid_moves((4, 0)) {
        assert_eq!(v.len(), 1);
    }

    game.set_at_pos((6, 0), None);
    for v in game.valid_moves((4, 0)) {
        if v.len() == 3 {
            found = true;
            break;
        }
    }
    assert!(found);

    found = false;
    game.set_at_pos((3, 0), None);
    game.set_at_pos((2, 0), None);
    game.set_at_pos((6, 0), Some(&WHITE[2]));
    for v in game.valid_moves((4, 0)) {
        assert_eq!(v.len(), 1);
    }

    game.set_at_pos((1, 0), None);
    for v in game.valid_moves((4, 0)) {
        if v.len() == 3 {
            found = true;
            break;
        }
    }
    assert!(found);

    game.set_at_pos((3, 1), None);
    game.set_at_pos((3, 5), Some(&BLACK[4]));
    for v in game.valid_moves((4, 0)) {
        assert_eq!(v.len(), 1);
    }
}
