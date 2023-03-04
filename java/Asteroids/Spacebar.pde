public class Spacebar extends Overlay {
    private final static int WIDTH = 200;
    private final static int HEIGHT = 30;

    private PVector position;

    Spacebar(PVector position) {
        this.position = position;
    }

    void draw() {
        push();

        translate(this.position.x, this.position.y);

        fill(Colors.BACKGROUND);
        stroke(Colors.EDGE);
        strokeWeight(1);

        translate(-this.WIDTH / 2, -this.HEIGHT / 2);

        rect(0, 0, this.WIDTH, this.HEIGHT, this.HEIGHT / 5);

        fill(Colors.EDGE);
        noStroke();
        textFont(Fonts.fontLight);
        textAlign(CENTER);
        textSize(15);
        text("SPACEBAR", this.WIDTH / 2, this.HEIGHT / 2 + 5);

        pop();
    }
}