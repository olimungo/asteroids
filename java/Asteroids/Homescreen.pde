public class Homescreen extends Overlay {
    private int pastMillis = 0;
    private Boolean displayInsertCoin = true;

    void draw() {
        push();

        fill(Colors.EDGE);
        textAlign(CENTER);

        textSize(100);
        text("ASTEROIDS", this.centerX, this.centerY - 100);

        textFont(Fonts.fontLight);
        textSize(15);
        text("by olimungo", this.centerX, this.centerY - 65);

        int now = millis();

        if (now - this.pastMillis > 1000) {
            this.displayInsertCoin = !this.displayInsertCoin;
            this.pastMillis = now;
        }

        textFont(Fonts.fontThin);

        if (this.displayInsertCoin) {
            textSize(40);
            text("INSERT 1 COIN", this.centerX, this.centerY + 50);
        }

        textFont(Fonts.fontLight);
        textSize(25);
        text("PRESS \"H\" FOR HELP", this.centerX, this.centerY + 250);

        pop();
    }
}