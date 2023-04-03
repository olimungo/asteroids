impl ConfigGame {
    pub fn new() -> ConfigGame {
        ConfigGame {
            lifes_when_starting: 3,
            asteroids_start_count: 12,
            asteroids_level_increment: 3,
            game_over_state_timeout: 8000,
            add_life_when_scored: 3000,
            asteroid_hit_score: 10,
            ufo_hit_score: 50,
            ufo_create_init_frequency: 15000,
            ufo_create_decrement_frequency: 1000,
            ufo_create_minimal_frequency: 10000,
            ufo_shoot_init_frequency: 15000,
            ufo_shoot_decrement_frequency: 500,
            ufo_shoot_minimal_frequency: 5000,
        }
    }
}

impl SpritesManager {
    pub fn new() -> SpritesManager {
        SpritesManager {
            diameter_min: 40.0,
            diameter_max: 120.0,
            sides_min: 8,
            sides_max: 20,
            asteroid_velocity_min_value: 0.2,
            asteroid_velocity_limit: 1.0,
            ufo_velocity_min_value: 0.2,
            ufo_velocity_limit: 1.0,
        }
    }
}

pub struct Config {
    pub sprites: ConfigSprites,
    pub overlays: ConfigOverlays,
    pub game: ConfigGame,
}

impl Config {
    pub fn new() -> Config {
        Config {
            sprites: ConfigSprites::new(),
            overlays: ConfigOverlays::new(),
            game: ConfigGame::new(),
        }
    }
}

// Sprites
pub struct SpritesManager {
    pub diameter_min: f64,
    pub diameter_max: f64,
    pub sides_min: u32,
    pub sides_max: u32,
    pub asteroid_velocity_min_value: f64,
    pub asteroid_velocity_limit: f64,
    pub ufo_velocity_min_value: f64,
    pub ufo_velocity_limit: f64,
}

pub struct Ship {}

pub struct Ufo {}

pub struct ConfigSprites {
    pub manager: SpritesManager,
    pub ship: Ship,
    pub ufo: Ufo,
}

impl ConfigSprites {
    pub fn new() -> ConfigSprites {
        ConfigSprites {
            manager: SpritesManager::new(),
            ship: Ship {},
            ufo: Ufo {},
        }
    }
}

// Overlays
pub struct ConfigOverlays {}

// Game
pub struct ConfigGame {
    pub lifes_when_starting: u32,
    pub asteroids_start_count: u32,
    pub asteroids_level_increment: u32,
    pub game_over_state_timeout: u32, // ms
    pub add_life_when_scored: u32,
    pub asteroid_hit_score: u32,
    pub ufo_hit_score: u32,
    pub ufo_create_init_frequency: u32,      // ms
    pub ufo_create_decrement_frequency: u32, // ms
    pub ufo_create_minimal_frequency: u32,   // ms
    pub ufo_shoot_init_frequency: u32,       // ms
    pub ufo_shoot_decrement_frequency: u32,  // ms
    pub ufo_shoot_minimal_frequency: u32,    // ms
}

impl ConfigOverlays {
    pub fn new() -> ConfigOverlays {
        ConfigOverlays {}
    }
}
