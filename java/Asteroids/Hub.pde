public class Hub extends Overlay {
    private String frame = "";

    @Override
    void draw() {
        push();

        stroke(Colors.DARK);
        noFill();

        rect(0, 0, width, height);

        line(0, this.centerY, width, this.centerY);
        line(this.centerX, 0, this.centerX, height);

        ellipse(this.centerX, this.centerY, 400, 400);

        if (frameCount % 10 == 0 || frameCount < 5) {
            this.frame = String.format("%2.0f", frameRate);
        }

        fill(Colors.DARK);
        textFont(Fonts.fontLight);
        textSize(30);
        textAlign(LEFT);
        text(this.frame, 30, height - 45);
        text("FPS", 70, height - 45);

        pop();
    }
}