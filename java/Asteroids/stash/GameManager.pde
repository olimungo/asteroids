public class GameManager {
    private final static int LIFES_WHEN_STARTING = 3;
    private final static int ASTEROIDS_START_MAX = 12;
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
    private int gameOverTimeout;
    private int lifeAddedSoFar;
    private int ufoFrequency;
    private int ufoShootFrequency;
    private Boolean gamePaused = false;

    GameManager() {
        // this.spritesManager = new SpritesManager();
        // this.overlaysManager = new OverlaysManager();

        // this.spritesManager.createAsteroids(ASTEROIDS_START_MAX);
        // this.spritesManager.createShip();
        // this.spritesManager.createUfo(0);

        // for (const cookie of document.cookie.replace(/ /g, '').split(';')) {
        //     const cookieSplit = cookie.split('=');

        //     if (cookieSplit[0] === 'top-score') {
        //         this.topScore = parseInt(cookieSplit[1]);
        //     }
        // }
    }

    // void update() {
        // this.overlaysManager.update();

        // if (!this.gamePaused) {
        //     this.spritesManager.update();

        //     // if (this.gameState === GameState.PLAYING) {
        //     //     this.checkLevel();
        //     //     this.checkNewLife();
        //     // }
        // }
    }

    // void draw() {
        // this.overlaysManager.drawBackground();

        // this.spritesManager.draw();

        // this.overlaysManager.drawForeground(
        //     this.gameState,
        //     this.gamePaused,
        //     this.topScore,
        //     this.getScore(),
        //     this.level,
        //     this.lifes
        // );
    // }

    // void keyPressed(int keyCode) {
        // this.overlaysManager.keyPressed(keyCode);

        // switch (this.gameState) {
        //     case GameState.HOMESCREEN:
        //     case GameState.GAME_OVER:
        //         if (keyCode === 83) {
        //             // s
        //             // this.startGame();
        //         }

        //         break;
        //     case GameState.PLAYING:
        //         this.spritesManager.keyPressed(keyCode);

        //         if (keyCode === 80) {
        //             // p
        //             this.gamePaused = !this.gamePaused;
        //         }

        //         break;
        //     case GameState.NEXT_LEVEL:
        //         if (keyCode == 83) {
        //             // s
        //             // this.nextLevel();
        //         }

        //         break;
        //     case GameState.NEXT_LIFE:
        //         if (keyCode == 83) {
        //             // s
        //             // this.startLevel();
        //         }

        //         break;
        // }
    // }

    // void keyReleased(keyCode) {
    //     // this.spritesManager.keyReleased(keyCode);
    // }
}