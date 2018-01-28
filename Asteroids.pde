PFont fontThin;
PFont fontLight;
Helpers helpers;
Boolean slowFrameRate = false;

float middleWidth = 1000 / 2;
float middleHeight = 800 / 2;

GameManager gameManager;

void pre() {
}

void setup() {
    size(1000, 800, P2D);

    // fullScreen();
    // frameRate(5);

    helpers = new Helpers();

    fontThin = createFont("Exo2-Thin.ttf", 100);
    fontLight = createFont("Exo2-Light.ttf", 100);
    textFont(fontThin);

    gameManager = new GameManager();
}

void draw() {
    background(0);

    // helpers.translateSketch(1.5);
    // helpers.drawPattern();

    gameManager.update();
    gameManager.draw();

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
    gameManager.keyPressed(keyCode);
}

void keyReleased() {
    gameManager.keyReleased(keyCode);
}