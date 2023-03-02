public class Patatoid extends Sprite {
    PGraphics shape;

    int sides = 0;

    Patatoid(
        PVector position,
        float diameter,
        int sides,
        float rotationStep
    ) {
        super(position, diameter, rotationStep);

        this.sides = sides;

        this.shape = this.generateShape(this.generateVertices());
    }

    @Override
    void draw() {
        pushStyle();

        translate(this.position.x, this.position.y);

        rotate(this.rotation);

        image(this.shape, -this.diameter / 2, -this.diameter / 2);

        popStyle();
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
