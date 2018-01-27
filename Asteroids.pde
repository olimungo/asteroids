ArrayList<Asteroid> asteroids;
PFont font;
Helpers helpers;
Boolean slowFrameRate = false;

float middleWidth = 1000 / 2;
float middleHeight = 800 / 2;

Starfield starfield;
Ship ship;
int maxAsteroids = 20;

void pre() {
}

void setup() {
    size(1000, 800, P2D);

    //fullScreen();
    // frameRate(5);

    helpers = new Helpers();

    font = loadFont("HelveticaNeue-48.vlw");
    textFont(font, 48);

    starfield = new Starfield();
    asteroids = new ArrayList<Asteroid>();

    for (int i = 0; i < maxAsteroids; i++) {
        asteroids.add(new Asteroid());
    }

    ship = new Ship(asteroids);
}

void draw() {
    background(0);

    helpers.translateSketch(1.5);
    helpers.drawPattern();

    starfield.draw();

    for (Asteroid asteroid : asteroids) {
        asteroid.update();
        asteroid.draw();
    }

    ship.update();
    ship.draw();

    if (ship.hitsAsteroid()) {
        noLoop();
        println("hit");
    }

    // helpers.showFrameRate();
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
    if (keyCode == LEFT) {
        ship.setRotation(-0.1);
    } else if (keyCode == RIGHT) {
        ship.setRotation(0.1);
    } else if (keyCode == UP) { // SPACE 
        ship.setBoost(true);
    } else if (keyCode == 32) {
        ship.shoot();
    }
}

void keyReleased() {
    if (keyCode == UP) {
        ship.setBoost(false);
    } else {
        ship.setRotation(0);
    }
}