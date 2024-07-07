use ai::AI;
use takeiteasy::*;

#[test]
fn test_eval_empty_position() {
    let empty_board = Board::new();
    assert_eq!(0.0, AI::estimated_score(&empty_board).unwrap()); // TODO: expected value
}

// #[test]
// fn test_ai_full_game() {
//     todo!();
//     // let
//     // assert_eq!(0, Board::new().score());
// }
