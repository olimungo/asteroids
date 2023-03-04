PFont fontThin;
PFont fontLight;
Boolean slowFrameRate = false;

GameManager gameManager;

void pre() {
}

void setup() {
    size(1024, 700, P2D);

    Fonts.setFontThin(createFont("Exo2-Thin.ttf", 100, true));
    Fonts.setFontLight(createFont("Exo2-Light.ttf", 100, true));

    textFont(Fonts.fontThin);

    gameManager = new GameManager();
}

void draw() {
    background(Colors.BACKGROUND);

    gameManager.update();
    gameManager.draw();
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
    loop();
}

void keyPressed() {
    gameManager.keyPressed(keyCode);
}

void keyReleased() {
    gameManager.keyReleased(keyCode);
}