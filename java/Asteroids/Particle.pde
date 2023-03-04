public class Particle extends Sprite {
    private final static int POLYGON_SIDES = 5;
    private final static int HEALTH_DECREMENT = 5;
    private final static int MINIMAL_HEALTH = 50;

    private int health;

    Particle(PVector position, float diameter) {
        super(position, diameter, PVector.random2D(), 0);

        this.health = int(random(175, 230));
    }

    Boolean update() {
        super.update();

        this.health -= HEALTH_DECREMENT;

        if (this.health <= MINIMAL_HEALTH) {
            return false;
        }

        return true;
    }

    void draw() {
        push();

        stroke(Colors.EDGE);
        noFill();

        this.polygon(this.position, this.diameter, POLYGON_SIDES);

        pop();
    }

    private void polygon(PVector position, float radius, int sides) {
        float angle = TWO_PI / sides;

        beginShape();

        for (int a = 0; a < TWO_PI; a += angle) {
            float sx = position.x + cos(a) * radius;
            float sy = position.y + sin(a) * radius;

            vertex(sx, sy);
        }

        endShape(CLOSE);
    }
}
