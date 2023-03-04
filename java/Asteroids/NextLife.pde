public class NextLife extends Overlay {
    private int pastMillis = 0;
    private Boolean displayRetry = true;

    void draw(int lifes) {
        push();

        fill(Colors.EDGE);
        textAlign(CENTER);
        textSize(60);

        String message = lifes + " LIFE";

        if (lifes > 1) {
            message += "S LEFT";
        } else {
            message += " LEFT";
        }

        text(message, this.centerX, this.centerY);

        int now = millis();

        if (now - this.pastMillis > 1000) {
            this.displayRetry = !this.displayRetry;
            this.pastMillis = now;
        }

        if (this.displayRetry) {
            textFont(Fonts.fontLight);
            textSize(25);

            text(
                "PRESS \"S\" TO RE-TRY",
                this.centerX,
                this.centerY + 50
            );
        }

        pop();
    }
}