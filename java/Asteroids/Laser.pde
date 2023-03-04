public class Laser extends Sprite {
    private final static int LASER_SIZE = 4;

    Boolean isOffScreen = false;

    Laser(PVector position, float angle) {
        super(position, LASER_SIZE, PVector.fromAngle(angle).mult(10), 0);
    }

    void draw() {
        push();

        stroke(Colors.EDGE);
        strokeWeight(LASER_SIZE);
        point(this.position.x, this.position.y);

        pop();
    }

    Boolean checkWindowEdges() {
        this.isOffScreen = super.checkWindowEdges();

        return this.isOffScreen;
    }
}