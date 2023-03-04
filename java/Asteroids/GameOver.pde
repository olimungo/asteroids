public class GameOver extends Overlay {
    private int pastMillis = 0;
    private Boolean displayPlayAgain = true;

    void draw() {
        push();

        fill(Colors.EDGE);
        textAlign(CENTER);

        textSize(80);
        text("GAME OVER", this.centerX, this.centerY);

        int now = millis();

        if (now - this.pastMillis > 1000) {
            this.displayPlayAgain = !this.displayPlayAgain;
            this.pastMillis = now;
        }

        if (this.displayPlayAgain) {
            textFont(Fonts.fontLight);

            textSize(25);
            text(
                "PRESS \"S\" TO PLAY AGAIN",
                this.centerX,
                this.centerY + 50
            );
        }

        pop();
    }
}