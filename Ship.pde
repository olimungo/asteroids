public class Ship extends Sprite {
    private float heading = -PI / 2;
    private float rotation = 0;
    private Boolean isBoosting = false;
    private ArrayList<Fragment> fragments = new ArrayList<Fragment>();
    private ShipShell shipShell;
    // private ArrayList<Laser> lasers = new ArrayList<Laser>();
    // private ArrayList<Asteroid> asteroids;
    // private ArrayList<Ufo> ufos;
    // private GameManager gameManager;

    // Ship(GameManager gameManager, ArrayList<Asteroid> asteroids, ArrayList<Ufo> ufos) {
    Ship() {
        super(width / 2, height / 2, 18);

        // this.gameManager = gameManager;
        // this.asteroids = asteroids;
        // this.ufos = ufos;
        this.shipShell = new ShipShell(this.position.x, this.position.y);
    }
  
    @Override
    void update() {
        if (this.fragments.size() == 0) {
            this.turn();

            if (this.isBoosting) {
                this.boost();
            }

            // for (int i = this.lasers.size() - 1; i >= 0; i--) {
            //     Laser laser = this.lasers.get(i);
            //     laser.update();

            //     if (laser.isOffScreen) {
            //         this.lasers.remove(laser);
            //     }

            //     if (laser.hitsAsteroid()) {
            //         this.lasers.remove(laser);
            //         this.gameManager.asteroidHit();
            //     }

            //     if (laser.hitsUfo()) {
            //         this.lasers.remove(laser);
            //         this.gameManager.ufoHit();
            //     }
            // }

            this.shipShell.heading = this.heading;
            this.shipShell.position = this.position;

            this.velocity.limit(10);

            super.update();
            this.velocity.mult(0.995);
        } else {
            for (Fragment fragment : this.fragments) {
                fragment.update();
            }
        }
    }

    @Override
    void draw() {
        if (this.fragments.size() == 0) {
            // for (Laser laser : this.lasers) {
            //     laser.draw();
            // }

            this.shipShell.draw();
        } else {
            for (Fragment fragment : this.fragments) {
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
        // this.lasers.add(new Laser(this.position, this.heading, this.asteroids, this.ufos));
    }

    void explode() {
        if (this.fragments.size() == 0) {
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
    }

    @Override
    Boolean hits(Sprite sprite) {
        if (this.fragments.size() == 0) {
            if (super.hits(sprite)) {
                this.explode();
                return true;
            }
        }

        return false;
    }

    Boolean hitsUfo() {
        // if (this.fragments.size() == 0) {
        //     for (Ufo ufo : this.ufos) {
        //         if (ufo.hit(this.position.x, this.position.y, this.radius)) {
        //             Fragment fragment = new Fragment(this.position.x, this.position.y, this.radius * 0.8, 5, 0.1);
        //             fragment.velocity = PVector.random2D();
        //             this.fragments.add(fragment);
        //             fragment = new Fragment(this.position.x, this.position.y, this.radius * 0.8, 3, -0.1);
        //             fragment.velocity = PVector.random2D();
        //             this.fragments.add(fragment);
        //             fragment = new Fragment(this.position.x, this.position.y, this.radius * 0.5, 3, 0.2);
        //             fragment.velocity = PVector.random2D();
        //             this.fragments.add(fragment);

        //             return true;
        //         }
        //     }
        // }

        return false;
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
