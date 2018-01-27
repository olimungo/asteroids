public class Ship extends Sprite {
    private float heading = -PI / 2;
    private float rotation = 0;
    private Boolean isBoosting = false;
    private ArrayList<Laser> lasers = new ArrayList<Laser>();
    private ArrayList<Asteroid> asteroids;
    private ArrayList<Fragment> fragments = new ArrayList<Fragment>();
    private GameManager gameManager;

    Ship(GameManager gameManager, ArrayList<Asteroid> asteroids) {
        super(width / 2, height / 2, 18);

        this.gameManager = gameManager;
        this.asteroids = asteroids;
    }
  
    @Override
    void update() {
        this.turn();

        if (this.isBoosting) {
            this.boost();
        }

        super.update();
        this.velocity.mult(0.995);
    }

    @Override
    void draw() {
        if (this.fragments.size() == 0) {
            for (int i = this.lasers.size() - 1; i >= 0; i--) {
                Laser laser = this.lasers.get(i);
                laser.update();
                laser.draw();

                Boolean asteroidHit = laser.hitsAsteroid();

                if (asteroidHit || laser.isOffScreen) {
                    this.lasers.remove(laser);

                    if (asteroidHit) {
                        this.gameManager.asteroidHit();
                    }
                }
            }

            pushStyle();
            pushMatrix();
                noFill();
                strokeWeight(1.4);

                translate(this.position.x, this.position.y);

                rotate(this.heading + PI / 2);

                float alpha1 = random(170, 255);
                float alpha2 = random(170, 255);
                float alpha3 = random(170, 255);

                float smallerRadius = this.radius / 5 * 3.5;

                stroke(219, 233, 255, alpha1);
                bezier(-smallerRadius, this.radius, 0, smallerRadius, 0, smallerRadius, smallerRadius, this.radius);
                stroke(219, 233, 255, 30);
                ellipse(smallerRadius, this.radius, 2, 2);
                stroke(219, 233, 255, alpha2);
                line(smallerRadius, this.radius, 0, -this.radius);
                stroke(219, 233, 255, 30);
                ellipse(0, -this.radius, 2, 2);
                stroke(219, 233, 255, alpha3);
                line(0, -this.radius, -smallerRadius, this.radius);
                stroke(219, 233, 255, 30);
                ellipse(-smallerRadius, this.radius, 2, 2);

            popMatrix();
            popStyle();
        } else {
            for (Fragment fragment : this.fragments) {
                fragment.update();
                fragment.draw();
            }
        }
    }

    void setRotation(float angle) {
        this.rotation = angle;
    }

    void setBoost(Boolean value) {
        this.isBoosting = value;
    }

    void shoot() {
        this.lasers.add(new Laser(this.position, this.heading, this.asteroids));
    }

    Boolean hitsAsteroid() {
        if (this.fragments.size() == 0) {
            Asteroid asteroid = super.hits(this.asteroids);

            if (asteroid != null) {
                Fragment fragment = new Fragment(this.position.x, this.position.y, this.radius * 0.8, 5, 0.1);
                fragment.velocity = PVector.random2D();
                this.fragments.add(fragment);
                fragment = new Fragment(this.position.x, this.position.y, this.radius * 0.8, 3, -0.1);
                fragment.velocity = PVector.random2D();
                this.fragments.add(fragment);
                fragment = new Fragment(this.position.x, this.position.y, this.radius * 0.5, 3, 0.2);
                fragment.velocity = PVector.random2D();
                this.fragments.add(fragment);
            }

            return asteroid != null;
        }

        return true;
    }

    private void boost() {
        PVector force = PVector.fromAngle(this.heading);
        force.limit(.15);
        this.velocity.add(force);
    }

    private void turn() {
        this.heading += this.rotation;
    }
}
