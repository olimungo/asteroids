import Patatoid from './patatoid';
import P5 from 'p5';
import Sprite from './sprite';

const SIDES_MIN = 8;
const SIDES_MAX = 20;
const RADIUS_MIN = 40;
const RADIUS_MAX = 120;
const ASTEROID_MIN_DISTANCE_TO_CENTER = 250;
const ASTEROID_MINIMAL_DIAMETER_BREAKUP = 50;

export default class Asteroid extends Patatoid {
    static radiusMin = RADIUS_MIN;
    static radiusMax = RADIUS_MAX;
    static sidesMin = SIDES_MIN;
    static sidesMax = SIDES_MAX;

    constructor(
        p5: P5,
        position?: P5.Vector,
        radius?: number,
        sides?: number,
        rotationStep?: number
    ) {
        if (!position) {
            position = new P5.Vector(p5.random(p5.width), p5.random(p5.height));

            // As the position is randomly chosen, make sure that the asteroid is not
            // placed too close from the center of the screen... where the ship will be.
            const middle = new P5.Vector(p5.width / 2, p5.height / 2);
            const distanceToCenter = P5.Vector.sub(position, middle);

            if (distanceToCenter.mag() < ASTEROID_MIN_DISTANCE_TO_CENTER) {
                position.setMag(ASTEROID_MIN_DISTANCE_TO_CENTER);
            }
        }

        if (!radius) {
            radius = p5.floor(
                p5.random(Asteroid.radiusMin, Asteroid.radiusMax)
            );
        }

        if (!sides) {
            sides = p5.floor(p5.random(Asteroid.sidesMin, Asteroid.sidesMax));
        }

        if (!rotationStep) {
            rotationStep = p5.map(p5.random(1), 0, 1, -0.01, 0.01);
        }

        super(p5, position, radius, sides, rotationStep);

        this.velocity = P5.Vector.random2D();
    }

    breakup(): Asteroid[] {
        if (this.diameter > ASTEROID_MINIMAL_DIAMETER_BREAKUP) {
            const countnewAsteroids =
                this.diameter > (RADIUS_MAX / 10) * 7 ? 3 : 2;

            const newAsteroids: Asteroid[] = [];

            for (let counter = 0; counter < countnewAsteroids; counter++) {
                newAsteroids.push(
                    new Asteroid(
                        this.p5,
                        this.position.copy(),
                        this.diameter / countnewAsteroids,
                        this.sides,
                        this.rotationStep
                    )
                );
            }

            return newAsteroids;
        }

        return [];
    }

    static setRadius(min: number, max: number) {
        this.radiusMin = min;
        this.radiusMax = max;
    }

    static setSides(min: number, max: number) {
        this.sidesMin = min;
        this.sidesMax = max;
    }
}
