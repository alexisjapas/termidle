use std::cmp;

pub const PLAYER_MAX_LEVEL: u8 = 100;

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Victory,
    GameOver,
}

pub struct Enemy {
    health: u8,
    attack: u8,
}

pub struct GameState {
    player_level: u8,
    player_health: u8,
    player_attack: u8,
    status: GameStatus,
    
}

impl Enemy {
    fn new(initial_health: u8, initial_attack: u8) -> Self {
        // Enemy scales with player level
        Self {
            health: initial_health,
            attack: initial_attack,
        }
    }
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
        let mut enemy = Enemy::new(11, 10);
        
        if self.fight(&mut enemy) {
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
    
    pub fn fight(&mut self, enemy: &mut Enemy) -> bool {
        loop {
            enemy.health = enemy.health.saturating_sub(self.player_attack);
            if enemy.health == 0 {
                return true;
            }
            
            self.player_health = self.player_health.saturating_sub(enemy.attack);
            if self.player_health == 0 {
                return false;
            }
        }
    }
    
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    
    pub fn player_level(&self) -> u8 {
        self.player_level
    }
    
    pub fn player_health(&self) -> u8 {
        self.player_health
    }
    
    pub fn player_attack(&self) -> u8 {
        self.player_attack
    }
}

