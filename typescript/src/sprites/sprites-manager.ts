import P5 from 'p5';
import Interval from '../utils/interval';
import Explosion from './explosion';
import Potatoid from './potatoid';
import Ship from './ship';
import Ufo from './ufo';
import { random_limit } from '../utils/random';

const DIAMETER_MIN = 40;
const DIAMETER_MAX = 120;
const SIDES_MIN = 8;
const SIDES_MAX = 20;
const ASTEROIDS_VELOCITY_LIMIT = 1.0;
const ASTEROIDS_VELOCITY_MIN_VALUE = 0.2;

export default class SpritesManager {
    private p5: P5;
    private asteroids: Potatoid[] = [];
    private ship: Ship | null;
    private shipFragments: Potatoid[] = [];
    private ufos: Ufo[] = [];
    private explosions: Explosion[] = [];
    private createUfoInterval = new Interval();
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

        let newAsteroids: Potatoid[] = [];

        this.asteroids = this.asteroids.filter((asteroid) => {
            asteroid.update();

            if (this.ship) {
                if (this.ship.lasersHit(asteroid)) {
                    const asteroids = asteroid.breakUp();

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
                    this.shipFragments = this.ship.breakUp();
                    this.ship = null;
                }
            }

            return true;
        });

        this.asteroids = [...this.asteroids, ...newAsteroids];

        this.ufos = this.ufos.filter((ufo) => {
            ufo.update(this.ship?.position.copy());

            if (this.ship) {
                if (this.ship.lasersHit(ufo)) {
                    this.countUfosHit++;

                    this.explosions.push(
                        new Explosion(this.p5, ufo.position.copy())
                    );

                    return false;
                }

                if (this.ship.collideWith(ufo) || ufo.lasersHit(this.ship)) {
                    this.shipFragments = this.ship.breakUp();
                    this.ship = null;
                }
            }

            return true;
        });

        if (this.createUfoInterval.isElapsed()) {
            this.createUfo(this.ufoShootFrequency);
        }

        this.explosions = this.explosions.filter((explosion) =>
            explosion.update()
        );
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

        this.ship = new Ship(
            this.p5,
            new P5.Vector(this.p5.width / 2, this.p5.height / 2),
            false
        );

        this.createAsteroids(countAsteroids);

        this.createUfoInterval.set(ufoCreateFrequency);
    }

    stopLevel() {
        this.ufos = [];
        this.createUfoInterval.cancel();
        this.ufoShootFrequency = 0;
        this.countAsteroidsHit = 0;
        this.countUfosHit = 0;
    }

    createAsteroids(count: number) {
        this.asteroids = [];

        for (let counter = 0; counter < count; counter++) {
            const radius = this.p5.random(
                this.p5.height / 2,
                (this.p5.height / 2) * 1.3
            );

            const angle = this.p5.map(counter, 0, count, 0, this.p5.TWO_PI);
            const x = radius * this.p5.cos(angle);
            const y = radius * this.p5.sin(angle);
            const position = new P5.Vector(x, y);

            position.add(this.p5.width / 2, this.p5.height / 2);

            const velocity = random_limit(
                ASTEROIDS_VELOCITY_LIMIT,
                ASTEROIDS_VELOCITY_MIN_VALUE
            );

            const diameter = this.p5.random(DIAMETER_MIN, DIAMETER_MAX);

            const rotationStep = this.p5.map(
                this.p5.random(1),
                0,
                1,
                -0.01,
                0.01
            );

            const sides = this.p5.floor(this.p5.random(SIDES_MIN, SIDES_MAX));

            this.asteroids.push(
                new Potatoid(
                    this.p5,
                    position,
                    diameter,
                    velocity,
                    rotationStep,
                    sides
                )
            );
        }
    }

    createUfo(ufoShootFrequency: number) {
        this.ufos.push(new Ufo(this.p5, ufoShootFrequency));
    }

    getAsteroidsCount(): number {
        return this.asteroids.length;
    }

    keyPressed(keyCode: number) {
        if (this.ship) {
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
        if (this.ship) {
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
    }

    pause() {
        this.createUfoInterval.pause();

        for (const ufo of this.ufos) {
            ufo.pause();
        }
    }

    unpause() {
        this.createUfoInterval.unpause();

        for (const ufo of this.ufos) {
            ufo.unpause();
        }
    }
}
