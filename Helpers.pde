public class Helpers {
    float middleWidth = width / 2;
    float middleHeight = height / 2;
    String frameRateMessage = "";

    void translateSketch(float ratio) {
        scale(1/ratio);
        translate((width * ratio - width) / 2, (height * ratio - height) / 2);
    }

    void drawPattern() {
        stroke(255);
        noFill();
        rect(0, 0, width, height);
        line(0, middleHeight, width, middleHeight);
        line(middleWidth, 0, middleWidth, height);
        ellipse(middleWidth, middleHeight, 400, 400);
    }

    void showFrameRate() {
        if (frameCount % 5 == 0 || frameCount < 5) {
            this.frameRateMessage = String.format("%2.0f / %d / %d / %d + %d", frameRate, frameCount, millis(), mouseX, mouseY);
        }

        fill(255, 255, 255, 100);
        textSize(30);
        textAlign(LEFT);
        text(this.frameRateMessage, 30, height - 40);
    }
}