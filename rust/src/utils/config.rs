pub struct Config {
    pub sprites: ConfigSprites,
    pub overlays: ConfigOverlays,
    pub game: ConfigGame,
}

pub struct ConfigSprites {}
pub struct ConfigOverlays {}
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
impl ConfigOverlays {
    pub fn new() -> ConfigOverlays {
        ConfigOverlays {}
    }
}
impl ConfigSprites {
    pub fn new() -> ConfigSprites {
        ConfigSprites {}
    }
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
