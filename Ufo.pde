public class Ufo extends Sprite {
    int ufoWidth = 60;
    int ufoHeight = 25;

    private PVector[] shapeVectors;
    private PShape ufo;

    Ufo(float x, float y) {
        super(x, y, 50);
        this.velocity = PVector.random2D();
        this.velocity.setMag(2.5);

        this.shapeVectors = this.generateShapeVectors();
    }

    @Override
    void update() {
        super.update();
        this.ufo = this.generateShape(this.shapeVectors);
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            translate(this.position.x, this.position.y);
            shape(this.ufo, 0, 0);
        popMatrix();
        popStyle();
    }

    Boolean hit(float x, float y, float radius) {
        println(radius);
        if ((x + radius > this.position.x && x + radius < this.position.x + this.ufoWidth) &&
            (y + radius > this.position.y && y + radius < this.position.y + this.ufoHeight)) {
                return true;
        }

        return false;
    }

    private PVector[] generateShapeVectors() {
        PVector[] vectors = new PVector[12];

        float height1 = this.ufoHeight / 4;
        float height2 = this.ufoHeight / 8 * 5;
        float width1 = this.ufoWidth / 3;
        float width2 = this.ufoWidth * 0.66;
        float width3 = this.ufoWidth / 10 * 6;
        float width4 = this.ufoWidth / 10 * 4;

        vectors[0] = new PVector(0, height2);
        vectors[1] = new PVector(width1, this.ufoHeight);
        vectors[2] = new PVector(width2, this.ufoHeight);
        vectors[3] = new PVector(this.ufoWidth, height2);
        vectors[4] = new PVector(0, height2);
        vectors[5] = new PVector(width1, height1);
        vectors[6] = new PVector(width2, height1);
        vectors[7] = new PVector(this.ufoWidth, height2);
        vectors[8] = new PVector(width2, height1);
        vectors[9] = new PVector(width3, 0);
        vectors[10] = new PVector(width4, 0);
        vectors[11] = new PVector(width1, height1);

        return vectors;
    }

    private PShape generateShape(PVector[] vectors) {
        PShape shape = createShape();

        shape.beginShape();
        shape.noFill();

        for (int i = 0 ; i < vectors.length; i++) {
            float alpha = random(170, 255);
            shape.stroke(219, 233, 255, alpha);
            shape.vertex(vectors[i].x, vectors[i].y);
        }

        shape.endShape();

        return shape;
    }
}