public class Asteroid extends Fragment {
    Asteroid(PVector position, float radius, int sides, float rotationStep) {
        super(position.x, position.y, radius * 0.5, sides, rotationStep);
        this.velocity = PVector.random2D();
    }

    Asteroid() {
        super(random(width), random(height), floor(random(15, 50)), floor(random(5, 15)), map(random(1), 0, 1, -0.01, 0.01));
        this.velocity = PVector.random2D();

        // As the position is randomly chosen, make sure that the asteroid is not
        // placed to close from the center of the screen, where the ship will be.
        PVector middle = new PVector(width / 2, height / 2);
        PVector position = PVector.sub(this.position, middle);

        if (position.mag() < 250) {
            position.setMag(250);
        }

        position.add(middle);

        this.position = position;
    }

    ArrayList<Asteroid> breakup() {
        ArrayList<Asteroid> asteroids = new ArrayList<Asteroid>();

        asteroids.add(new Asteroid(this.position, this.radius, this.sides, this.rotationStep));
        asteroids.add(new Asteroid(this.position, this.radius, this.sides, this.rotationStep));

        return asteroids;
    }
}