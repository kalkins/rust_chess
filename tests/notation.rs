extern crate chess;
extern crate env_logger;
use chess::*;

fn log() {
    let _ = env_logger::init();
}

#[test]
fn both() {
    log();
    let mut game = Game::new();
    // Start by moving a knight from B1 to C3.
    let mut m = game.an_to_move("Nc3", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Nc3");
    assert_eq!(m, Some(vec![((1, 0), (2, 2))]));
    game.move_pieces(&m.unwrap());
    
    // Move a black pawn from D7 to D5.
    m = game.an_to_move("d5", Color::Black);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "d5");
    assert_eq!(m, Some(vec![((3, 6), (3, 4))]));
    game.move_pieces(&m.unwrap());
    
    m = game.an_to_move("e4", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "e4");
    assert_eq!(m, Some(vec![((4, 1), (4, 3))]));
    game.move_pieces(&m.unwrap());
    
    // Now the pawn at D5 can capture the pawn at E4.
    m = game.an_to_move("dxe4", Color::Black);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "dxe4");
    assert_eq!(m, Some(vec![((3, 4), (4, 3))]));
    // Abbreviated notation is also valid.
    assert_eq!(m, game.an_to_move("de4", Color::Black));
    assert_eq!(game.move_to_an(&game.an_to_move("de4", Color::Black).unwrap(), true), "dxe4");
    assert_eq!(m, game.an_to_move("de", Color::Black));
    assert_eq!(game.move_to_an(&game.an_to_move("de", Color::Black).unwrap(), true), "dxe4");
    game.move_pieces(&m.unwrap());
    
    // Fast-forwards a little.
    m = game.an_to_move("Nf3", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Nf3");
    game.move_pieces(&m.unwrap());
    m = game.an_to_move("Ng5", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Ng5");
    game.move_pieces(&m.unwrap());
    
    // Now both white knights can reach E4, so "Ne4" isn't enough.
    m = game.an_to_move("Ne4", Color::White);
    assert_eq!(m, None);
    
    // ...so we must specify the file the knight is moving from.
    m = game.an_to_move("Ncxe4", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Ncxe4");
    assert_eq!(m, Some(vec![((2, 2), (4, 3))]));
    
    // We could also specify the rank, or both the rank and the file.
    assert_eq!(m, game.an_to_move("N3e4", Color::White));
    assert_eq!(game.move_to_an(&game.an_to_move("N3e4", Color::White).unwrap(), true), "Ncxe4");
    assert_eq!(m, game.an_to_move("Nc3e4", Color::White));
    assert_eq!(game.move_to_an(&game.an_to_move("Nc3e4", Color::White).unwrap(), true), "Ncxe4");
    game.move_pieces(&m.unwrap());
    
    // Fast forwards some more.
    m = game.an_to_move("Qf3", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Qf3");
    game.move_pieces(&m.unwrap());
    m = game.an_to_move("Be2", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Be2");
    game.move_pieces(&m.unwrap());
    m = game.an_to_move("b3", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "b3");
    game.move_pieces(&m.unwrap());
    m = game.an_to_move("Bb2", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "Bb2");
    game.move_pieces(&m.unwrap());
    
    // Kingside castling.
    m = game.an_to_move("0-0", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "0-0");
    assert_eq!(m, Some(vec![((4, 0), (5, 0)), ((5, 0), (6, 0)), ((7, 0), (5, 0))]));
    
    // Queenside castling.
    m = game.an_to_move("0-0-0", Color::White);
    assert_eq!(game.move_to_an(&m.as_ref().unwrap(), true), "0-0-0");
    assert_eq!(m, Some(vec![((4, 0), (3, 0)), ((3, 0), (2, 0)), ((0, 0), (3, 0))]));
}
