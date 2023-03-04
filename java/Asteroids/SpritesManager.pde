public class SpritesManager {
    private final static int VARIABILITY_IN_CREATING_UFOS = 5000;
    private final static int ASTEROID_MIN_DISTANCE_TO_CENTER = 250;
    private final static int DIAMETER_MIN = 40;
    private final static int DIAMETER_MAX = 120;
    private final static int SIDES_MIN = 8;
    private final static int SIDES_MAX = 20;

    private ArrayList<Patatoid> asteroids = new ArrayList<Patatoid>();
    private Ship ship;
    private ArrayList<Patatoid> shipFragments = new ArrayList<Patatoid>();
    private ArrayList<Ufo> ufos = new ArrayList<Ufo>();
    private ArrayList<Explosion> explosions = new ArrayList<Explosion>();
    private Interval createUfoInterval;
    private int ufoShootFrequency = 0;

    int countAsteroidsHit;
    int countUfosHit;

    void update() {
        if (this.ship != null) {
            this.ship.update();
        } else {
            for (Patatoid shipFragment : this.shipFragments) {
                shipFragment.update();
            }
        }

        for (int asteroidIndex = this.asteroids.size() - 1; asteroidIndex >= 0; asteroidIndex--) {
            Patatoid asteroid = this.asteroids.get(asteroidIndex);
            asteroid.update();

            if (this.ship != null) {
                if (this.ship.lasersHit(asteroid)) {
                    ArrayList<Patatoid> newAsteroids = asteroid.breakUp();

                    if (newAsteroids.size() > 0) {
                        for (Patatoid newAsteroid : newAsteroids) {
                            this.asteroids.add(newAsteroid);
                        }
                    } else {
                        this.explosions.add(new Explosion(asteroid.position.copy()));
                    }

                    this.countAsteroidsHit++;

                    this.asteroids.remove(asteroidIndex);
                }

                if (this.ship.collideWith(asteroid)) {
                    this.shipFragments = this.ship.breakUp();
                    this.ship = null;
                }
            }
        }

        for (int ufoIndex = this.ufos.size() - 1; ufoIndex >= 0; ufoIndex--) {
            Ufo ufo = this.ufos.get(ufoIndex);

            PVector position = null;

            if (this.ship != null) {
                position = this.ship.position.copy();

                if (this.ship.lasersHit(ufo)) {
                    this.countUfosHit++;

                    this.explosions.add(
                        new Explosion(ufo.position.copy())
                    );

                    this.ufos.remove(ufoIndex);
                }

                if (this.ship.collideWith(ufo) || ufo.lasersHit(this.ship)) {
                    this.shipFragments = this.ship.breakUp();
                    this.ship = null;
                }                
            }

            ufo.update(position);
        }

        if (this.createUfoInterval != null && this.createUfoInterval.isElapsed()) {
            this.createUfo(this.ufoShootFrequency);
        }

        for (int explosionIndex = this.explosions.size() - 1; explosionIndex >= 0 ; explosionIndex--) {
            Explosion explosion = this.explosions.get(explosionIndex);

            if (!explosion.update()) {
                this.explosions.remove(explosionIndex);
            }
        }
    }

    void draw() {
        for (Explosion explosion : this.explosions) {
            explosion.draw();
        }

        if (this.ship != null) {
            this.ship.draw();
        }

        for (Patatoid asteroid : this.asteroids) {
            asteroid.draw();
        }

        for (Ufo ufo : this.ufos) {
            ufo.draw();
        }

        for (Patatoid shipFragment : this.shipFragments) {
            shipFragment.draw();
        }
    }

    void startLevel(
        int countAsteroids,
        int ufoCreateFrequency,
        int ufoShootFrequency
    ) {
        this.ufoShootFrequency = ufoShootFrequency;

        this.reset();

        this.ship = new Ship(
            new PVector(width / 2, height / 2),
            false);

        this.createAsteroids(countAsteroids);

        this.createUfoInterval = new Interval(
            int(random(
                ufoCreateFrequency - VARIABILITY_IN_CREATING_UFOS,
                ufoCreateFrequency + VARIABILITY_IN_CREATING_UFOS
            ))
        );
    }

    void stopLevel() {
        this.createUfoInterval = null;
        this.ufoShootFrequency = 0;
        this.ufos.clear();
    }

    void createAsteroids(int count) {
        this.asteroids = new ArrayList<Patatoid>();

        for (int counter = 0; counter < count; counter++) {
            float radius = random(
                height / 2,
                (height / 2) * 1.3
            );

            float angle = map(counter, 0, count, 0, TWO_PI);
            float x = radius * cos(angle);
            float y = radius * sin(angle);
            PVector position = new PVector(x, y);

            position.add(width / 2, height / 2);

            float diameter = random(this.DIAMETER_MIN, this.DIAMETER_MAX);

            float rotationStep = map(
                random(1),
                0,
                1,
                -0.01,
                0.01
            );

            int sides = floor(random(this.SIDES_MIN, this.SIDES_MAX));

            this.asteroids.add(
                new Patatoid(
                    position,
                    diameter,
                    PVector.random2D(),
                    rotationStep,
                    sides
                )
            );
        }



        // for (int counter = 0; counter < count; counter++) {
        //     PVector position = new PVector(
        //         random(width),
        //         random(height)
        //     );

        //     // As the position is randomly chosen, make sure that the asteroid is not
        //     // placed too close from the center of the screen... where the ship will be.
        //     PVector middle = new PVector(width / 2, height / 2);
        //     PVector distanceToCenter = PVector.sub(position, middle);

        //     if (distanceToCenter.mag() < ASTEROID_MIN_DISTANCE_TO_CENTER) {
        //         position.setMag(ASTEROID_MIN_DISTANCE_TO_CENTER);
        //     }

        //     float diameter = random(DIAMETER_MIN, DIAMETER_MAX);
        //     float rotationStep = map(
        //         random(1),
        //         0,
        //         1,
        //         -0.01,
        //         0.01
        //     );
        //     int sides = floor(random(SIDES_MIN, SIDES_MAX));

        //     this.asteroids.add(
        //         new Patatoid(
        //             position,
        //             diameter,
        //             PVector.random2D(),
        //             rotationStep,
        //             sides
        //         )
        //     );
        // }
    }

    void createUfo(int ufoShootFrequency) {
        int randomSide = int(floor(random(4)));
        PVector vector = PVector.random2D();

        switch (randomSide) {
            case 1:
                vector.x += width;
                break;
            case 2:
                vector.x -= width;
                break;
            case 3:
                vector.y += height;
                break;
            case 4:
                vector.y -= height;
                break;
        }

        Ufo ufo = new Ufo(vector, ufoShootFrequency);
        this.ufos.add(ufo);
    }

    int getAsteroidsCount() {
        return this.asteroids.size();
    }

    int getUfosCount() {
        return this.ufos.size();
    }

    void keyPressed(int keyCode) {
        if (this.ship != null) {
            switch (keyCode) {
                case 37: // LEFT ARROW
                    ship.setRotation(-0.1);
                    break;
                case 39: // RIGHT ARROW
                    ship.setRotation(0.1);
                    break;
                case 38: // UP ARROW
                    ship.setBoost(true);
                    break;
                case 32: // SPACE
                    ship.shoot();
                    break;
            }
        }
    }

    void keyReleased(int keyCode) {
        if (this.ship != null) {
            if (keyCode == 38) { // UP ARROW
                this.ship.setBoost(false);
            } else if ( keyCode == 37 || keyCode == 39) {
                // LEFT ARROW OR RIGHT ARROW
                this.ship.setRotation(0);
            }
        }
    }

    Boolean shipHit() {
        return this.ship == null;
    }

    void reset() {
        this.asteroids.clear();
        this.shipFragments.clear();
        this.ufos.clear();
        this.countAsteroidsHit = 0;
        this.countUfosHit = 0;
    }

    void pause() {
        if (this.createUfoInterval != null) {
            this.createUfoInterval.pause();
        }

        for (Ufo ufo : this.ufos) {
            ufo.pause();
        }
    }

    void unpause() {
        if (this.createUfoInterval != null) {
            this.createUfoInterval.unpause();
        }

        for (Ufo ufo : this.ufos) {
            ufo.unpause();
        }
    }
}