public class Fragment extends Sprite {
    int sides;
    float rotationStep;

    private float[] offsets;
    private float rotation = 0;

    Fragment(float x, float y, float radius, int sides, float rotationStep) {
        super(x, y, radius);
        this.sides = sides;
        this.rotationStep = rotationStep;
        this.offsets = new float[this.sides];

        for (int i = 0; i < this.sides; i++) {
            this.offsets[i] = random(-this.radius / 3, this.radius / 3);
        }
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

            beginShape();
                for (int i = 0; i < this.sides; i++) {
                    float radius = this.radius + this.offsets[i];
                    float angle = map(i, 0, this.sides, 0, TWO_PI);
                    float x = radius * cos(angle);
                    float y = radius * sin (angle);

                    float alpha = random(170, 255);
                    stroke(219, 233, 255, alpha);
                    strokeWeight(1.4);
                    
                    vertex(x, y);
                }
            endShape(CLOSE);

            for (int i = 0; i < this.sides; i++) {
                float radius = this.radius + this.offsets[i];
                float angle = map(i, 0, this.sides, 0, TWO_PI);
                float x = radius * cos(angle);
                float y = radius * sin (angle);

                stroke(219, 233, 255, 10);

                ellipse(x, y, 2, 2);
            }
        popMatrix();
        popStyle();
    }
}