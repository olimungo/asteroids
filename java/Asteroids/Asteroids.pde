// PFont fontThin;
// PFont fontLight;
// Helpers helpers;
// Boolean slowFrameRate = false;
// Boolean fineTuning = false;

// float middleWidth = 1024 / 2;
// float middleHeight = 700 / 2;

// GameManager gameManager;

Patatoid patatoid;
ArrayList<Patatoid> patatoids;
Ship ship;
OverlaysManager overlaysManager;
int timerShoot = 3000;

void pre() {
}

void setup() {
    size(1024, 700, P2D);

    // fullScreen();
    // frameRate(10);

    // helpers = new Helpers();

    // fontThin = createFont("Exo2-Thin.ttf", 100);
    // fontLight = createFont("Exo2-Light.ttf", 100);
    // textFont(fontThin);

    // gameManager = new GameManager();

    overlaysManager = new OverlaysManager();

    patatoid = new Patatoid(PVector.random2D(), 46, PVector.random2D(), 0.01, 8);
    ship = new Ship(new PVector(width / 2, height / 2), false);
}

void draw() {
    background(0);

    // if (fineTuning) {
    //     helpers.scaleStage(1.5);
    //     helpers.drawPattern();
    // }

    // gameManager.update();
    // gameManager.draw();

    // if (fineTuning) {
    //     helpers.showFrameRate();
    // }

    if (millis() > timerShoot) {
        timerShoot = 50000000;
        patatoids = patatoid.breakup();
    }

    if (patatoids != null) {
        for (Patatoid patatoid : patatoids) {
            patatoid.update();
            patatoid.draw();
        }
    } else {
        patatoid.update();
        patatoid.draw();
    }

    ship.update();
    ship.draw();

    overlaysManager.draw();
}

void mousePressed() {
    // if (slowFrameRate) {
    //     frameRate(60);
    // } else {
    //     frameRate(10);
    // }

    // slowFrameRate = !slowFrameRate;
    // noLoop();
}

void mouseReleased() {
    // loop();
}

void keyPressed() {
    // println(keyCode);

    switch (keyCode) {
        case 37: // LEFT ARROW
            ship.setRotation(-0.1);
            break;
        case 39: // RIGHT ARROW
            ship.setRotation(0.1);
            break;
        case 38: // UP ARROW
            ship.setBoost(true);
            break;
        case 32: // SPACE
            ship.shoot();
            break;
    }
}

void keyReleased() {
    switch (keyCode) {
        case 37: // LEFT ARROW
        case 39: // RiGHT ARROW
            ship.setRotation(0);
            break;
        case 38: // UP ARROW
            ship.setBoost(false);
            break;
    }
}