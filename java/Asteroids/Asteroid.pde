public class Asteroid extends Patatoid {
    // static final int DIAMETER_MIN = 40;
    // static final int DIAMETER_MAX = 120;
    // static final int SIDES_MIN = 8;
    // static final int SIDES_MAX = 20;
    // static final int ASTEROID_MIN_DISTANCE_TO_CENTER = 250;
    // static final int ASTEROID_MINIMAL_DIAMETER_BREAKUP = 50;

    public Asteroid(
        PVector position,
        float diameter,
        PVector velocity,
        float rotationStep,
        int sides) {
        super(position, diameter, velocity, rotationStep, sides);
    }

    // Asteroid() {
    //     super(random(width), random(height), floor(random(20, 60)), floor(random(10, 15)), map(random(1), 0, 1, -0.01, 0.01));
    //     this.velocity = PVector.random2D();

    //     // As the position is randomly chosen, make sure that the asteroid is not
    //     // placed to close from the center of the screen, where the ship will be.
    //     PVector middle = new PVector(width / 2, height / 2);
    //     PVector position = PVector.sub(this.position, middle);

    //     if (position.mag() < 250) {
    //         position.setMag(250);
    //     }

    //     position.add(middle);

    //     this.position = position;
    // }

    // ArrayList<Asteroid> breakup() {
    //     ArrayList<Asteroid> asteroids = new ArrayList<Asteroid>();

    //     asteroids.add(new Asteroid(this.position, this.radius, this.sides, this.rotationStep));
    //     asteroids.add(new Asteroid(this.position, this.radius, this.sides, this.rotationStep));

    //     return asteroids;
    // }

    // Boolean hit(float x, float y, float radius) {
    //     float distance = dist(this.position.x, this.position.y, x, y);

    //     if (distance < this.radius + radius) {
    //         return true;
    //     }

    //     return false;
    // }
}