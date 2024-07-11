use crate::{Board, Field, Game};
use std::collections::HashMap;

fn best_field(scores: &HashMap<Field, f64>) -> Field {
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

pub struct AI {}

impl AI {
    pub fn new() -> Result<AI, ()> {
        Ok(AI {})
    }

    pub fn play_game(depth: i32, print: bool) -> u32 {
        let mut game = Game::new();
        while !game.finished() {
            let mut scores: HashMap<Field, f64> = HashMap::new();
            for field in game.board.empty_fields() {
                let board_with_new_tile = game
                    .board
                    .place_tile_on_new_board(field, game.current_tile.unwrap())
                    .unwrap();
                scores.insert(
                    field,
                    AI::estimated_score(&board_with_new_tile, depth).unwrap(),
                );
            }
            assert!(game.place_tile(best_field(&scores)).is_ok());
            if print {
                dbg!(scores);
                println!("{}", game.board);
            }
        }
        let score = game.board.score();
        if print {
            dbg!(score);
            println!("{}", game.board);
        }
        score
    }

    fn eval_position(board: &Board) -> Result<f64, ()> {
        if board.is_full() {
            return Ok(board.score() as f64);
        }
        Ok(board.max_score() as f64) // TODO: better estimate
    }

    pub fn estimated_score(board: &Board, iterations: i32) -> Result<f64, ()> {
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
            let mut scores: HashMap<Field, f64> = HashMap::new();
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
