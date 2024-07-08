use std::collections::{HashMap, HashSet};
use crate::{tile::Tile, Board, Field, Game, TileReservoir};

pub struct AI {
    game: Game,
}

fn best_field(scores:&HashMap<Field, f64>) -> Field {
    let mut best_field = scores.keys().into_iter().next().unwrap();
    let mut best_score = scores.get(&best_field).unwrap();
    for (field, score) in scores {
        if score > best_score {
            best_score = score;
            best_field = field;
        }
    }
    *best_field
}

impl AI {
    pub fn new() -> Result<AI, ()> {
        Ok(AI { game: Game::new() })
    }

    pub fn play_game(depth:i32) -> u32 {

        let mut game = Game::new();
        while !game.finished() {
            let mut scores:HashMap<Field, f64> = HashMap::new();
            for field in game.board.empty_fields() {
                scores.insert(field, AI::estimated_score(&game.board, depth).unwrap());
            }
            assert!(game.place_tile(best_field(&scores)).is_ok());
            dbg!(scores);
        }
        let score = game.board.score();
        dbg!(score);
        println!("{}", game.board);
        score
    }

    fn eval_position(board: &Board) -> Result<f64, ()> {
        if board.is_full() {
            return Ok(board.score() as f64);
        }
        Ok(board.score() as f64) // TODO: better estimate
    }

    pub fn estimated_score(board: &Board, iterations:i32) -> Result<f64, ()> {
        if iterations == 0 {
            return AI::eval_position(board);
        }

        if board.is_full() {
            return AI::eval_position(board);
        }

        // calc more depth levels
        // one level/iteration means the average of all remaining tiles on all empty fields
        let mut total_score = 0.0;
        for tile in board.remaining_tiles() {
            let mut scores:HashMap<Field, f64> = HashMap::new();
            for field in board.empty_fields() {
                let new_board = board.place_tile_on_new_board(field, tile).unwrap();
                let score = AI::estimated_score(&new_board, iterations - 1).unwrap();
                scores.insert(field, score);
            }
            let best_score = scores.get(&best_field(&scores)).unwrap();
            total_score += best_score;
        }
        let avg_score = total_score / board.remaining_tiles().len() as f64;
        return Ok(avg_score);
    }
}
