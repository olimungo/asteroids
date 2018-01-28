
public static enum State {
    HOMESCREEN, PLAYING, NEXT_LIFE, GAME_OVER 
}

public class GameManager {
    int asteroidsHit;
    int lifes;
    State state = State.HOMESCREEN;

    private Starfield starfield;
    private int maxAsteroids = 15;
    private ArrayList<Asteroid> asteroids;
    private Helpers helpers;
    private Ufo ufo;
    private Ship ship;

    GameManager() {
        this.helpers = new Helpers();
        this.starfield = new Starfield();
        this.ufo = new Ufo(500, 500);
        this.asteroids = new ArrayList<Asteroid>();
        this.ship = new Ship(gameManager, asteroids);

        generateAsteroids();

    }

    void update() {
        if (this.state == State.PLAYING) {
            if (this.ship.hitsAsteroid()) {
                this.lifeLost();
            }
        }     
    }

    void draw() {
        pushStyle();
            starfield.draw();

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
                this.ufo.update();
                this.ufo.draw();
            } else if (this.state == State.PLAYING) {
                this.helpers.showScore(this.getScore());
                this.helpers.showRemainingLifes(this.lifes - 1);
            } else if (this.state == State.NEXT_LIFE) {
                this.helpers.showScore(this.getScore());
                this.helpers.showRemainingLifes(this.lifes);
                this.helpers.showNextLife(this.lifes);
            } else if (this.state == State.GAME_OVER) {
                this.helpers.showScore(this.getScore());
                this.helpers.showGameOver();
            }
        popStyle();
    }

    void startGame() {
        this.asteroidsHit = 0;
        this.lifes = 3;
        this.state = State.PLAYING;
    }

    void lifeLost() {
        this.lifes--;

        if (this.lifes == 0) {
            this.endGame();
        } else {
            this.state = State.NEXT_LIFE;
        }
    }

    void startNewLife() {
        this.state = State.PLAYING;
        this.generateAsteroids();
        this.ship = new Ship(this, asteroids);
    }

    void endGame() {
        this.state = State.GAME_OVER;
    }

    void asteroidHit() {
        this.asteroidsHit++;
    }

    void keyPressed(int keyCode) {
        switch(this.state) {
            case HOMESCREEN:
            case GAME_OVER:
                if (keyCode == 80) { // p
                    this.startNewLife();
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
            case NEXT_LIFE:
                if (keyCode == 80) { // p
                    this.startNewLife();
                    break;
                }
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

    private int getScore() {
        return this.asteroidsHit * 10;
    }

    void generateAsteroids() {
        this.asteroids.clear();

        for (int i = 0; i < this.maxAsteroids; i++) {
            this.asteroids.add(new Asteroid());
        }
    }
}