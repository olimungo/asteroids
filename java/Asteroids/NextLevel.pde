public class NextLevel extends Overlay {
    private int pastMillis = 0;
    private Boolean displayNextLevel = true;

    void draw(int level) {
        push();

        fill(Colors.EDGE);
        textAlign(CENTER);
        textSize(60);

        text(
            "LEVEL " + level + " COMPLETED",
            this.centerX,
            this.centerY
        );

        int now = millis();

        if (now - this.pastMillis > 1000) {
            this.displayNextLevel = !this.displayNextLevel;
            this.pastMillis = now;
        }

        if (this.displayNextLevel) {
            textFont(Fonts.fontLight);
            textSize(25);

            text(
                "PRESS \"S\" TO GO TO THE NEXT LEVEL",
                this.centerX,
                this.centerY + 50
            );
        }

        pop();
    }
}