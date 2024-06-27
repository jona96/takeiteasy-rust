use takeiteasy::*;

#[test]
fn test_all_tiles() {
    let all_tiles = TileReservoir::all_tiles();

    assert!(all_tiles.contains(&tile!(1, 2, 3)));
    assert!(all_tiles.contains(&tile!(5, 2, 3)));
    assert!(all_tiles.contains(&tile!(9, 2, 3)));

    assert!(all_tiles.contains(&tile!(1, 6, 3)));
    assert!(all_tiles.contains(&tile!(5, 6, 3)));
    assert!(all_tiles.contains(&tile!(9, 6, 3)));

    assert!(all_tiles.contains(&tile!(1, 7, 3)));
    assert!(all_tiles.contains(&tile!(5, 7, 3)));
    assert!(all_tiles.contains(&tile!(9, 7, 3)));

    assert!(all_tiles.contains(&tile!(1, 2, 4)));
    assert!(all_tiles.contains(&tile!(5, 2, 4)));
    assert!(all_tiles.contains(&tile!(9, 2, 4)));

    assert!(all_tiles.contains(&tile!(1, 6, 4)));
    assert!(all_tiles.contains(&tile!(5, 6, 4)));
    assert!(all_tiles.contains(&tile!(9, 6, 4)));

    assert!(all_tiles.contains(&tile!(1, 7, 4)));
    assert!(all_tiles.contains(&tile!(5, 7, 4)));
    assert!(all_tiles.contains(&tile!(9, 7, 4)));

    assert!(all_tiles.contains(&tile!(1, 2, 8)));
    assert!(all_tiles.contains(&tile!(5, 2, 8)));
    assert!(all_tiles.contains(&tile!(9, 2, 8)));

    assert!(all_tiles.contains(&tile!(1, 6, 8)));
    assert!(all_tiles.contains(&tile!(5, 6, 8)));
    assert!(all_tiles.contains(&tile!(9, 6, 8)));

    assert!(all_tiles.contains(&tile!(1, 7, 8)));
    assert!(all_tiles.contains(&tile!(5, 7, 8)));
    assert!(all_tiles.contains(&tile!(9, 7, 8)));
}

#[test]
fn test_new_reservoir() {
    let reservoir = TileReservoir::new();

    assert_eq!(reservoir.remaining_tiles, TileReservoir::all_tiles());
}

#[test]
fn test_pick_valid_tile() {
    let mut reservoir = TileReservoir::new();
    assert!(reservoir.pick_tile(&tile!(1, 2, 3)).is_ok());
    assert!(!reservoir.remaining_tiles.contains(&tile!(1, 2, 3)));
}

#[test]
fn test_pick_invalid_tile() {
    let mut reservoir = TileReservoir::new();
    assert!(reservoir.pick_tile(&tile!(1, 2, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 2, 3)).is_err());
}

#[test]
fn test_pick_all_tiles() {
    let mut reservoir = TileReservoir::new();
    assert!(reservoir.pick_tile(&tile!(1, 2, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 2, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 2, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 6, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 6, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 6, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 7, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 7, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 7, 3)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 2, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 2, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 2, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 6, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 6, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 6, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 7, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 7, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 7, 4)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 2, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 2, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 2, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 6, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 6, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 6, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(1, 7, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(5, 7, 8)).is_ok());
    assert!(reservoir.pick_tile(&tile!(9, 7, 8)).is_ok());
    assert!(reservoir.remaining_tiles.is_empty());
}

#[test]
fn test_get_random_tile() {
    assert!(TileReservoir::new().pick_random_tile().is_ok());
}

#[test]
fn test_pick_all_random_tiles() {
    let mut reservoir = TileReservoir::new();
    for _ in 0..TileReservoir::all_tiles().len() {
        assert!(reservoir.pick_random_tile().is_ok());
    }
    assert!(reservoir.pick_random_tile().is_err());
}

#[test]
fn test_pick_random_tile_not_always_the_same() {
    let tries = 1000.0;
    let error_rate = 0.1;
    let mut errors = 0.0;
    for _ in 0..tries as i32 {
        let pick1 = TileReservoir::new().pick_random_tile();
        let pick2 = TileReservoir::new().pick_random_tile();
        if pick1 == pick2 {
            errors += 1.0;
        }
    }
    assert!(
        errors / tries < error_rate,
        "error rate: {}",
        errors / tries
    );
}
