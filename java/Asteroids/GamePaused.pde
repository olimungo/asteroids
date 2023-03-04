public class GamePaused extends Overlay {
    void draw() {
        push();

        fill(Colors.EDGE);
        textFont(Fonts.fontLight);
        textAlign(CENTER);
        textSize(60);

        text("GAME PAUSED ", this.centerX, this.centerY - 100);

        pop();
    }
}