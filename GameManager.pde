
public static enum State {
    HOMESCREEN, PLAYING, NEXT_LIFE, NEXT_LEVEL, GAME_OVER 
}

public class GameManager {
    int asteroidsHit;
    int ufosHit;
    int lifes;
    int level;
    State state = State.HOMESCREEN;

    private Starfield starfield;
    private int maxAsteroids = 15;
    private ArrayList<Asteroid> asteroids;
    private ArrayList<Ufo> ufos;
    private Helpers helpers;
    private Ship ship;
    private int timerUfo = 0;
    private Boolean showStarfield = false;

    GameManager() {
        this.helpers = new Helpers();
        this.starfield = new Starfield();
        this.asteroids = new ArrayList<Asteroid>();
        this.ufos = new ArrayList<Ufo>();
        this.ship = new Ship(this, this.asteroids, this.ufos);
        this.ufos.add(new Ufo(random(width), random(height)));

        generateAsteroids();
    }

    void update() {
        if (this.state == State.PLAYING) {
            if (this.ship.hitsAsteroid() || ship.hitsUfo()) {
                this.lifeLost();
            }

            this.checkLevelFinished();
        }

        if (this.timerUfo != 0) {
            this.checkTimerUfo();
        }    
    }

    void draw() {
        pushStyle();
            if (this.showStarfield) {
                starfield.draw();
            }

            fill(219, 233, 255, random(200, 255));

            for (Asteroid asteroid : this.asteroids) {
                asteroid.update();
                asteroid.draw();
            }

            if (this.state != State.HOMESCREEN) {
                this.ship.update();
                this.ship.draw();
            }

            if (this.state == State.HOMESCREEN) {
                this.helpers.showHomescreen();
            } else if (this.state == State.PLAYING) {
                this.helpers.showScore(this.getScore());
                this.helpers.showRemainingLifes(this.lifes - 1);
                this.helpers.showLevel(this.level);
            } else if (this.state == State.NEXT_LEVEL) {
                this.helpers.showScore(this.getScore());
                this.helpers.showRemainingLifes(this.lifes - 1);
                this.helpers.showNextLevel(this.level);
            } else if (this.state == State.NEXT_LIFE) {
                this.helpers.showScore(this.getScore());
                this.helpers.showRemainingLifes(this.lifes);
                this.helpers.showLevel(this.level);
                this.helpers.showNextLife(this.lifes);
            } else if (this.state == State.GAME_OVER) {
                this.helpers.showScore(this.getScore());
                this.helpers.showGameOver();
            }

            if (this.ufos.size() > 0) {
                for (Ufo ufo : this.ufos) {
                    ufo.update();
                    ufo.draw();
                }
            }
        popStyle();
    }

    void startGame() {
        this.asteroidsHit = 0;
        this.ufosHit = 0;
        this.lifes = 3;
        this.level = 1;
        this.maxAsteroids = 10;
        this.ufos.clear();
        this.startNewLife();
    }

    void startNewLife() {
        this.state = State.PLAYING;

        this.generateAsteroids();
        this.ship = new Ship(this, this.asteroids, this.ufos);
        this.timerUfo = millis() + floor(random(2000, 5000));
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
            this.ufos.clear();
            this.timerUfo = 0;
            this.state = State.NEXT_LIFE;
        }
    }

    void endGame() {
        this.state = State.GAME_OVER;
    }

    void asteroidHit() {
        this.asteroidsHit++;
    }

    void ufoHit() {
        this.ufosHit++;

        if (this.ufos.size() == 0) {
            this.timerUfo = millis() + floor(random(2000, 5000));
        }
    }

    void keyPressed(int keyCode) {
        switch(this.state) {
            case HOMESCREEN:
            case GAME_OVER:
                if (keyCode == 80) { // p
                    this.startGame();
                }
                break;
            case PLAYING:
                if (keyCode == LEFT) {
                    this.ship.setRotation(-0.1);
                } else if (keyCode == RIGHT) {
                    this.ship.setRotation(0.1);
                } else if (keyCode == UP) {
                    this.ship.setBoost(true);
                } else if (keyCode == 32) { // SPACE 
                    this.ship.shoot();
                }
                break;
            case NEXT_LEVEL:
                if (keyCode == 80) { // p
                    this.startNewLevel();
                    break;
                }
            case NEXT_LIFE:
                if (keyCode == 80) { // p
                    this.startNewLife();
                    break;
                }
        }

        if (keyCode == 83) { // S
            this.showStarfield = !this.showStarfield;
        }
    }

    void keyReleased(int keyCode) {
        if (this.state == State.PLAYING) {
            if (keyCode == UP) {
                this.ship.setBoost(false);
            } else if (keyCode == LEFT || keyCode == RIGHT) {
                this.ship.setRotation(0);
            }
        }
    }

    private void checkLevelFinished() {
        if (this.asteroids.size() == 0) {
            this.ufos.clear();
            this.state = State.NEXT_LEVEL;
        }
    }

    private int getScore() {
        return this.asteroidsHit * 10 + this.ufosHit * 30;
    }

    private void generateAsteroids() {
        this.asteroids.clear();

        for (int i = 0; i < this.maxAsteroids; i++) {
            this.asteroids.add(new Asteroid());
        }
    }

    private void checkTimerUfo() {
        if (millis() > this.timerUfo) {
            this.timerUfo = 0;

            int randomSide = floor(random(4));
            PVector vector = PVector.random2D();

            switch(randomSide) {
                case 1:
                    vector.x += width;
                    break;
                case 2:
                    vector.x -= width;
                    break;
                case 3:
                    vector.y += height;
                    break;
                case 4:
                    vector.y -= height;
                    break;

            }

            for (int i = 0; i < this.level; i++) {
                this.ufos.add(new Ufo(vector.x, vector.y));
            }
        }
    }
}