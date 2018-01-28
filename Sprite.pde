public class Sprite {
    PVector position;
    PVector velocity;
    float radius;

    Sprite(float x, float y, float radius) {
        this.position = new PVector(x, y);
        this.velocity = new PVector();
        this.radius = radius;
    }

    void update() {
        this.position.add(this.velocity);
        this.checkEdges();
    }

    void draw() {
        fill(255);
        noStroke();
        ellipse(this.position.x, this.position.y, this.radius, this.radius);
    }

    Boolean checkEdges() {
        if (this.position.x > width + this.radius) {
            this.position.x = -this.radius; 
            return true;
        } else if (this.position.x < -this.radius) {
            this.position.x = width + this.radius;
            return true;
        }

        if (this.position.y > height + this.radius) {
            this.position.y = -this.radius; 
            return true;
        } else if (this.position.y < -this.radius) {
            this.position.y = height + this.radius;
            return true;
        }

        return false;
    }
}