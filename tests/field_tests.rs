use takeiteasy::*;

macro_rules! check_new_valid_field {
    ($col:expr, $row:expr) => {
        assert_eq!(
            Field::new(
                Coordinate::from_int($col).unwrap(),
                Coordinate::from_int($row).unwrap(),
            ),
            Ok(field!($col, $row))
        );
    };
}

macro_rules! check_new_invalid_field {
    ($col:expr, $row:expr, $msg:expr) => {
        assert_eq!(
            Field::new(
                Coordinate::from_int($col).unwrap(),
                Coordinate::from_int($row).unwrap()
            ),
            Err($msg.to_string())
        );
    };
}

#[test]
fn test_field_construction() {
    // first column
    check_new_valid_field!(1, 1);
    check_new_valid_field!(1, 2);
    check_new_valid_field!(1, 3);
    check_new_invalid_field!(1, 4, "invalid coordinates: 1, 4");
    check_new_invalid_field!(1, 5, "invalid coordinates: 1, 5");

    // second column
    check_new_valid_field!(2, 1);
    check_new_valid_field!(2, 2);
    check_new_valid_field!(2, 3);
    check_new_valid_field!(2, 4);
    check_new_invalid_field!(2, 5, "invalid coordinates: 2, 5");

    // third column
    check_new_valid_field!(3, 1);
    check_new_valid_field!(3, 2);
    check_new_valid_field!(3, 3);
    check_new_valid_field!(3, 4);
    check_new_valid_field!(3, 5);

    // fourth column
    check_new_valid_field!(4, 1);
    check_new_valid_field!(4, 2);
    check_new_valid_field!(4, 3);
    check_new_valid_field!(4, 4);
    check_new_invalid_field!(4, 5, "invalid coordinates: 4, 5");

    // fifth column
    check_new_valid_field!(5, 1);
    check_new_valid_field!(5, 2);
    check_new_valid_field!(5, 3);
    check_new_invalid_field!(5, 4, "invalid coordinates: 5, 4");
    check_new_invalid_field!(5, 5, "invalid coordinates: 5, 5");
}
