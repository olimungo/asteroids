ArrayList<Asteroid> asteroids;
PFont font;
Helpers helpers;
Boolean slowFrameRate = false;

float middleWidth = 1000 / 2;
float middleHeight = 800 / 2;

GameManager gameManager;
Starfield starfield;
Ship ship;
int maxAsteroids = 15;

void pre() {
}

void setup() {
    size(1000, 800, P2D);

    // fullScreen();
    // frameRate(5);

    helpers = new Helpers();

    font = createFont("Megrim.ttf", 48);
    textFont(font);

    starfield = new Starfield();
    asteroids = new ArrayList<Asteroid>();

    generateAsteroids();

    gameManager = new GameManager();
    ship = new Ship(gameManager, asteroids);
}

void draw() {
    background(0);

    // helpers.translateSketch(1.5);
    // helpers.drawPattern();

    starfield.draw();

    for (Asteroid asteroid : asteroids) {
        asteroid.update();
        asteroid.draw();
    }

    if (gameManager.state == State.PLAYING || gameManager.state == State.GAME_OVER) {
        ship.update();
        ship.draw();

        if (gameManager.state == State.PLAYING) {
            if (ship.hitsAsteroid()) {
                gameManager.endGame();
            }
        }
    }

    gameManager.draw();

    // helpers.showFrameRate();
}

void generateAsteroids() {
    asteroids.clear();

    for (int i = 0; i < maxAsteroids; i++) {
        asteroids.add(new Asteroid());
    }
}

void mousePressed() {
    if (slowFrameRate) {
        frameRate(60);
    } else {
        frameRate(10);
    }

    slowFrameRate = !slowFrameRate;
    noLoop();
}

void mouseReleased() {
    // frameRate(60);
    loop();
}

void keyPressed() {
    if (gameManager.state == State.HOMESCREEN || gameManager.state == State.GAME_OVER) {
        if (keyCode == 32) {
            generateAsteroids();
            ship = new Ship(gameManager, asteroids);
            gameManager.startGame();
        }
    } else if (gameManager.state == State.PLAYING) {
        if (keyCode == LEFT) {
            ship.setRotation(-0.1);
        } else if (keyCode == RIGHT) {
            ship.setRotation(0.1);
        } else if (keyCode == UP) {
            ship.setBoost(true);
        } else if (keyCode == 32) { // SPACE 
            ship.shoot();
        }
    }
}

void keyReleased() {
    if (gameManager.state == State.PLAYING) {
        if (keyCode == UP) {
            ship.setBoost(false);
        } else if (keyCode == LEFT || keyCode == RIGHT) {
            ship.setRotation(0);
        }
    }
}