public class Keycap extends Overlay {
    private final static int SIZE = 30;

    private PVector position;
    private float angle;

    Keycap(PVector position, float angle) {
        this.position = position;
        this.angle = angle; 
    }

    void draw() {
        push();

        translate(this.position.x, this.position.y);

        rotate(radians(this.angle));

        fill(Colors.BACKGROUND);
        stroke(Colors.EDGE);
        strokeWeight(1);

        translate(-this.SIZE / 2, -this.SIZE / 2);

        rect(0, 0, this.SIZE, this.SIZE, this.SIZE / 5);

        fill(Colors.EDGE);
        triangle(
            this.SIZE / 2,
            (this.SIZE / 10) * 3,
            (this.SIZE / 10) * 7,
            (this.SIZE / 10) * 7,
            (this.SIZE / 10) * 3,
            (this.SIZE / 10) * 7
        );
        
        pop();
    }
}