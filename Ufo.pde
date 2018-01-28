public class Ufo extends Sprite {
    Ufo(float x, float y) {
        super(x, y, 50);
        this.velocity = PVector.random2D();
        this.velocity.setMag(2);
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            float radius = this.radius;
            float radius10 = radius / 10;
            float radius20 = radius10 * 2;
            float radius30 = radius10 * 3;
            float radius40 = radius10 * 4;

            translate(this.position.x, this.position.y);
            
            this.setColor();
            line(radius30, radius - radius20, radius - radius30, radius - radius20);
            this.setColor();
            line(0, radius - radius40, radius, radius - radius40);
            this.setColor();
            line(radius30, radius40, radius - radius30, radius40);
            this.setColor();
            line(radius40, radius30, radius - radius40, radius30);

            this.setColor();
            line(radius30, radius - radius20, 0, radius - radius40);
            this.setColor();
            line(0, radius - radius40, radius30, radius40);
            this.setColor();
            line(radius30, radius40, radius40, radius30);

            this.setColor();
            line(radius - radius30, radius - radius20, radius, radius - radius40);
            this.setColor();
            line(radius, radius - radius40, radius - radius30, radius40);
            this.setColor();
            line(radius - radius30, radius40, radius - radius40, radius30);
        popMatrix();
        popStyle();
    }

    private void setColor() {
        float alpha = random(170, 255);
        stroke(219, 233, 255, alpha);
    }
}