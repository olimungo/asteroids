// Config
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

// Sprites
pub struct SpritesManager {
    pub asteroid_diameter_min: f64,
    pub asteroid_diameter_max: f64,
    pub asteroid_sides_min: u32,
    pub asteroid_sides_max: u32,
    pub asteroid_velocity_min_value: f64,
    pub asteroid_velocity_limit: f64,
    pub ufo_velocity_min_value: f64,
    pub ufo_velocity_limit: f64,
}

impl SpritesManager {
    pub fn new() -> SpritesManager {
        SpritesManager {
            asteroid_diameter_min: 40.0,
            asteroid_diameter_max: 120.0,
            asteroid_sides_min: 8,
            asteroid_sides_max: 20,
            asteroid_velocity_min_value: 0.2,
            asteroid_velocity_limit: 1.0,
            ufo_velocity_min_value: 0.2,
            ufo_velocity_limit: 1.0,
        }
    }
}

pub struct Ship {
    pub ship_shell_size: f64,
    pub booster_interval: u32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            ship_shell_size: 36.0,
            booster_interval: 150,
        }
    }
}

pub struct Ufo {
    pub ufo_width: f64,
    pub ufo_height: f64,
    pub ufo_velocity: f64,
    pub change_heading_frequency: u32,
    pub variability_in_heading: u32,
    pub variability_in_shooting: u32,
}

impl Ufo {
    pub fn new() -> Ufo {
        Ufo {
            ufo_width: 60.0,
            ufo_height: 25.0,
            ufo_velocity: 2.0,
            change_heading_frequency: 6000,
            variability_in_heading: 3000,
            variability_in_shooting: 1000,
        }
    }
}

pub struct Potatoid {
    pub diameter_max: f64,
    pub potatoid_minimal_diameter_breakup: f64,
    pub vertex_radius_min: f64,
    pub vertex_radius_max: f64,
}

impl Potatoid {
    pub fn new() -> Potatoid {
        Potatoid {
            diameter_max: 100.0,
            potatoid_minimal_diameter_breakup: 60.0,
            vertex_radius_min: 0.35,
            vertex_radius_max: 0.5,
        }
    }
}

pub struct Particle {
    pub polygon_sides: u8,
    pub health_decrement: u32,
    pub minimal_health: u32,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            polygon_sides: 5,
            health_decrement: 5,
            minimal_health: 50,
        }
    }
}

pub struct Explosion {
    pub count_particules: u8,
}

impl Explosion {
    pub fn new() -> Explosion {
        Explosion {
            count_particules: 8,
        }
    }
}

pub struct ConfigSprites {
    pub manager: SpritesManager,
    pub ship: Ship,
    pub ufo: Ufo,
    pub potatoid: Potatoid,
    pub particle: Particle,
    pub explosion: Explosion,
}

impl ConfigSprites {
    pub fn new() -> ConfigSprites {
        ConfigSprites {
            manager: SpritesManager::new(),
            ship: Ship::new(),
            ufo: Ufo::new(),
            potatoid: Potatoid::new(),
            particle: Particle::new(),
            explosion: Explosion::new(),
        }
    }
}

// Overlays

pub struct OverlayManager {
    pub display_new_life_timeout: u32,
}

impl OverlayManager {
    pub fn new() -> OverlayManager {
        OverlayManager {
            display_new_life_timeout: 7000,
        }
    }
}

pub struct Keycap {
    pub size: f64,
}

impl Keycap {
    pub fn new() -> Keycap {
        Keycap { size: 30.0 }
    }
}

pub struct Spacebar {
    pub width: f64,
    pub height: f64,
}

impl Spacebar {
    pub fn new() -> Spacebar {
        Spacebar {
            width: 200.0,
            height: 30.0,
        }
    }
}

pub struct Starfield {
    pub stars_count: u32,
}

impl Starfield {
    pub fn new() -> Starfield {
        Starfield { stars_count: 400 }
    }
}

pub struct ConfigOverlays {
    pub manager: OverlayManager,
    pub keycap: Keycap,
    pub spacebar: Spacebar,
    pub starfield: Starfield,
}

impl ConfigOverlays {
    pub fn new() -> ConfigOverlays {
        ConfigOverlays {
            manager: OverlayManager::new(),
            keycap: Keycap::new(),
            spacebar: Spacebar::new(),
            starfield: Starfield::new(),
        }
    }
}
