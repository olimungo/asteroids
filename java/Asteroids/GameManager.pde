public class GameManager {
    private final static int LIFES_WHEN_STARTING = 3;
    private final static int ASTEROIDS_START_MAX = 15;
    private final static int ASTEROIDS_LEVEL_INCREMENT = 3;
    private final static int GAME_OVER_STATE_TIMEOUT = 8000; // ms
    private final static int ADD_LIFE_WHEN_SCORED = 3000;
    private final static int ASTEROID_HIT_SCORE = 10;
    private final static int UFO_HIT_SCORE = 50;
    private final static int UFO_INIT_FREQUENCY = 25000; // ms
    private final static int UFO_DECREMENT_FREQUENCY = 1000; // ms
    private final static int UFO_MINIMAL_FREQUENCY = 10000; // ms
    private final static int UFO_SHOOT_INIT_FREQUENCY = 15000; // ms
    private final static int UFO_SHOOT_DECREMENT_FREQUENCY = 500; // ms
    private final static int UFO_SHOOT_MINIMAL_FREQUENCY = 5000; // ms

    private SpritesManager spritesManager;
    private OverlaysManager overlaysManager;

    private GameState gameState = GameState.HOMESCREEN;

    private int level;
    private int lifes;
    private int score;
    private int topScore = 0;
    private int maxAsteroids;
    private Interval gameOverInterval;
    private int lifeAddedSoFar;
    private int ufoFrequency;
    private int ufoShootFrequency;
    private Boolean gamePaused = false;

    GameManager() {
        this.spritesManager = new SpritesManager();
        this.overlaysManager = new OverlaysManager();

        this.spritesManager.createAsteroids(ASTEROIDS_START_MAX);
        this.spritesManager.createUfo(0);

        // Get the top score from a file
        String[] lines = loadStrings("data/top-score.txt");

        if (lines != null) {
            this.topScore = Integer.parseInt(lines[0]);
        }
    }

    void update() {
        this.overlaysManager.update();

        if (!this.gamePaused) {
            this.spritesManager.update();

            if (this.gameState == GameState.PLAYING) {
                this.checkLevel();
                this.checkNewLife();
            }
        }

        if (this.gameOverInterval != null &&  this.gameOverInterval.isElapsed()) {
            this.gameState = GameState.HOMESCREEN;
            this.gameOverInterval = null;

            // If there is no UFO on the screen, create one
            if (this.spritesManager.getUfosCount() == 0) {
                this.spritesManager.createUfo(0);
            }
        }
    }

    void draw() {
        this.overlaysManager.drawBackground();

        this.spritesManager.draw();

        this.overlaysManager.drawForeground(
            this.gameState,
            this.gamePaused,
            this.topScore,
            this.getScore(),
            this.level,
            this.lifes
        );
    }

    void keyPressed(int keyCode) {
        this.overlaysManager.keyPressed(keyCode);

        switch (this.gameState) {
            case HOMESCREEN:
            case GAME_OVER:
                if (keyCode == 83) {
                    // s
                    this.startGame();
                }

                break;
            case PLAYING:
                this.spritesManager.keyPressed(keyCode);

                if (keyCode == 80) {
                    // p
                    this.gamePaused = !this.gamePaused;
                    
                    if (this.gamePaused) {
                        this.spritesManager.pause();
                        this.overlaysManager.pause();
                    } else {
                        this.spritesManager.unpause();
                        this.overlaysManager.unpause();
                    } 
                }

                break;
            case NEXT_LEVEL:
                if (keyCode == 83) {
                    // s
                    this.nextLevel();
                }

                break;
            case NEXT_LIFE:
                if (keyCode == 83) {
                    // s
                    this.startLevel();
                }

                break;
        }
    }

    void keyReleased(int keyCode) {
        this.spritesManager.keyReleased(keyCode);
    }

    private int getScore() {
        return (
            this.score +
            this.spritesManager.countAsteroidsHit * ASTEROID_HIT_SCORE +
            this.spritesManager.countUfosHit * UFO_HIT_SCORE
        );
    }

    private void startGame() {
        this.gameOverInterval = null;

        this.score = 0;
        this.level = 1;
        this.maxAsteroids = ASTEROIDS_START_MAX;
        this.lifes = LIFES_WHEN_STARTING;
        this.overlaysManager.setLifeCount(this.lifes - 1);
        this.lifeAddedSoFar = 0;
        this.ufoFrequency = UFO_INIT_FREQUENCY;
        this.ufoShootFrequency = UFO_SHOOT_INIT_FREQUENCY;

        this.spritesManager.reset();

        this.startLevel();
    }

    private void startLevel() {
        this.gameState = GameState.PLAYING;

        this.spritesManager.startLevel(
            this.maxAsteroids,
            this.ufoFrequency,
            this.ufoShootFrequency
        );
    }

    private void nextLevel() {
        this.level++;
        this.maxAsteroids += ASTEROIDS_LEVEL_INCREMENT;
        this.ufoFrequency -= UFO_DECREMENT_FREQUENCY;
        this.ufoShootFrequency -= UFO_SHOOT_DECREMENT_FREQUENCY;

        if (this.ufoFrequency < UFO_MINIMAL_FREQUENCY) {
            this.ufoFrequency = UFO_MINIMAL_FREQUENCY;
        }

        if (this.ufoShootFrequency < UFO_SHOOT_MINIMAL_FREQUENCY) {
            this.ufoShootFrequency = UFO_SHOOT_MINIMAL_FREQUENCY;
        }

        this.startLevel();
    }    

    private void checkLevel() {
        if (!this.spritesManager.shipHit()) {
            if (this.spritesManager.getAsteroidsCount() == 0) {
                this.gameState = GameState.NEXT_LEVEL;
                this.spritesManager.stopLevel();
                this.score = this.getScore();
            }
        } else {
            this.lifes--;
            this.overlaysManager.setLifeCount(this.lifes - 1);
            this.spritesManager.stopLevel();

            if (this.lifes == 0) {
                this.gameState = GameState.GAME_OVER;

                int score = this.getScore();
                this.topScore = score > this.topScore ? score : this.topScore;

                // Save the top score to a file
                String[] topScore = split(String.valueOf(this.topScore), ' ');
                saveStrings("data/top-score.txt", topScore);

                // Return to homescreen after some time...
                this.gameOverInterval = new Interval(GAME_OVER_STATE_TIMEOUT);
            } else {
                this.gameState = GameState.NEXT_LIFE;
                this.score = this.getScore();
            }
        }
    }

    private void checkNewLife() {
        if (
            this.getScore() - this.lifeAddedSoFar * ADD_LIFE_WHEN_SCORED >=
            ADD_LIFE_WHEN_SCORED
        ) {
            this.lifeAddedSoFar++;
            this.lifes++;
            this.overlaysManager.setLifeCount(this.lifes);
            this.overlaysManager.displayNewLife();
        }
    }
}