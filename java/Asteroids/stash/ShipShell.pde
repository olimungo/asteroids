public class ShipShell extends Sprite {
    float heading;

    ShipShell(float x, float y) {
        super(x, y, 18);
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            translate(this.position.x, this.position.y);
            rotate(heading + PI / 2);
            
            noFill();
            strokeWeight(1.4);

            float alpha1 = random(170, 255);
            float alpha2 = random(170, 255);
            float alpha3 = random(170, 255);

            float smallerRadius = this.radius / 5 * 3.5;

            stroke(219, 233, 255, alpha1);
            bezier(-smallerRadius, this.radius, 0, smallerRadius, 0, smallerRadius, smallerRadius, this.radius);
            stroke(219, 233, 255, 30);
            ellipse(smallerRadius, this.radius, 2, 2);
            stroke(219, 233, 255, alpha2);
            line(smallerRadius, this.radius, 0, -this.radius);
            stroke(219, 233, 255, 30);
            ellipse(0, -this.radius, 2, 2);
            stroke(219, 233, 255, alpha3);
            line(0, -this.radius, -smallerRadius, this.radius);
            stroke(219, 233, 255, 30);
            ellipse(-smallerRadius, this.radius, 2, 2);
        popMatrix();
        popStyle();
    }
}
