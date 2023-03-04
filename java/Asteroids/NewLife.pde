public class NewLife extends Overlay {
    private int pastMillis = 0;
    private Boolean displayNewLife = true;

    void draw() {
        int now = millis();

        if (now - this.pastMillis > 500) {
            this.displayNewLife = !this.displayNewLife;
            this.pastMillis = now;
        }

        if (this.displayNewLife) {
            push();

            textFont(Fonts.fontThin);
            fill(Colors.EDGE);
            textFont(Fonts.fontLight);
            textAlign(CENTER);
            textSize(20);

            text("NEW LIFE!", this.centerX, 85);

            pop();
        }
    }
}