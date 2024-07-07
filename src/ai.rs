use std::collections::{HashMap, HashSet};
use crate::{tile::Tile, Board, Game, TileReservoir};

pub struct AI {
    game: Game,
}

impl AI {
    pub fn new() -> Result<AI, ()> {
        Ok(AI { game: Game::new() })
    }

    pub fn play_game(&mut self) -> Result<i32, ()> {
        todo!();
        // for _ in 0..len(TileReservoir::all_tiles()){
        //     // let position = self.game.board.tiles
        // }
    }

    pub fn estimated_score(board: &Board) -> Result<f64, ()> {
        if board.empty_fields().is_empty() {
            return Ok(board.score() as f64);
        }

        for field in board.empty_fields() {
            let mut total_score = 0.0;
            for tile in board.remaining_tiles() {
                let new_board = board.place_tile_on_new_board(field, tile).unwrap();
                let score = AI::estimated_score(&new_board).unwrap();
                total_score += score;
            }
            let avg_score = total_score / board.remaining_tiles().len() as f64;
            return Ok(avg_score);
        }

        panic!();
    }
}
