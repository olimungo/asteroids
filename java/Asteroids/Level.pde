public class Level extends Overlay {
    void draw(int level) {
        push();

        fill(Colors.EDGE);
        textFont(Fonts.fontLight);
        textAlign(CENTER);
        textSize(25);

        text("LEVEL " + level, this.centerX, 55);

        pop();
    }
}