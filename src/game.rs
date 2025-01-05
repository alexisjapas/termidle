use std::cmp;

pub const PLAYER_MAX_LEVEL: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weapon {
    Hands,
    Axe,
    Bow,
    Sword,
}

impl Weapon {
    pub fn name(&self) -> &str {
        match self {
            Weapon::Hands => "Hands",
            Weapon::Axe => "Axe",
            Weapon::Bow => "Bow",
            Weapon::Sword => "Sword",
        }
    }

    pub fn damages(&self) -> u8 {
        match self {
            Weapon::Hands => 0,
            Weapon::Axe => 20,
            Weapon::Bow => 13,
            Weapon::Sword => 16,
        }
    }
}

#[derive(Debug)]
pub enum LogType {
    System,
    Status,
    Fight,
}

#[derive(Debug)]
pub struct LogEntry {
    pub log_type: LogType,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    PlayerSelection,
    Playing,
    Victory,
    GameOver,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    health: u8,
    attack: u8,
}

pub struct GameState {
    player_level: u8,
    player_health: u8,
    player_attack: u8,
    player_weapon: Weapon,
    status: GameStatus,
    logs: Vec<LogEntry>,
    weapons: Vec<Weapon>,
    current_selection: usize,
    enemy: Option<Enemy>,
}

impl Enemy {
    fn new(initial_health: u8, initial_attack: u8) -> Self {
        // Enemy scales with player level
        Self {
            health: initial_health,
            attack: initial_attack,
        }
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn attack(&self) -> u8 {
        self.attack
    }
}

impl GameState {
    pub fn new(initial_level: u8, initial_health: u8, initial_attack: u8) -> Self {
        let mut state = Self {
            player_level: initial_level,
            player_health: initial_health,
            player_attack: initial_attack,
            player_weapon: Weapon::Hands,
            status: GameStatus::PlayerSelection,
            logs: Vec::new(),
            weapons: vec![Weapon::Axe, Weapon::Bow, Weapon::Sword],
            current_selection: 0,
            enemy: None,
        };
        state.add_log(LogType::System, "Game started");
        state
    }

    fn add_log(&mut self, log_type: LogType, message: &str) {
        self.logs.insert(
            0,
            LogEntry {
                log_type,
                message: message.to_string(),
            },
        );
    }

    pub fn increase_selection(&mut self) {
        if self.status == GameStatus::PlayerSelection {
            self.current_selection =
                (self.current_selection + self.weapons.len() - 1) % self.weapons.len();
        }
    }

    pub fn decrease_selection(&mut self) {
        if self.status == GameStatus::PlayerSelection {
            self.current_selection = (self.current_selection + 1) % self.weapons.len();
        }
    }

    pub fn validate_selection(&mut self) {
        if self.status == GameStatus::PlayerSelection {
            self.player_weapon = self.weapons[self.current_selection];
            self.status = GameStatus::Playing;
        }
    }

    pub fn update(&mut self) {
        if self.status() == &GameStatus::Playing {
            self.enemy = Some(Enemy::new(28, 10));

            if self.fight() {
                self.add_log(LogType::Status, "Won the fight!");
                self.level_up(1);
                if self.player_level >= PLAYER_MAX_LEVEL {
                    self.status = GameStatus::Victory;
                    self.add_log(LogType::System, "Victory!");
                }
            } else {
                self.add_log(LogType::Status, "Lost the fight!");
                self.status = GameStatus::GameOver;
                self.add_log(LogType::System, "Game Over!");
            }
        }
    }

    pub fn level_up(&mut self, amount: u8) {
        self.player_level = cmp::min(self.player_level + amount, PLAYER_MAX_LEVEL);
        self.add_log(
            LogType::Status,
            &format!("Level up to {}", self.player_level()),
        );
    }

    pub fn fight(&mut self) -> bool {
        if self.enemy.is_none() {
            return true;
        }
        loop {
            let (enemy_health, enemy_attack) = if let Some(enemy) = &self.enemy {
                (enemy.health, enemy.attack)
            } else {
                return true;
            };

            // Player attacks
            let player_damage = self.player_attack + self.player_weapon.damages();
            let new_enemy_health = enemy_health.saturating_sub(player_damage);

            if let Some(enemy) = &mut self.enemy {
                enemy.health = new_enemy_health;
            }
            self.add_log(
                LogType::Fight,
                &format!(
                    "You attack the enemy by {} => Enemy remaining hp is {}",
                    player_damage, new_enemy_health
                ),
            );
            if new_enemy_health == 0 {
                return true;
            }

            // Enemy attacks
            self.player_health = self.player_health.saturating_sub(enemy_attack);
            self.add_log(
                LogType::Fight,
                &format!(
                    "The enemy attacks you by {} => Your remaining hp is {}",
                    enemy_attack, self.player_health
                ),
            );
            if self.player_health == 0 {
                return false;
            }
        }
    }

    // Getters

    pub fn logs(&self) -> &Vec<LogEntry> {
        &self.logs
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

    pub fn weapons(&self) -> &[Weapon] {
        &self.weapons
    }

    pub fn current_selection(&self) -> usize {
        self.current_selection
    }

    pub fn player_weapon(&self) -> Weapon {
        self.player_weapon
    }

    pub fn enemy(&self) -> Option<&Enemy> {
        self.enemy.as_ref()
    }
}
