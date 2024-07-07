use crate::{Board, Game, TileReservoir};

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

    pub fn estimated_score(board: &Board) -> Result<f32, ()> {
        Ok(0.0)
    }
}
