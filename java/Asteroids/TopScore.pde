public class TopScore extends Overlay {
    void draw(int score) {
        if (score > 0) {
            push();

            fill(Colors.EDGE);
            textFont(Fonts.fontLight);
            textAlign(CENTER);
            textSize(25);

            text("TOP   " + score, this.centerX + 220, 55);

            pop();
        }
    }
}