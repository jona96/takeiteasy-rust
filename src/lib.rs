use core::fmt;
use rand::Rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[repr(i32)]
#[derive(PartialEq, Debug, Copy, Clone, EnumIter)]
pub enum NumTop {
    N1 = 1,
    N5 = 5,
    N9 = 9,
}

impl NumTop {
    pub fn from_int(num: i32) -> Result<NumTop, ()> {
        match num {
            1 => Ok(NumTop::N1),
            5 => Ok(NumTop::N5),
            9 => Ok(NumTop::N9),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, EnumIter)]
pub enum NumLeft {
    N2 = 2,
    N6 = 6,
    N7 = 7,
}

impl NumLeft {
    pub fn from_int(num: i32) -> Result<NumLeft, ()> {
        match num {
            2 => Ok(NumLeft::N2),
            6 => Ok(NumLeft::N6),
            7 => Ok(NumLeft::N7),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, EnumIter)]
pub enum NumRight {
    N3 = 3,
    N4 = 4,
    N8 = 8,
}

impl NumRight {
    pub fn from_int(num: i32) -> Result<NumRight, ()> {
        match num {
            3 => Ok(NumRight::N3),
            4 => Ok(NumRight::N4),
            8 => Ok(NumRight::N8),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Coordinate {
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
}

impl Coordinate {
    pub fn from_int(num: i32) -> Result<Coordinate, ()> {
        match num {
            1 => Ok(Coordinate::C1),
            2 => Ok(Coordinate::C2),
            3 => Ok(Coordinate::C3),
            4 => Ok(Coordinate::C4),
            5 => Ok(Coordinate::C5),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Field {
    pub column: Coordinate,
    pub row: Coordinate,
}

impl Field {
    pub fn new(column: Coordinate, row: Coordinate) -> Result<Field, String> {
        let field = Field { column, row };
        match field.check() {
            Ok(_) => Ok(field),
            Err(e) => Err(e),
        }
    }

    fn check(&self) -> Result<(), String> {
        if (self.column == Coordinate::C1 || self.column == Coordinate::C5) && self.row as i32 > 3 {
            return Err(format!(
                "invalid coordinates: {}, {}",
                self.column as i32, self.row as i32
            ));
        }
        if (self.column == Coordinate::C2 || self.column == Coordinate::C4) && self.row as i32 > 4 {
            return Err(format!(
                "invalid coordinates: {}, {}",
                self.column as i32, self.row as i32
            ));
        }

        return Ok(());
    }

    pub fn all_fields() -> Vec<Field> {
        vec![
            Field::new(Coordinate::C1, Coordinate::C1).unwrap(),
            Field::new(Coordinate::C1, Coordinate::C2).unwrap(),
            Field::new(Coordinate::C1, Coordinate::C3).unwrap(),
            Field::new(Coordinate::C2, Coordinate::C1).unwrap(),
            Field::new(Coordinate::C2, Coordinate::C2).unwrap(),
            Field::new(Coordinate::C2, Coordinate::C3).unwrap(),
            Field::new(Coordinate::C2, Coordinate::C4).unwrap(),
            Field::new(Coordinate::C3, Coordinate::C1).unwrap(),
            Field::new(Coordinate::C3, Coordinate::C2).unwrap(),
            Field::new(Coordinate::C3, Coordinate::C3).unwrap(),
            Field::new(Coordinate::C3, Coordinate::C4).unwrap(),
            Field::new(Coordinate::C3, Coordinate::C5).unwrap(),
            Field::new(Coordinate::C4, Coordinate::C1).unwrap(),
            Field::new(Coordinate::C4, Coordinate::C2).unwrap(),
            Field::new(Coordinate::C4, Coordinate::C3).unwrap(),
            Field::new(Coordinate::C4, Coordinate::C4).unwrap(),
            Field::new(Coordinate::C5, Coordinate::C1).unwrap(),
            Field::new(Coordinate::C5, Coordinate::C2).unwrap(),
            Field::new(Coordinate::C5, Coordinate::C3).unwrap(),
        ]
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field({} {})", self.column as i32, self.row as i32)
    }
}

#[macro_export]
macro_rules! field {
    ($column:expr, $row:expr) => {
        Field {
            column: Coordinate::from_int($column).unwrap(),
            row: Coordinate::from_int($row).unwrap(),
        }
    };
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Tile {
    pub top: NumTop,
    pub left: NumLeft,
    pub right: NumRight,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tile({} {} {})",
            self.top as i32, self.left as i32, self.right as i32
        )
    }
}

#[macro_export]
macro_rules! tile {
    ($top:expr, $left:expr, $right:expr) => {
        Tile {
            top: NumTop::from_int($top).unwrap(),
            left: NumLeft::from_int($left).unwrap(),
            right: NumRight::from_int($right).unwrap(),
        }
    };
}

#[derive(Debug)]
pub struct Board {
    pub tiles: HashMap<Field, Option<Tile>>,
}

impl Board {
    pub fn new() -> Board {
        let board = Board {
            tiles: HashMap::new(),
        };
        board
    }

    pub fn place_tile(&mut self, field: Field, tile: Tile) -> Result<(), ()> {
        if !field.check().is_ok() {
            return Err(());
        }
        if self.tiles.contains_key(&field) {
            return Err(());
        }
        if self.tiles.values().any(|t| t == &Some(tile)) {
            return Err(());
        }
        self.tiles.insert(field, Some(tile));
        Ok(())
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;

        // sections
        let section_top_1 = vec![field!(1, 1), field!(1, 2), field!(1, 3)];
        let section_top_2 = vec![field!(2, 1), field!(2, 2), field!(2, 3), field!(2, 4)];
        let section_top_3 = vec![
            field!(3, 1),
            field!(3, 2),
            field!(3, 3),
            field!(3, 4),
            field!(3, 5),
        ];
        let section_top_4 = vec![field!(4, 1), field!(4, 2), field!(4, 3), field!(4, 4)];
        let section_top_5 = vec![field!(5, 1), field!(5, 2), field!(5, 3)];

        let section_left_1 = vec![field!(1, 1), field!(2, 1), field!(3, 1)];
        let section_left_2 = vec![field!(1, 2), field!(2, 2), field!(3, 2), field!(4, 1)];
        let section_left_3 = vec![
            field!(1, 3),
            field!(2, 3),
            field!(3, 3),
            field!(4, 2),
            field!(5, 1),
        ];
        let section_left_4 = vec![field!(2, 4), field!(3, 4), field!(4, 3), field!(5, 2)];
        let section_left_5 = vec![field!(3, 5), field!(4, 4), field!(5, 3)];

        let section_right_1 = vec![field!(3, 1), field!(4, 1), field!(5, 1)];
        let section_right_2 = vec![field!(2, 1), field!(3, 2), field!(4, 2), field!(5, 2)];
        let section_right_3 = vec![
            field!(1, 1),
            field!(2, 2),
            field!(3, 3),
            field!(4, 3),
            field!(5, 3),
        ];
        let section_right_4 = vec![field!(1, 2), field!(2, 3), field!(3, 4), field!(4, 4)];
        let section_right_5 = vec![field!(1, 3), field!(2, 4), field!(3, 5)];

        let top_sections = vec![
            section_top_1,
            section_top_2,
            section_top_3,
            section_top_4,
            section_top_5,
        ];
        let left_sections = vec![
            section_left_1,
            section_left_2,
            section_left_3,
            section_left_4,
            section_left_5,
        ];
        let right_sections = vec![
            section_right_1,
            section_right_2,
            section_right_3,
            section_right_4,
            section_right_5,
        ];

        // top sections
        for section in top_sections {
            let numbers: Vec<u32> = section
                .iter()
                .map(|field| match self.tiles.get(field) {
                    Some(tile) => tile.unwrap().top as u32,
                    None => 0,
                })
                .collect();

            if all_elements_equal(&numbers) {
                score += numbers.first().unwrap() * numbers.len() as u32;
            }
        }

        // left sections
        for section in left_sections {
            let numbers: Vec<u32> = section
                .iter()
                .map(|field| match self.tiles.get(field) {
                    Some(tile) => tile.unwrap().left as u32,
                    None => 0,
                })
                .collect();

            if all_elements_equal(&numbers) {
                score += numbers.first().unwrap() * numbers.len() as u32;
            }
        }

        // right sections
        for section in right_sections {
            let numbers: Vec<u32> = section
                .iter()
                .map(|field| match self.tiles.get(field) {
                    Some(tile) => tile.unwrap().right as u32,
                    None => 0,
                })
                .collect();

            if all_elements_equal(&numbers) {
                score += numbers.first().unwrap() * numbers.len() as u32;
            }
        }

        score
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = format!(
            r"
                                _______
                               /       \     
                       _______/    {}    \_______
                      /       \  {}   {}  /       \     
              _______/    {}    \_______/    {}    \_______
             /       \  {}   {}  /       \  {}   {}  /       \
            /    {}    \_______/    {}    \_______/    {}    \
            \  {}   {}  /       \  {}   {}  /       \  {}   {}  /
             \_______/    {}    \_______/    {}    \_______/   
             /       \  {}   {}  /       \  {}   {}  /       \
            /    {}    \_______/    {}    \_______/    {}    \
            \  {}   {}  /       \  {}   {}  /       \  {}   {}  /
             \_______/    {}    \_______/    {}    \_______/   
             /       \  {}   {}  /       \  {}   {}  /       \
            /    {}    \_______/    {}    \_______/    {}    \
            \  {}   {}  /       \  {}   {}  /       \  {}   {}  /
             \_______/    {}    \_______/    {}    \_______/   
                     \  {}   {}  /       \  {}   {}  /
                      \_______/    {}    \_______/   
                              \  {}   {}  /
                               \_______/",
            match self.tiles.get(&field!(3, 1)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 1)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 1)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 1)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 1)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 1)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 1)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 1)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 1)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 1)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 2)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 1)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 1)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 1)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 2)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 2)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 1)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 1)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 2)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 2)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 2)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 2)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 2)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 2)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 2)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 3)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 2)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 2)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 2)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 3)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 3)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 2)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 2)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 3)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 3)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 3)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 3)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 3)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 3)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 3)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 4)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 3)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 3)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(1, 3)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 4)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 4)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 3)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(5, 3)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 4)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 4)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 4)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(2, 4)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 4)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(4, 4)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 5)) {
                Some(&t) => format!("{}", t.unwrap().top as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 5)) {
                Some(&t) => format!("{}", t.unwrap().left as i32),
                None => " ".to_string(),
            },
            match self.tiles.get(&field!(3, 5)) {
                Some(&t) => format!("{}", t.unwrap().right as i32),
                None => " ".to_string(),
            },
        );

        write!(f, "{}", str)
    }
}

fn all_elements_equal<T: PartialEq>(vec: &[T]) -> bool {
    if let Some(first) = vec.first() {
        vec.iter().all(|x| x == first)
    } else {
        true // An empty vector is considered to have all equal elements
    }
}

#[derive(Debug)]
pub struct TileReservoir {
    pub remaining_tiles: Vec<Tile>,
}

impl TileReservoir {
    pub fn new() -> TileReservoir {
        TileReservoir {
            remaining_tiles: TileReservoir::all_tiles(),
        }
    }

    pub fn all_tiles() -> Vec<Tile> {
        let mut tiles: Vec<Tile> = vec![];
        for top in NumTop::iter() {
            for left in NumLeft::iter() {
                for right in NumRight::iter() {
                    tiles.push(Tile { top, left, right });
                }
            }
        }
        tiles
    }

    pub fn pick_tile(&mut self, tile: &Tile) -> Result<(), ()> {
        if !self.remaining_tiles.contains(&tile) {
            return Err(());
        }
        self.remaining_tiles.retain(|&t| &t != tile);
        Ok(())
    }

    pub fn pick_random_tile(&mut self) -> Result<Tile, ()> {
        if self.remaining_tiles.is_empty() {
            return Err(());
        }
        let random_index = rand::thread_rng().gen_range(0..self.remaining_tiles.len());
        let tile = self.remaining_tiles[random_index];
        self.pick_tile(&tile).map(|_| tile)
    }
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    tile_reservoir: TileReservoir,
    pub current_tile: Option<Tile>,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();
        let mut tile_reservoir = TileReservoir::new();
        let current_tile = tile_reservoir.pick_random_tile().unwrap();
        Game {
            board,
            tile_reservoir,
            current_tile: Some(current_tile),
        }
    }

    pub fn place_tile(&mut self, field: Field) -> Result<(), ()> {
        if self
            .board
            .place_tile(field, self.current_tile.unwrap())
            .is_err()
        {
            return Err(());
        }
        self.current_tile = match self.tile_reservoir.pick_random_tile() {
            Ok(tile) => Some(tile),
            Err(_) => None,
        };
        Ok(())
    }

    pub fn finished(&self) -> bool {
        self.board.tiles.len() == 3 + 4 + 5 + 4 + 3
    }
}
