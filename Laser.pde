public class Laser extends Sprite {
    Boolean isOffScreen = false;

    private ArrayList<Asteroid> asteroids;
    private ArrayList<Ufo> ufos;

    Laser(PVector position, float angle, ArrayList<Asteroid> asteroids, ArrayList<Ufo> ufos) {
        super(position.x, position.y, 1);
        this.velocity = PVector.fromAngle(angle).mult(10);
        this.asteroids = asteroids;
        this.ufos = ufos;
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
    Boolean checkEdges() {
        this.isOffScreen = super.checkEdges();
        
        return this.isOffScreen;
    }

    Boolean hitsAsteroid() {
        for (Asteroid asteroid : this.asteroids) {
            if (asteroid.hit(this.position.x, this.position.y, this.radius)) {
                if (asteroid.radius > 20) {
                    asteroids.addAll(asteroid.breakup());
                }

                asteroids.remove(asteroid);

                return true;
            }
        }

        return false;
    }

    Boolean hitsUfo() {
        for (Ufo ufo : this.ufos) {
            if (ufo.hit(this.position.x, this.position.y, this.radius)) {
                ufos.remove(ufo);

                return true;
            }
        }

        return false;
    }
}