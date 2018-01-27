public class Laser extends Sprite {
    Boolean isOffScreen = false;

    private ArrayList<Asteroid> asteroids;

    Laser(PVector position, float angle, ArrayList<Asteroid> asteroids) {
        super(position.x, position.y, 1);
        this.velocity = PVector.fromAngle(angle).mult(10);
        this.asteroids = asteroids;
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

    Boolean hitsAsteroids() {
        Asteroid asteroid = super.hits(this.asteroids);

        if (asteroid != null) {
            if (asteroid.radius > 15) {
                asteroids.addAll(asteroid.breakup());
            }

            asteroids.remove(asteroid);
        }

        return asteroid != null;
    }
}