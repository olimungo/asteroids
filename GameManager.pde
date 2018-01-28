
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
    private int timerUfo = 0;

    GameManager() {
        this.helpers = new Helpers();
        this.starfield = new Starfield();
        this.ufo = new Ufo(random(width), random(height));
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

        if (this.timerUfo != 0) {
            this.checkTimerUfo();
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

            if (this.ufo != null) {
                this.ufo.update();
                this.ufo.draw();
            }
        popStyle();
    }

    void startGame() {
        this.asteroidsHit = 0;
        this.lifes = 3;
        this.startNewLife();
    }

    void startNewLife() {
        this.state = State.PLAYING;

        this.generateAsteroids();
        this.ship = new Ship(this, asteroids);
        this.ufo = null;
        this.timerUfo = millis() + floor(random(1000, 2000));
    }

    void lifeLost() {
        this.lifes--;

        if (this.lifes == 0) {
            this.endGame();
        } else {
            this.state = State.NEXT_LIFE;
        }
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

    void checkTimerUfo() {
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

            this.ufo = new Ufo(vector.x, vector.y);
        }
    }
}