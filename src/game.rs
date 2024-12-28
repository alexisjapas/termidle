use std::cmp;

pub const PLAYER_MAX_LEVEL: u8 = 100;

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Victory,
    GameOver,
}

pub struct GameState {
    player_level: u8,
    player_health: u8,
    player_attack: u8,
    status: GameStatus,
    
}

impl GameState {
    pub fn new(initial_level: u8, initial_health: u8, initial_attack: u8) -> Self {
        Self {
            player_level: initial_level,
            player_health: initial_health,
            player_attack: initial_attack,
            status: GameStatus::Playing,
        }
    }
    
    pub fn update(&mut self) {
        if self.fight() {
            self.level_up(1);
            if self.player_level >= PLAYER_MAX_LEVEL {
                self.status = GameStatus::Victory;
            }
        } else {
            self.status = GameStatus::GameOver;
        }
    }
    
    pub fn level_up(&mut self, amount: u8) {
        self.player_level = cmp::min(self.player_level + amount, PLAYER_MAX_LEVEL);
    }
    
    pub fn fight(&mut self) -> bool {
        true
    }
    
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    
    pub fn player_level(&self) -> u8 {
        self.player_level
    }
}