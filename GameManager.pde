import java.util.Timer;
import java.util.TimerTask;

public class GameManager {
    int asteroidsHit;
    int ufosHit;
    int lifes;
    int level;
    GameState state = GameState.HOMESCREEN;

    private Starfield starfield;
    private int maxAsteroids = 15;
    private Helpers helpers;
    private SpritesManager spritesManager;
    private Boolean showStarfield = false;
    private int addLifeWhenScored = 2000;
    private int lifeAddedSoFar = 0;
    private Boolean showNewLife = false;
    private int newLifeTimer = 0;
    private int topScore = 0;
    private int homescreenTimer = 0;
    private Boolean isPaused = false;

    GameManager() {
        this.helpers = new Helpers();
        this.starfield = new Starfield();
        this.spritesManager = new SpritesManager();

        this.spritesManager.createAsteroids(maxAsteroids);
        this.spritesManager.createUfos(1);
    }

    void update() {
        this.spritesManager.update();

        if (!this.isPaused) {
            if (this.state == GameState.PLAYING) {
                // if (this.ship.hitsAsteroid() || ship.hitsUfo()) {
                //     this.lifeLost();
                // }

                // for (Ufo ufo : this.ufos) {
                //     if (ufo.hitsShip()) {
                //         // this.ship.explode();
                //         this.lifeLost();
                //         break;
                //     }
                // }

                this.checkLevelFinished();
                this.checkScore();
            }

            // if (this.state != GameState.HOMESCREEN) {
            //     this.ship.update();
            // }

            if (this.homescreenTimer > 0) {
                this.checkTimerHomescreen();
            }
        }
    }

    void draw() {
        this.spritesManager.draw();

        pushStyle();
            if (this.showStarfield) {
                starfield.draw();
            }

            fill(219, 233, 255);

            // if (this.state != GameState.HOMESCREEN) {
            //     this.ship.draw();
            // }

            this.helpers.showTitles(this.state, this.lifes, this.level, this.getScore(), this.topScore, this.isPaused);

            if (this.state == GameState.PLAYING) {
                if (this.showNewLife) {
                    this.helpers.showNewLife();
                    this.checkTimerNewLife();
                }
            }

            // for (Ufo ufo : this.ufos) {
            //     ufo.draw();
            // }
        popStyle();
    }

    void startGame() {
        this.asteroidsHit = 0;
        this.ufosHit = 0;
        this.lifes = 3;
        this.level = 1;
        this.maxAsteroids = 10;
        this.homescreenTimer = 0;
        this.spritesManager.resetAll();
        this.startNewLife();
    }

    void startNewLife() {
        this.state = GameState.PLAYING;

        this.spritesManager.createShip();
        this.spritesManager.createAsteroids(this.maxAsteroids);

        Timer timer = new Timer();
        timer.schedule(new TimerTask() {
            @Override
            public void run() {
                spritesManager.createUfos(level);
            }
        }, floor(random(1000, 1500)));
    }

    void startNewLevel() {
        this.level++;
        this.maxAsteroids += 5;
        this.startNewLife();
    }

    void lifeLost() {
        this.lifes--;

        if (this.lifes == 0) {
            this.endGame();
        } else {
            this.spritesManager.resetUfos();
            this.state = GameState.NEXT_LIFE;
        }
    }

    void checkScore() {
        if (this.getScore() - this.lifeAddedSoFar * this.addLifeWhenScored >= this.addLifeWhenScored) {
            this.lifes++;
            this.lifeAddedSoFar++;
            this.showNewLife = true;
            this.newLifeTimer = millis() + 2000;
        }
    }

    void endGame() {
        this.state = GameState.GAME_OVER;
        this.spritesManager.resetUfos();
        this.homescreenTimer = millis() + 5000;

        int score = this.getScore();

        if (score > this.topScore) {
            this.topScore = score;
        }
    }

    void asteroidHit() {
        this.asteroidsHit++;
    }

    void ufoHit() {
        this.ufosHit++;

        // if (this.ufos.size() == 0) {
        //     this.timerUfo = millis() + floor(random(10000, 15000));
        // }
    }

    void keyPressed(int keyCode) {
        switch(this.state) {
            case HOMESCREEN:
            case GAME_OVER:
                if (keyCode == 83) { // S
                    this.startGame();
                }
                break;
            case PLAYING:
                this.spritesManager.keyPressed(keyCode);
                break;
            case NEXT_LEVEL:
                if (keyCode == 83) { // S
                    this.startNewLevel();
                    break;
                }
            case NEXT_LIFE:
                if (keyCode == 83) { // S
                    this.startNewLife();
                    break;
                }
        }

        if (keyCode == 88) { // X
            this.showStarfield = !this.showStarfield;
        }
    }

    void keyReleased(int keyCode) {
        if (this.state == GameState.PLAYING) {
            this.spritesManager.keyReleased(keyCode);
        }
    }

    private void checkLevelFinished() {
        if (this.spritesManager.getAsteroidsCount() == 0) {
            this.spritesManager.resetUfos();
            this.state = GameState.NEXT_LEVEL;

            int score = this.getScore();

            if (score > this.topScore) {
                this.topScore = score;
            }
        }
    }

    private int getScore() {
        return this.asteroidsHit * 10 + this.ufosHit * 50;
    }

    private void checkTimerNewLife() {
        if (millis() > this.newLifeTimer) {
            this.newLifeTimer = 0;
            this.showNewLife = false;
        }
    }

    private void checkTimerHomescreen() {
        if (millis() > this.homescreenTimer) {
            this.homescreenTimer = 0;
            this.state = GameState.HOMESCREEN;
        }
    }
}