public class Helpers {
    float middleWidth = width / 2;
    float middleHeight = height / 2;
    String frameRateMessage = "";

    private ShipShell[] shipShells;

    Helpers() {
        shipShells = new ShipShell[3];
        int x = 20;

        for (int i = 0; i < 3; i++) {
            x += 50;
            ShipShell shipShell = new ShipShell(x, 60);
            shipShell.heading = -PI / 2;
            this.shipShells[i] = shipShell;
        }
    }

    void translateSketch(float ratio) {
        scale(1/ratio);
        translate((width * ratio - width) / 2, (height * ratio - height) / 2);
    }

    void drawPattern() {
        pushStyle();
            stroke(255);
            noFill();
            rect(0, 0, width, height);
            line(0, middleHeight, width, middleHeight);
            line(middleWidth, 0, middleWidth, height);
            ellipse(middleWidth, middleHeight, 400, 400);
        popStyle();
    }

    void showFrameRate() {
        if (frameCount % 5 == 0 || frameCount < 5) {
            this.frameRateMessage = String.format("%2.0f / %d / %d / %d + %d", frameRate, frameCount, millis(), mouseX, mouseY);
        }

        pushStyle();
            fill(255, 255, 255, 100);
            textSize(30);
            textAlign(LEFT);
            text(this.frameRateMessage, 30, height - 40);
        popStyle();
    }

    void showHomescreen() {
        pushStyle();
            textAlign(CENTER);
            textSize(100);
            text("ASTEROIDS", width / 2, height / 2 - 100);
            textSize(50);
            text("INSERT 1 COIN", width / 2, height / 2 + 50);
            textFont(fontLight);
            textSize(25);
            text("OR PRESS \"P\" TO PLAY", width / 2, height / 2 + 90);
        popStyle();
    }

    void showScore(int score) {
        pushStyle();
            textSize(50);
            textAlign(RIGHT);
            text(score, width - 80, 80);
        popStyle();
    }

    void showRemainingLifes(int lifes) {
        pushStyle();
            for (int i = 0; i < lifes; i++) {
                this.shipShells[i].draw();
            }
        popStyle();
    }

    void showNextLife(int lifes) {
        pushStyle();
            textAlign(CENTER);
            textSize(60);
            String msg = lifes + " LIFE";

            if (lifes > 1) {
                msg += "S LEFT";
            } else {
                msg += " LEFT";
            }

            text(msg, width / 2, height / 2);
            textFont(fontLight);
            textSize(25);
            text("PRESS \"P\" TO RE-TRY", width / 2, height / 2 + 50);
        popStyle();
    }

    void showGameOver() {
        pushStyle();
            textAlign(CENTER);
            textSize(80);
            text("GAME OVER", width / 2, height / 2);
            textFont(fontLight);
            textSize(25);
            text("PRESS \"P\" TO PLAY AGAIN", width / 2, height / 2 + 50);
        popStyle();
    }
}