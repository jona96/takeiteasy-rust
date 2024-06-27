use takeiteasy::*;

#[test]
fn test_empty_board() {
    assert_eq!(0, Board::new()
        .score());
}

#[test]
fn test_incomplete_section() {
    let mut board = Board::new();
    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(1,2), tile!(1,2,4)).is_ok());
    assert_eq!(0, board.score());
}

#[test]
fn test_complete_non_matching_section() {
    // left column
    let mut board = Board::new();
    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(1,2), tile!(1,2,4)).is_ok());
    assert!(board.place_tile(field!(1,3), tile!(9,2,8)).is_ok());
    assert_eq!(0, board.score());
}

#[test]
fn test_complete_top_section() {
    // left column
    let mut board = Board::new();
    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(1,2), tile!(1,2,4)).is_ok());
    assert!(board.place_tile(field!(1,3), tile!(1,2,8)).is_ok());
    assert_eq!(3 * 1, board.score());

    // middle column
    let mut board = Board::new();
    assert!(board.place_tile(field!(3,1), tile!(5,2,3)).is_ok());
    assert!(board.place_tile(field!(3,2), tile!(5,2,4)).is_ok());
    assert!(board.place_tile(field!(3,3), tile!(5,2,8)).is_ok());
    assert!(board.place_tile(field!(3,4), tile!(5,6,8)).is_ok());
    assert!(board.place_tile(field!(3,5), tile!(5,6,3)).is_ok());
    assert_eq!(5 * 5, board.score());

    // right column
    let mut board = Board::new();
    assert!(board.place_tile(field!(5,1), tile!(9,2,3)).is_ok());
    assert!(board.place_tile(field!(5,2), tile!(9,2,4)).is_ok());
    assert!(board.place_tile(field!(5,3), tile!(9,2,8)).is_ok());
    assert_eq!(3 * 9, board.score());
}

#[test]
fn test_multiple_complete_top_sections() {
    // left and second to right column
    let mut board = Board::new();
    
    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(1,2), tile!(1,2,4)).is_ok());
    assert!(board.place_tile(field!(1,3), tile!(1,2,8)).is_ok());
    
    assert!(board.place_tile(field!(4,1), tile!(5,2,3)).is_ok());
    assert!(board.place_tile(field!(4,2), tile!(5,2,4)).is_ok());
    assert!(board.place_tile(field!(4,3), tile!(5,2,8)).is_ok());
    assert!(board.place_tile(field!(4,4), tile!(5,6,8)).is_ok());

    assert_eq!(3 * 1 + 4 * 5, board.score());
}

#[test]
fn test_complete_left_section() {
    // row from top left corner to top middle corner
    let mut board = Board::new();
    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(2,1), tile!(5,2,3)).is_ok());
    assert!(board.place_tile(field!(3,1), tile!(9,2,3)).is_ok());
    assert_eq!(3 * 2, board.score());

    // row from bottom left corner to top right corner
    let mut board = Board::new();
    assert!(board.place_tile(field!(1,3), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(2,3), tile!(5,2,3)).is_ok());
    assert!(board.place_tile(field!(3,3), tile!(9,2,3)).is_ok());
    assert!(board.place_tile(field!(4,2), tile!(9,2,4)).is_ok());
    assert!(board.place_tile(field!(5,1), tile!(5,2,4)).is_ok());
    assert_eq!(5 * 2, board.score());

    // row from bottom middle corner to bottom right corner
    let mut board = Board::new();
    assert!(board.place_tile(field!(3,5), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(4,4), tile!(5,2,3)).is_ok());
    assert!(board.place_tile(field!(5,3), tile!(9,2,3)).is_ok());
    assert_eq!(3 * 2, board.score());
}

#[test]
fn test_full_board() {

    // ****************************************************************
    // *                               3                              *
    // *                            _______                           *
    // *                      2    /       \    4                     *
    // *                   _______/    1    \_______                  *
    // *             1    /       \  6   3  /       \    5            *
    // *          _______/    5    \_______/    1    \_______         *
    // *         /       \  6   8  /       \  2   3  /       \        *
    // *        /    9    \_______/    1    \_______/    5    \       *
    // *        \  7   3  /       \  2   8  /       \  2   3  /       *
    // *      1  \_______/    5    \_______/    1    \_______/  1     *
    // *         /       \  2   8  /       \  6   8  /       \        *
    // *        /    9    \_______/    9    \_______/    5    \       *
    // *        \  2   4  /       \  6   4  /       \  7   8  /       *
    // *      2  \_______/    1    \_______/    1    \_______/  2     *
    // *         /       \  2   4  /       \  7   8  /       \        *
    // *        /    9    \_______/    1    \_______/    5    \       *
    // *        \  2   3  /       \  7   4  /       \  7   3  /       *
    // *      3  \_______/    5    \_______/    5    \_______/  3     *
    // *                 \  6   3  /       \  7   4  /                *
    // *               4  \_______/    1    \_______/  4              *
    // *                          \  7   3  /                         *
    // *                        5  \_______/  5                       *
    // *                                                              *
    // ****************************************************************


    let mut board = Board::new();
    assert!(board.place_tile(field!(1,1), tile!(9,7,3)).is_ok());
    assert!(board.place_tile(field!(1,2), tile!(9,2,4)).is_ok());
    assert!(board.place_tile(field!(1,3), tile!(9,2,3)).is_ok());

    assert!(board.place_tile(field!(2,1), tile!(5,6,8)).is_ok());
    assert!(board.place_tile(field!(2,2), tile!(5,2,8)).is_ok());
    assert!(board.place_tile(field!(2,3), tile!(1,2,4)).is_ok());
    assert!(board.place_tile(field!(2,4), tile!(5,6,3)).is_ok());

    assert!(board.place_tile(field!(3,1), tile!(1,6,3)).is_ok());
    assert!(board.place_tile(field!(3,2), tile!(1,2,8)).is_ok());
    assert!(board.place_tile(field!(3,3), tile!(9,6,4)).is_ok());
    assert!(board.place_tile(field!(3,4), tile!(1,7,4)).is_ok());
    assert!(board.place_tile(field!(3,5), tile!(1,7,3)).is_ok());

    assert!(board.place_tile(field!(4,1), tile!(1,2,3)).is_ok());
    assert!(board.place_tile(field!(4,2), tile!(1,6,8)).is_ok());
    assert!(board.place_tile(field!(4,3), tile!(1,7,8)).is_ok());
    assert!(board.place_tile(field!(4,4), tile!(5,7,4)).is_ok());

    assert!(board.place_tile(field!(5,1), tile!(5,2,3)).is_ok());
    assert!(board.place_tile(field!(5,2), tile!(5,7,8)).is_ok());
    assert!(board.place_tile(field!(5,3), tile!(5,7,3)).is_ok());

    assert_eq!(board.score(), 0
        + (3 * 9) // column 1
        + (3 * 5) // column 5
        + (4 * 2) // left 2
        + (3 * 7) // left 5
        + (3 * 3) // right 1
        + (4 * 8) // right 2
        + (4 * 4) // right 4
        + (3 * 3) // right 5
    );
    
}
