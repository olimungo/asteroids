public class Patatoid extends Sprite {
    final static int DIAMETER_MAX = 120;
    final static int ASTEROID_MINIMAL_DIAMETER_BREAKUP = 45;

    PGraphics shape;

    int sides = 0;

    Patatoid(
        PVector position,
        float diameter,
        PVector velocity,
        float rotationStep,
        int sides
    ) {
        super(position, diameter, velocity, rotationStep);

        this.sides = sides;
        this.shape = this.generateShape(this.generateVertices());
    }

    @Override
    void draw() {
        push();

        translate(this.position.x, this.position.y);

        rotate(this.rotation);

        image(this.shape, -this.diameter / 2, -this.diameter / 2);

        pop();
    }

    ArrayList<Patatoid> breakup() {
        ArrayList<Patatoid> patatoids = new ArrayList<Patatoid>();

        if (this.diameter > ASTEROID_MINIMAL_DIAMETER_BREAKUP) {
            int countNewAsteroids = this.diameter > DIAMETER_MAX * 0.7 ? 3 : 2;

            for (int counter = 0; counter < countNewAsteroids; counter++) {
                patatoids.add(new Patatoid(this.position.copy(), this.diameter / (countNewAsteroids * 0.7), PVector.random2D(), this.rotationStep, this.sides));
            }
        }

        return patatoids;
    }

    private ArrayList<PVector> generateVertices() {
        ArrayList<PVector> vertices = new ArrayList<PVector>();

        for (int side = 0; side < this.sides; side++) {
            float radius = random(
                this.diameter * 0.35,
                this.diameter * 0.5
            );

            float angle = map(side, 0, this.sides, 0, TWO_PI);
            float x = radius * cos(angle);
            float y = radius * sin(angle);

            vertices.add(new PVector(x, y));
        }

        return vertices;
    }

    private PGraphics generateShape(ArrayList<PVector> vertices) {
        PGraphics shape = createGraphics(Math.round(this.diameter), Math.round(this.diameter));

        shape.beginDraw();

        shape.translate(this.diameter / 2, this.diameter / 2);

        shape.strokeWeight(1.4);
        shape.stroke(255);
        shape.noFill();

        shape.beginShape();

        for (PVector vertex : vertices) {
            shape.vertex(vertex.x, vertex.y);
        }

        shape.endShape(CLOSE);

        shape.endDraw();

        return shape;
    }
}
