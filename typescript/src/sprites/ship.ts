import P5 from 'p5';
import Colors from '../ui/colors';
import Patatoid from './patatoid';
import Laser from './laser';
import Sprite from './sprite';
import Interval from '../interval';

const SHIP_SHELL_SIZE = 36;

export default class Ship extends Sprite {
    heading: number;
    isBoosting = false;
    fillShip: boolean;

    private lasers: Laser[] = [];
    private boosterFlamesInterval: Interval;
    private switchFlames = false;

    constructor(p5: P5, position?: P5.Vector, fillShip?: boolean) {
        position = position || new P5.Vector(p5.width / 2, p5.height / 2);

        super(p5, position, SHIP_SHELL_SIZE);

        this.fillShip = fillShip || false;
        this.heading = -p5.PI / 2;

        this.boosterFlamesInterval = new Interval(100);
    }

    update() {
        this.heading += this.rotation;

        if (this.isBoosting) {
            const force = P5.Vector.fromAngle(this.heading);
            force.limit(0.15);

            this.velocity.add(force);
        }

        this.velocity.limit(10);

        super.update();

        this.velocity.mult(0.995);

        this.lasers = this.lasers.filter((laser) => {
            laser.update();
            return !laser.isOffScreen;
        });
    }

    draw() {
        for (let laser of this.lasers) {
            laser.draw();
        }

        this.p5.push();

        this.p5.translate(this.position.x, this.position.y);
        this.p5.rotate(this.heading + this.p5.PI / 2);

        const smallerRadius = (this.diameter / 10) * 3.5;

        this.p5.beginShape();

        if (this.fillShip) {
            this.p5.fill(Colors.EDGE);
        } else {
            this.p5.fill(Colors.BACKGROUND);
        }

        this.p5.stroke(Colors.EDGE);
        this.p5.strokeWeight(1.4);

        this.p5.vertex(smallerRadius, this.diameter / 2);
        this.p5.vertex(0, -this.diameter / 2);
        this.p5.vertex(-smallerRadius, this.diameter / 2);

        this.p5.bezierVertex(
            0,
            smallerRadius,
            0,
            smallerRadius,
            smallerRadius,
            this.diameter / 2
        );

        this.p5.endShape();

        if (this.isBoosting) {
            if (this.boosterFlamesInterval.isElapsed()) {
                this.switchFlames = !this.switchFlames;
            }

            this.drawBoosterFlames();
        }

        this.p5.pop();
    }

    setRotation(angle: number) {
        this.rotation = angle;
    }

    setBoost(value: boolean) {
        this.isBoosting = value;
    }

    shoot() {
        this.lasers.push(
            new Laser(this.p5, this.position.copy(), this.heading)
        );
    }

    lasersHit(sprite: Sprite): boolean {
        let laserIndex: number = -1;

        for (let [index, laser] of this.lasers.entries()) {
            if (laser.collideWith(sprite)) {
                laserIndex = index;
                break;
            }
        }

        if (laserIndex > -1) {
            this.lasers.splice(laserIndex, 1);
            return true;
        }

        return false;
    }

    breakup(): Patatoid[] {
        const patatoids: Patatoid[] = [];

        for (let index of [
            [0.9, 5, 0.05],
            [0.9, 6, -0.08],
            [0.9, 3, 0.15],
            [0.7, 5, -0.1],
        ]) {
            let patatoid = new Patatoid(
                this.p5,
                this.position.copy(),
                this.diameter * index[0],
                index[1],
                index[2]
            );

            patatoid.velocity = P5.Vector.random2D();

            patatoids.push(patatoid);
        }

        return patatoids;
    }

    private drawBoosterFlames() {
        const smallerRadius = (this.diameter / 10) * 3.5;
        const marginX = this.diameter / 9;
        const marginY = this.diameter / 3;

        if (this.switchFlames) {
            this.p5.line(
                smallerRadius - marginX,
                this.diameter / 2,
                0,
                this.diameter
            );

            this.p5.line(
                -smallerRadius + marginX,
                this.diameter / 2,
                0,
                this.diameter
            );
        } else {
            this.p5.line(
                smallerRadius - marginX,
                this.diameter / 2,
                0,
                this.diameter - marginY
            );

            this.p5.line(
                -smallerRadius + marginX,
                this.diameter / 2,
                0,
                this.diameter - marginY
            );
        }
    }
}
