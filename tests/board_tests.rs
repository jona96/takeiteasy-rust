use takeiteasy::*;

#[test]
fn test_new_board_has_no_tiles() {
    let board = Board::new();
    assert!(&board.tiles.is_empty());
}

#[test]
fn test_new_board_fields_empty() {
    let board = Board::new();

    assert_eq!(board.tiles.get(&field!(1, 1)), None);
    assert_eq!(board.tiles.get(&field!(1, 2)), None);
    assert_eq!(board.tiles.get(&field!(1, 3)), None);

    assert_eq!(board.tiles.get(&field!(2, 1)), None);
    assert_eq!(board.tiles.get(&field!(2, 2)), None);
    assert_eq!(board.tiles.get(&field!(2, 3)), None);
    assert_eq!(board.tiles.get(&field!(2, 4)), None);

    assert_eq!(board.tiles.get(&field!(3, 1)), None);
    assert_eq!(board.tiles.get(&field!(3, 2)), None);
    assert_eq!(board.tiles.get(&field!(3, 3)), None);
    assert_eq!(board.tiles.get(&field!(3, 4)), None);
    assert_eq!(board.tiles.get(&field!(3, 5)), None);

    assert_eq!(board.tiles.get(&field!(4, 1)), None);
    assert_eq!(board.tiles.get(&field!(4, 2)), None);
    assert_eq!(board.tiles.get(&field!(4, 3)), None);
    assert_eq!(board.tiles.get(&field!(4, 4)), None);

    assert_eq!(board.tiles.get(&field!(5, 1)), None);
    assert_eq!(board.tiles.get(&field!(5, 2)), None);
    assert_eq!(board.tiles.get(&field!(5, 3)), None);
}

#[test]
fn test_place_valid_tile() {
    let mut board = Board::new();
    assert!(board.place_tile(field!(1, 3), tile!(1, 2, 3)).is_ok());
    assert_eq!(
        board.tiles.get(&field!(1, 3)),
        Some(&Some(tile!(1, 2, 3)))
    );
}

#[test]
fn test_place_tile_on_invalid_field() {
    let mut board = Board::new();
    assert!(board.place_tile(field!(1,5), tile!(1,2,3)).is_err());
}

#[test]
fn test_place_multiple_tiles() {
    let mut board = Board::new();
    assert!(board.place_tile(field!(1, 3), tile!(1, 2, 3)).is_ok());
    assert_eq!(board.tiles.get(&field!(1, 3)), Some(&Some(tile!(1, 2, 3))));
    assert!(board.place_tile(field!(5, 2), tile!(9, 7, 8)).is_ok());
    assert_eq!(board.tiles.get(&field!(1, 3)), Some(&Some(tile!(1, 2, 3)))); // old tile shouldn't change
    assert_eq!(board.tiles.get(&field!(5, 2)), Some(&Some(tile!(9, 7, 8))));
}

#[test]
fn test_place_used_tile() {
    let mut board = Board::new();
    assert!(board.place_tile(field!(1, 3), tile!(1, 2, 3)).is_ok());
    assert!(board.place_tile(field!(5, 2), tile!(1, 2, 3)).is_err());
}

#[test]
fn test_place_on_used_field() {
    let mut board = Board::new();
    assert!(board.place_tile(field!(1, 3), tile!(1, 2, 3)).is_ok());
    assert!(board.place_tile(field!(1, 3), tile!(9, 7, 8)).is_err());
}

#[test]
fn test_print_empty_board() {
    
    let expected = r"
                                _______
                               /       \     
                       _______/         \_______
                      /       \         /       \     
              _______/         \_______/         \_______
             /       \         /       \         /       \
            /         \_______/         \_______/         \
            \         /       \         /       \         /
             \_______/         \_______/         \_______/   
             /       \         /       \         /       \
            /         \_______/         \_______/         \
            \         /       \         /       \         /
             \_______/         \_______/         \_______/   
             /       \         /       \         /       \
            /         \_______/         \_______/         \
            \         /       \         /       \         /
             \_______/         \_______/         \_______/   
                     \         /       \         /
                      \_______/         \_______/   
                              \         /
                               \_______/";

    let board = Board::new();
    assert_eq!(expected, format!("{}", board));
}

#[test]
fn test_print_single_tile() {
    
    let expected = r"
                                _______
                               /       \     
                       _______/         \_______
                      /       \         /       \     
              _______/         \_______/         \_______
             /       \         /       \         /       \
            /    1    \_______/         \_______/         \
            \  2   3  /       \         /       \         /
             \_______/         \_______/         \_______/   
             /       \         /       \         /       \
            /         \_______/         \_______/         \
            \         /       \         /       \         /
             \_______/         \_______/         \_______/   
             /       \         /       \         /       \
            /         \_______/         \_______/         \
            \         /       \         /       \         /
             \_______/         \_______/         \_______/   
                     \         /       \         /
                      \_______/         \_______/   
                              \         /
                               \_______/";

    let mut board = Board::new();
    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert_eq!(expected, format!("{}", board));
}

#[test]
fn test_remaining_tiles_len() {
    let mut tiles = TileReservoir::new();
    let mut num_tiles = tiles.remaining_tiles.len();

    let mut board = Board::new();
    assert_eq!(num_tiles, board.remaining_tiles().len());

    for field in Field::all_fields() {
        let tile = tiles.pick_random_tile().unwrap();
        board.place_tile(field, tile).unwrap();
        num_tiles -= 1;
        assert_eq!(num_tiles, board.remaining_tiles().len());
    }
    assert_ne!(0, board.remaining_tiles().len());
}

#[test]
fn test_remaining_tiles_value() {
    let mut board = Board::new();

    assert!(board.remaining_tiles().contains(&tile!(1,2,3)));
    assert!(board.remaining_tiles().contains(&tile!(9,7,8)));

    assert!(board.place_tile(field!(1,1), tile!(1,2,3)).is_ok());
    assert!(!board.remaining_tiles().contains(&tile!(1,2,3)));
    assert!(board.remaining_tiles().contains(&tile!(9,7,8)));

    assert!(board.place_tile(field!(1,2), tile!(9,7,8)).is_ok());
    assert!(!board.remaining_tiles().contains(&tile!(1,2,3)));
    assert!(!board.remaining_tiles().contains(&tile!(9,7,8)));
}

#[test]
fn test_print_full_board() {

    let expected = r"
                                _______
                               /       \     
                       _______/    1    \_______
                      /       \  6   3  /       \     
              _______/    5    \_______/    1    \_______
             /       \  6   8  /       \  2   3  /       \
            /    9    \_______/    1    \_______/    5    \
            \  7   3  /       \  2   8  /       \  2   3  /
             \_______/    5    \_______/    1    \_______/   
             /       \  2   8  /       \  6   8  /       \
            /    9    \_______/    9    \_______/    5    \
            \  2   4  /       \  6   4  /       \  7   8  /
             \_______/    1    \_______/    1    \_______/   
             /       \  2   4  /       \  7   8  /       \
            /    9    \_______/    1    \_______/    5    \
            \  2   3  /       \  7   4  /       \  7   3  /
             \_______/    5    \_______/    5    \_______/   
                     \  6   3  /       \  7   4  /
                      \_______/    1    \_______/   
                              \  7   3  /
                               \_______/";

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

    assert_eq!(expected, format!("{}", board));
}