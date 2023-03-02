public class Patatoid extends Sprite {
    // final static ASTEROID_MINIMAL_DIAMETER_BREAKUP = 50;

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

        patatoids.add(new Patatoid(this.position.copy(), this.diameter, PVector.random2D(), this.rotationStep, this.sides));
        patatoids.add(new Patatoid(this.position.copy(), this.diameter, PVector.random2D(), this.rotationStep, this.sides));

        return patatoids;
    }

        // if (this.diameter > ASTEROID_MINIMAL_DIAMETER_BREAKUP) {
        //     const countnewAsteroids =
        //         this.diameter > (DIAMETER_MAX / 10) * 7 ? 3 : 2;

        //     const newAsteroids: Asteroid[] = [];

        //     for (let counter = 0; counter < countnewAsteroids; counter++) {
        //         newAsteroids.push(
        //             new Asteroid(
        //                 this.p5,
        //                 this.position.copy(),
        //                 this.diameter / countnewAsteroids,
        //                 this.sides,
        //                 this.rotationStep
        //             )
        //         );
        //     }

        //     return newAsteroids;
        // }

        // return [];

    // Boolean hit(float x, float y, float radius) {
    //     float distance = dist(this.position.x, this.position.y, x, y);

    //     if (distance < this.radius + radius) {
    //         return true;
    //     }

    //     return false;
    // }

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
