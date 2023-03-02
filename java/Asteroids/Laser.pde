public class Laser extends Sprite {
    final static int LASER_SIZE = 36;

    Boolean isOffScreen = false;

    Laser(PVector position, float angle) {
        super(position, LASER_SIZE, PVector.fromAngle(angle).mult(10), 0);
    }

    @Override
    void draw() {
        push();

        stroke(Colors.EDGE);
        strokeWeight(4);
        point(this.position.x, this.position.y);

        pop();
    }

    Boolean checkWindowEdges() {
        this.isOffScreen = super.checkWindowEdges();

        return this.isOffScreen;
    }
}