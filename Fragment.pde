public class Fragment extends Sprite {
    int sides;
    float rotationStep;

    private float rotation = 0;
    private PShape fragment;

    Fragment(float x, float y, float radius, int sides, float rotationStep) {
        super(x, y, radius);
        this.sides = sides;
        this.rotationStep = rotationStep;
        this.fragment = this.generateShape();
    }

    @Override
    void update() {
        super.update();
        this.rotation += this.rotationStep;
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            noFill();
            
            translate(this.position.x, this.position.y); 
            rotate(this.rotation);

            shape(this.fragment, 0, 0);
        popMatrix();
        popStyle();
    }

    private PShape generateShape() {       
        PShape shape = createShape();

        shape.beginShape();
        shape.noFill();
        shape.strokeWeight(1.4);
        shape.stroke(219, 233, 255);

        for (int i = 0; i < this.sides; i++) {
            float radius = this.radius + random(-this.radius / 3, this.radius / 3);
            float angle = map(i, 0, this.sides, 0, TWO_PI);
            float x = radius * cos(angle);
            float y = radius * sin (angle);

            shape.vertex(x, y);
        }

        shape.endShape(CLOSE);

        return shape;
    }
}