public class Sprite {
    PVector position;
    PVector velocity;
    float diameter;
    float rotation;
    float rotationStep;

    Sprite(PVector position, float diameter, PVector velocity, float rotationStep) {
        this.position = position;
        this.diameter = diameter;
        this.velocity = velocity;
        this.rotationStep = rotationStep;
    }

    void update() {
        this.position.add(this.velocity);
        this.checkWindowEdges();

        this.rotation += this.rotationStep;
    }

    void draw() {
        push();

        translate(this.position.x, this.position.y);
        rotate(this.rotation);

        fill(Colors.EDGE);
        noStroke();

        ellipse(0, 0, this.diameter, this.diameter);

        fill(Colors.DARK);
        rectMode(CENTER);
        rect(0, 0, this.diameter / 4, this.diameter / 4);

        pop();
    }

    Boolean checkWindowEdges() {
        float radius = this.diameter / 2;
        float x = this.position.x;
        float y = this.position.y;

        if (x > width + radius) {
            this.position.x = -radius;
            return true;
        } else if (x < -radius) {
            this.position.x = width + radius;
            return true;
        }

        if (y > height + radius) {
            this.position.y = -radius;
            return true;
        } else if (y < -radius) {
            this.position.y = height + radius;
            return true;
        }

        return false;
    }
}