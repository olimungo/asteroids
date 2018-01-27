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

    void checkEdges() {
        if (this.position.x > width + this.radius) {
            this.position.x = -this.radius; 
        } else if (this.position.x < -this.radius) {
            this.position.x = width + this.radius;
        }

        if (this.position.y > height + this.radius) {
            this.position.y = -this.radius; 
        } else if (this.position.y < -this.radius) {
            this.position.y = height + this.radius;
        }
    }

    Asteroid hits(ArrayList<Asteroid> sprites) {
        for (int i = sprites.size() - 1; i >= 0; i--) {
            Asteroid sprite = sprites.get(i);
            float distance = dist(this.position.x, this.position.y, sprite.position.x, sprite.position.y);

            if (distance < this.radius + sprite.radius) {
                return sprite;
            }
        }

        return null;
    }
}