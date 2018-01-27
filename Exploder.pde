public class Exploder extends Sprite {
    Exploder(PVector position, float angle, float radius, int pieces) {
        super(position.x, position.y, radius);
        this.velocity = PVector.fromAngle(angle).mult(10);
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            stroke(255);
            strokeWeight(4);
        popMatrix();
        popStyle();
    }
}