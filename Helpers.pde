public class Helpers {
    float middleWidth = width / 2;
    float middleHeight = height / 2;
    String frameRateMessage = "";

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
            textSize(80);
            text("ASTEROIDS", width / 2, height / 2);
            textSize(40);
            text("PRESS SPACE TO PLAY", width / 2, height / 2 + 50);
        popStyle();
    }

    void showScore(int score) {
        pushStyle();
            textSize(50);
            textAlign(RIGHT);
            text(score, width - 80, 80);
        popStyle();
    }

    void showGameOver() {
        pushStyle();
            textAlign(CENTER);
            textSize(80);
            text("GAME OVER", width / 2, height / 2);
            textSize(40);
            text("PRESS SPACE TO PLAY AGAIN", width / 2, height / 2 + 50);
        popStyle();
    }
}