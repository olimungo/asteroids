public class Laser extends Sprite {
    Boolean isOffScreen = false;

    Laser(PVector position, float angle) {
        super(position.x, position.y, 0);
        this.velocity = PVector.fromAngle(angle).mult(10);
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            stroke(255);
            strokeWeight(4);
            point(this.position.x, this.position.y);
        popMatrix();
        popStyle();
    }

    @Override
    void checkEdges() {
        if (this.position.x > width + this.radius) {
            this.isOffScreen = true; 
        } else if (this.position.x < -this.radius) {
            this.isOffScreen = true; 
        }

        if (this.position.y > height + this.radius) {
            this.isOffScreen = true; 
        } else if (this.position.y < -this.radius) {
            this.isOffScreen = true; 
        }
    }

    Boolean hits(ArrayList<Asteroid> asteroids) {
        Boolean hit = false;

        for (int i = asteroids.size() - 1; i >= 0; i--) {
            Asteroid asteroid = asteroids.get(i);
            float distance = dist(this.position.x, this.position.y, asteroid.position.x, asteroid.position.y);

            if (distance < asteroid.radius) {
                if (asteroid.radius > 15) {
                    asteroids.addAll(asteroid.breakup());
                }

                asteroids.remove(asteroid);
                hit = true;
            }
        }

        return hit;
    }
}