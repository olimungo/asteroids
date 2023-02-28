import P5 from 'p5';
import Interval from '../interval';
import Asteroid from './asteroid';
import Explosion from './explosion';
import Patatoid from './patatoid';
import Ship from './ship';
import Ufo from './ufo';

const VARIABILITY_IN_CREATING_UFOS = 5000;

export default class SpritesManager {
    private p5: P5;
    private asteroids: Asteroid[] = [];
    private ship: Ship | null;
    private shipFragments: Patatoid[] = [];
    private ufos: Ufo[] = [];
    private explosions: Explosion[] = [];
    private createUfoInterval: Interval | null;
    private ufoShootFrequency: number = 0;

    countAsteroidsHit = 0;
    countUfosHit = 0;

    constructor(p5: P5) {
        this.p5 = p5;
    }

    update() {
        if (this.ship) {
            this.ship.update();
        } else {
            for (const shipFragment of this.shipFragments) {
                shipFragment.update();
            }
        }

        let newAsteroids: Asteroid[] = [];

        this.asteroids = this.asteroids.filter((asteroid) => {
            asteroid.update();

            if (this.ship) {
                if (this.ship.lasersHit(asteroid)) {
                    const asteroids = asteroid.breakup();

                    if (asteroids.length > 0) {
                        newAsteroids = [...newAsteroids, ...asteroids];
                    } else {
                        this.explosions.push(
                            new Explosion(this.p5, asteroid.position.copy())
                        );
                    }

                    this.countAsteroidsHit++;

                    return false;
                }

                if (this.ship.collideWith(asteroid)) {
                    this.shipFragments = this.ship.breakup();
                    this.ship = null;
                }
            }

            return true;
        });

        this.asteroids = [...this.asteroids, ...newAsteroids];

        this.ufos = this.ufos.filter((ufo) => {
            ufo.update(this.ship);

            if (this.ship) {
                if (this.ship.lasersHit(ufo)) {
                    this.countUfosHit++;

                    this.explosions.push(
                        new Explosion(this.p5, ufo.position.copy())
                    );

                    return false;
                }

                if (this.ship.collideWith(ufo) || ufo.lasersHit(this.ship)) {
                    this.shipFragments = this.ship.breakup();
                    this.ship = null;
                }
            }

            return true;
        });

        if (this.createUfoInterval?.isElapsed()) {
            this.createUfo(this.ufoShootFrequency);
        }

        for (const shipFragment of this.shipFragments) {
            shipFragment.update();
        }

        this.explosions = this.explosions.filter((explosion) => {
            explosion.update();

            return !explosion.isOver();
        });
    }

    draw() {
        for (const explosion of this.explosions) {
            explosion.draw();
        }

        if (this.ship) {
            this.ship.draw();
        }

        for (const asteroid of this.asteroids) {
            asteroid.draw();
        }

        for (const ufo of this.ufos) {
            ufo.draw();
        }

        for (const shipFragment of this.shipFragments) {
            shipFragment.draw();
        }
    }

    startLevel(
        countAsteroids: number,
        ufoCreateFrequency: number,
        ufoShootFrequency: number
    ) {
        this.ufoShootFrequency = ufoShootFrequency;

        this.reset();

        this.ship = new Ship(this.p5);
        this.createAsteroids(countAsteroids);

        this.createUfoInterval = new Interval(
            this.p5.random(
                ufoCreateFrequency - VARIABILITY_IN_CREATING_UFOS,
                ufoCreateFrequency + VARIABILITY_IN_CREATING_UFOS
            )
        );
    }

    stopLevel() {
        this.createUfoInterval = null;
        this.ufoShootFrequency = 0;
        this.ufos = [];
    }

    createAsteroids(count: number) {
        this.asteroids = [];

        for (let counter = 0; counter < count; counter++) {
            this.asteroids.push(new Asteroid(this.p5));
        }
    }

    createUfo(ufoShootFrequency: number) {
        const randomSide = this.p5.floor(this.p5.random(4));
        const vector = P5.Vector.random2D();

        switch (randomSide) {
            case 1:
                vector.x += this.p5.width;
                break;
            case 2:
                vector.x -= this.p5.width;
                break;
            case 3:
                vector.y += this.p5.height;
                break;
            case 4:
                vector.y -= this.p5.height;
                break;
        }

        const ufo = new Ufo(this.p5, vector, ufoShootFrequency);
        this.ufos.push(ufo);
    }

    getAsteroidsCount(): number {
        return this.asteroids.length;
    }

    getUfosCount() {
        return this.ufos.length;
    }

    keyPressed(keyCode: number) {
        if (this.ship !== null) {
            switch (keyCode) {
                case this.p5.LEFT_ARROW:
                    this.ship.setRotation(-0.1);
                    break;
                case this.p5.RIGHT_ARROW:
                    this.ship.setRotation(0.1);
                    break;
                case this.p5.UP_ARROW:
                    this.ship.setBoost(true);
                    break;
                case 32: // SPACE
                    this.ship.shoot();
                    break;
            }
        }
    }

    keyReleased(keyCode: number) {
        if (this.ship != null) {
            if (keyCode == this.p5.UP_ARROW) {
                this.ship.setBoost(false);
            } else if (
                keyCode == this.p5.LEFT_ARROW ||
                keyCode == this.p5.RIGHT_ARROW
            ) {
                this.ship.setRotation(0);
            }
        }
    }

    shipHit(): boolean {
        return this.ship === null;
    }

    reset() {
        this.asteroids = [];
        this.shipFragments = [];
        this.ufos = [];
        this.countAsteroidsHit = 0;
        this.countUfosHit = 0;
    }
}
