// PFont fontThin;
// PFont fontLight;
// Helpers helpers;
// Boolean slowFrameRate = false;
// Boolean fineTuning = false;

// float middleWidth = 1024 / 2;
// float middleHeight = 700 / 2;

// GameManager gameManager;

Sprite sprite;
Patatoid patatoid;

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
    sprite = new Sprite(new PVector(100,100), 50, 0.01);
    patatoid = new Patatoid(new PVector(200,200), 50, 8, 0.01);
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

    sprite.update();
    sprite.draw();

    patatoid.update();
    patatoid.draw();
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
    // if (keyCode == 68) { // D
    //     this.fineTuning = !this.fineTuning;
    // }
    // gameManager.keyPressed(keyCode);
}

void keyReleased() {
    // gameManager.keyReleased(keyCode);
}