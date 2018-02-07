public class SpritesManager {
    private Ship ship;
    private ArrayList<Laser> shipLasers = new ArrayList<Laser>();
    private ArrayList<Ufo> ufos;
    private ArrayList<Asteroid> asteroids;
    private int numUfosHit;
    private int numAsteroidsHit;
    private Boolean shipHit;
    private Boolean isPaused = false;

    SpritesManager() {
        this.ufos = new ArrayList<Ufo>();
        this.asteroids = new ArrayList<Asteroid>();
    }

    void update() {
        if (!this.isPaused) {
            if (this.ship != null) {
                this.ship.update();
            }

            for (Asteroid asteroid : this.asteroids) {
                asteroid.update();

                if (this.ship != null && this.ship.hits(asteroid)) {
                    println("ship hit");
                }
            } 

            for (int i = this.shipLasers.size() - 1; i >= 0; i--) {
                Laser laser = this.shipLasers.get(i);
                laser.update();

                if (laser.isOffScreen) {
                    this.shipLasers.remove(laser);
                }

                for(int j = this.asteroids.size() - 1; j >= 0; j--) {
                    Asteroid asteroid = this.asteroids.get(j);

                    if (laser.hits(asteroid)) {
                        this.shipLasers.remove(laser);
                        this.asteroids.remove(asteroid);
                        this.numAsteroidsHit++;

                        if (asteroid.radius > 20) {
                            this.asteroids.addAll(asteroid.breakup());
                        }
                    }
                }

                // if (laser.hitsUfo()) {
                //     this.lasers.remove(laser);
                //     this.gameManager.ufoHit();
                // }
            }

            // if (this.state == GameState.PLAYING) {
            //     if (this.ship.hitsAsteroid() || ship.hitsUfo()) {
            //         this.lifeLost();
            //     }

            //     for (Ufo ufo : this.ufos) {
            //         if (ufo.hitsShip()) {
            //             this.ship.explode();
            //             this.lifeLost();
            //             break;
            //         }
            //     }

            //     this.checkLevelFinished();
            //     this.checkScore();
            // }

            // if (this.timerUfo != 0) {
            //     this.checkTimerUfo();
            // }

            // if (this.state != GameState.HOMESCREEN) {
            //     this.ship.update();
            // }

            // for (Ufo ufo : this.ufos) {
            //     ufo.update();
            // }

            // if (this.homescreenTimer > 0) {
            //     this.checkTimerHomescreen();
            // }
        }
    }

    void draw() {
        pushStyle();
            fill(219, 233, 255);

            for (Asteroid asteroid : this.asteroids) {
                asteroid.draw();
            }

            if (this.ship != null) {
                this.ship.draw();
            }

            for (Laser laser : this.shipLasers) {
                laser.draw();
            }

            // if (this.state == GameState.PLAYING) {
            //     if (this.showNewLife) {
            //         this.helpers.showNewLife();
            //         this.checkTimerNewLife();
            //     }
            // }

            // for (Ufo ufo : this.ufos) {
            //     ufo.draw();
            // }
        popStyle();
    }

    int getAsteroidsCount() {
        return this.asteroids.size();
    }

    void createAsteroids(int num) {
        this.asteroids.clear();

        for (int i = 0; i < num; i++) {
            this.asteroids.add(new Asteroid());
        }
    }

    void createShip() {
        this.ship = new Ship();
    }

    void resetShip() {
        this.shipHit = false;
    }

    void resetAll() {
        this.numUfosHit = 0;
        this.numAsteroidsHit = 0;
        this.shipHit = false;
    }


    void keyPressed(int keyCode) {
        if (this.ship != null) {
            if (keyCode == LEFT) {
                this.ship.setRotation(-0.075);
            } else if (keyCode == RIGHT) {
                this.ship.setRotation(0.075);
            } else if (keyCode == UP) {
                this.ship.setBoost(true);
            } else if (keyCode == 32) { // SPACE
                this.shipLasers.add(new Laser(this.ship.position, this.ship.heading));
            } else if (keyCode == 80) { // P
                this.isPaused = !this.isPaused;
            }
        }
    }

    void keyReleased(int keyCode) {
        if (this.ship != null) {
            if (keyCode == UP) {
                this.ship.setBoost(false);
            } else if (keyCode == LEFT || keyCode == RIGHT) {
                this.ship.setRotation(0);
            }
        }
    }
}