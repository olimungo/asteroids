public class Score extends Overlay {
    void draw(int score) {
        push();

        fill(Colors.EDGE);
        textFont(Fonts.fontLight);
        textAlign(RIGHT);
        textSize(25);

        text(score, width - 50, 55);

        pop();
    }
}