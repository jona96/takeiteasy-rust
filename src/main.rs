use takeiteasy::*;
use std::io::{self, Write};

fn get_coordinate_from_console_input(msg: &str) -> Result<Coordinate, String> {

    // prompt
    print!{"{}", msg};
    io::stdout().flush().unwrap();
    
    // read
    let mut column = String::new();
    io::stdin().read_line(&mut column).expect("Failed to read line");

    // parse
    let parse_result = column.trim().parse::<i32>();
    if parse_result.is_err(){
        return Err("Please type a number!".to_string());
    }
    let column = parse_result.unwrap();

    // to coordinate
    match Coordinate::from_int(column) {
        Ok(c) => Ok(c),
        Err(()) => Err(format!("invalid coordinate: {}", column)),
    }
}

fn get_field_from_console_input() -> Result<Field, String> {
    
    // column
    let result = get_coordinate_from_console_input("column: ");
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let column = result.unwrap();

    // row
    let result = get_coordinate_from_console_input("row: ");
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let row = result.unwrap();
    
    // create field
    Ok(Field{ column, row })
}

fn place_tile_sequence(game: &mut Game) -> Result<(), String> {

    // board + prompt
    println!("{}\n", game.board);
    println!("where would you place {}?", game.current_tile.unwrap()); 

    // get field
    let field = get_field_from_console_input()
        .map_err(|e| format!("can't read field: {}", e))?;

    // place tile
    game.place_tile(field)
        .map_err(|_| format!("cannot place {} at {}", game.current_tile.unwrap(), &field))
}

fn main() {
    let mut game = Game::new();

    while !game.finished() {
        _ = place_tile_sequence(&mut game).inspect_err(|e| println!("{}", e));
    }
    println!("{}", game.board);
    println!("game finished! Your score: {}", game.board.score());   
}
