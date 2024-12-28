pub const PLAYER_MAX_LEVEL: u8 = 100;

#[derive(Debug, Default)]
pub struct GameState {
    player_level: u8,
}

impl GameState {
    pub fn update(&mut self) {
        if self.player_level < PLAYER_MAX_LEVEL {
            self.player_level += 1;
        }
    }
    
    pub fn player_level(&self) -> u8 {
        self.player_level
    }
}