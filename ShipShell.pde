public class ShipShell extends Sprite {
    float heading;

    private PShape shipShell;

    ShipShell(float x, float y) {
        super(x, y, 18);
        this.shipShell = this.generateShape();
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            translate(this.position.x, this.position.y);
            rotate(heading + PI / 2);
            
            shape(this.shipShell, 0, 0);
        popMatrix();
        popStyle();
    }

    private PShape generateShape() {
        PShape shape = createShape();
        float smallerRadius = this.radius / 5 * 3.5;

        shape.beginShape();
        shape.noFill();
        shape.strokeWeight(1.4);
        shape.stroke(219, 233, 255);
        shape.vertex(smallerRadius, this.radius);
        shape.vertex(0, -this.radius);
        shape.vertex(-smallerRadius, this.radius);
        shape.bezierVertex(0, smallerRadius, 0, smallerRadius, smallerRadius, this.radius);
        shape.endShape();

        return shape;
    }
}
