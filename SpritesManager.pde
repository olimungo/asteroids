public class SpritesManager {
    private Ship ship;
    private ArrayList<Laser> shipLasers = new ArrayList<Laser>();
    private ArrayList<Ufo> ufos;
    private ArrayList<Laser> ufosLasers = new ArrayList<Laser>();
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

            for (Ufo ufo : this.ufos) {
                ufo.update();
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

            for (int i = this.ufosLasers.size() - 1; i >= 0; i--) {
                Laser laser = this.ufosLasers.get(i);
                laser.update();

                if (laser.isOffScreen) {
                    this.ufosLasers.remove(laser);
                }
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

            for (Ufo ufo : this.ufos) {
                ufo.draw();
            }

            for (Laser laser : this.shipLasers) {
                laser.draw();
            }

            for (Laser laser : this.ufosLasers) {
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

    void createUfos(int num) {
        for (int i = 0; i < num; i++) {
            int randomSide = floor(random(4));
            PVector vector = PVector.random2D();

            switch(randomSide) {
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

            Ufo ufo = new Ufo(vector.x, vector.y);
            this.ufos.add(ufo);

            if (this.ship != null) {
                this.timerUfoShoot(ufo);
            }
        }
    }

    void resetUfos() {
        this.ufos.clear();
        this.ufosLasers.clear();
    }

    void resetAll() {
        this.numUfosHit = 0;
        this.numAsteroidsHit = 0;
        this.shipHit = false;
        this.ship = null;
        this.asteroids.clear();
        this.ufos.clear();
        this.shipLasers.clear();
        this.ufosLasers.clear();
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

    private void timerUfoShoot(final Ufo ufo) {
        Timer timer = new Timer();
        timer.schedule(new TimerTask() {
            @Override
            public void run() {
                PVector target = PVector.sub(ship.position, ufo.position);
                PVector middle = new PVector(ufo.ufoWidth / 2, ufo.ufoHeight / 2);
                PVector position = PVector.add(ufo.position, middle);

                ufosLasers.add(new Laser(position, target.heading()));
            }
        }, floor(random(2000, 2500)));
    }
}