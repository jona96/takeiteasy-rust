use std::collections::HashSet;

use ai::AI;
use takeiteasy::*;

fn get_full_board() -> Board {

    let mut board = Board::new();
    assert!(board.place_tile(field!(1, 1), tile!(9, 7, 3)).is_ok());
    assert!(board.place_tile(field!(1, 2), tile!(9, 2, 4)).is_ok());
    assert!(board.place_tile(field!(1, 3), tile!(9, 2, 3)).is_ok());

    assert!(board.place_tile(field!(2, 1), tile!(5, 6, 8)).is_ok());
    assert!(board.place_tile(field!(2, 2), tile!(5, 2, 8)).is_ok());
    assert!(board.place_tile(field!(2, 3), tile!(1, 2, 4)).is_ok());
    assert!(board.place_tile(field!(2, 4), tile!(5, 6, 3)).is_ok());

    assert!(board.place_tile(field!(3, 1), tile!(1, 6, 3)).is_ok());
    assert!(board.place_tile(field!(3, 2), tile!(1, 2, 8)).is_ok());
    assert!(board.place_tile(field!(3, 3), tile!(9, 6, 4)).is_ok());
    assert!(board.place_tile(field!(3, 4), tile!(1, 7, 4)).is_ok());
    assert!(board.place_tile(field!(3, 5), tile!(1, 7, 3)).is_ok());

    assert!(board.place_tile(field!(4, 1), tile!(1, 2, 3)).is_ok());
    assert!(board.place_tile(field!(4, 2), tile!(1, 6, 8)).is_ok());
    assert!(board.place_tile(field!(4, 3), tile!(1, 7, 8)).is_ok());
    assert!(board.place_tile(field!(4, 4), tile!(5, 7, 4)).is_ok());

    assert!(board.place_tile(field!(5, 1), tile!(5, 2, 3)).is_ok());
    assert!(board.place_tile(field!(5, 2), tile!(5, 7, 8)).is_ok());
    assert!(board.place_tile(field!(5, 3), tile!(5, 7, 3)).is_ok());

    board
}

fn get_almost_full_board() -> Board {

    // tile on field 5,3 is missing
    let mut board = Board::new();
    assert!(board.place_tile(field!(1, 1), tile!(9, 7, 3)).is_ok());
    assert!(board.place_tile(field!(1, 2), tile!(9, 2, 4)).is_ok());
    assert!(board.place_tile(field!(1, 3), tile!(9, 2, 3)).is_ok());

    assert!(board.place_tile(field!(2, 1), tile!(5, 6, 8)).is_ok());
    assert!(board.place_tile(field!(2, 2), tile!(5, 2, 8)).is_ok());
    assert!(board.place_tile(field!(2, 3), tile!(1, 2, 4)).is_ok());
    assert!(board.place_tile(field!(2, 4), tile!(5, 6, 3)).is_ok());

    assert!(board.place_tile(field!(3, 1), tile!(1, 6, 3)).is_ok());
    assert!(board.place_tile(field!(3, 2), tile!(1, 2, 8)).is_ok());
    assert!(board.place_tile(field!(3, 3), tile!(9, 6, 4)).is_ok());
    assert!(board.place_tile(field!(3, 4), tile!(1, 7, 4)).is_ok());
    assert!(board.place_tile(field!(3, 5), tile!(1, 7, 3)).is_ok());

    assert!(board.place_tile(field!(4, 1), tile!(1, 2, 3)).is_ok());
    assert!(board.place_tile(field!(4, 2), tile!(1, 6, 8)).is_ok());
    assert!(board.place_tile(field!(4, 3), tile!(1, 7, 8)).is_ok());
    assert!(board.place_tile(field!(4, 4), tile!(5, 7, 4)).is_ok());

    assert!(board.place_tile(field!(5, 1), tile!(5, 2, 3)).is_ok());
    assert!(board.place_tile(field!(5, 2), tile!(5, 7, 8)).is_ok());
    // assert!(board.place_tile(field!(5, 3), tile!(5, 7, 3)).is_ok());

    board
}



#[test]
fn test_eval_empty_board() {
    let empty_board = Board::new();
    assert!(AI::estimated_score(&empty_board, 2).unwrap() > 0.0);
}

#[test]
fn test_eval_full_board() {
    let board = get_full_board();
    let expected_score = board.score() as f64;
    assert_eq!(expected_score, AI::estimated_score(&board, 0).unwrap());
}

#[test]
fn test_eval_almost_full_board() {
    let empty_field = field!(5,3);
    let tolerance_percent = 1.0;
    let board = get_almost_full_board();
    assert_eq!(Board::all_fields().len() - 1, board.tiles.len());

    // calc mean of scores for all possible tiles
    let mut total_score = 0.0;
    for possible_last_tile in board.remaining_tiles() {
        let full_board = board.place_tile_on_new_board(empty_field, possible_last_tile).unwrap();
        assert!(full_board.is_full());
        assert_ne!(0, full_board.score());
        total_score += full_board.score() as f64;
    }
    let expected_score = total_score / board.remaining_tiles().len() as f64;
    let mut diff = expected_score - AI::estimated_score(&board, 1).unwrap();
    if diff < 0.0 {
        diff = -diff;
    }
    let diff_percent = diff / expected_score * 100.0;
    assert!(diff_percent < tolerance_percent);
}

// #[test]
// fn test_ai_full_game() {
//     todo!();
//     // let
//     // assert_eq!(0, Board::new().score());
// }
