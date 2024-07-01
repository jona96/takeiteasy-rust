use takeiteasy::*;

#[test]
fn test_start_to_finish() {
    let mut total_score = 0;
    for _ in 0..10 {
        let mut game = Game::new();

        for field in Board::all_fields() {
            assert!(!game.finished());
            assert!(game.current_tile.is_some());
            assert!(game.place_tile(field).is_ok());
        }
        assert!(game.finished());
        total_score += game.board.score();
    }
    assert!(total_score > 0);
}
