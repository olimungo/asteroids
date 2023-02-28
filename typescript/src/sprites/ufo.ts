import P5 from 'p5';
import Colors from '../ui/colors';
import Interval from '../interval';
import Sprite from './sprite';
import Ship from './ship';
import Laser from './laser';

const UFO_WIDTH = 60;
const UFO_HEIHT = 25;
const UFO_VELOCITY = 2;
const CHANGE_HEADING_FREQUENCY = 6000;
const VARIABILITY_IN_HEADING = 3000;
const VARIABILITY_IN_SHOOTING = 1000;

export default class Ufo extends Sprite {
    private shape: P5.Graphics;
    private lasers: Laser[] = [];

    // Helps to define actions based on a time frequency
    private changeHeadingInterval: Interval;
    private shootInterval: Interval;

    constructor(
        p5: P5,
        position: P5.Vector,
        shootIntervalFrequency: number = 0
    ) {
        super(p5, position, (UFO_WIDTH + UFO_HEIHT) / 2);

        this.velocity = P5.Vector.random2D();
        this.velocity.setMag(UFO_VELOCITY);
        this.shape = this.generateShape(this.generateVertices());

        this.changeHeadingInterval = new Interval(
            p5.random(
                CHANGE_HEADING_FREQUENCY - VARIABILITY_IN_HEADING,
                CHANGE_HEADING_FREQUENCY + VARIABILITY_IN_HEADING
            )
        );

        if (shootIntervalFrequency > 0) {
            this.shootInterval = new Interval(
                p5.random(
                    shootIntervalFrequency - VARIABILITY_IN_SHOOTING,
                    shootIntervalFrequency + VARIABILITY_IN_SHOOTING
                )
            );
        }
    }

    update(ship: Ship | null) {
        if (this.changeHeadingInterval.isElapsed()) {
            this.velocity = P5.Vector.random2D();
            this.velocity.setMag(UFO_VELOCITY);
        }

        if (ship) {
            if (this.shootInterval && this.shootInterval.isElapsed()) {
                const target = P5.Vector.sub(ship.position, this.position);

                this.lasers.push(
                    new Laser(this.p5, this.position.copy(), target.heading())
                );
            }

            this.lasers = this.lasers.filter((laser) => {
                laser.update();
                return !laser.isOffScreen;
            });
        }

        super.update();
    }

    draw() {
        this.p5.push();

        this.p5.translate(
            this.position.x - UFO_WIDTH / 2,
            this.position.y - UFO_HEIHT / 2
        );

        this.p5.image(this.shape, 0, 0);

        this.p5.pop();

        for (let laser of this.lasers) {
            laser.draw();
        }
    }

    private generateVertices(): P5.Vector[] {
        const vertices: P5.Vector[] = [];

        const height1 = UFO_HEIHT / 4;
        const height2 = (UFO_HEIHT / 8) * 5;
        const width1 = UFO_WIDTH / 3;
        const width2 = UFO_WIDTH * 0.66;
        const width3 = (UFO_WIDTH / 10) * 6;
        const width4 = (UFO_WIDTH / 10) * 4;

        vertices.push(new P5.Vector(7, height2));
        vertices.push(new P5.Vector(width1, UFO_HEIHT - 1));
        vertices.push(new P5.Vector(width2, UFO_HEIHT - 1));
        vertices.push(new P5.Vector(UFO_WIDTH - 7, height2));
        vertices.push(new P5.Vector(7, height2));
        vertices.push(new P5.Vector(width1, height1));
        vertices.push(new P5.Vector(width2, height1));
        vertices.push(new P5.Vector(UFO_WIDTH - 7, height2));
        vertices.push(new P5.Vector(width2, height1));
        vertices.push(new P5.Vector(width3, 1));
        vertices.push(new P5.Vector(width4, 1));
        vertices.push(new P5.Vector(width1, height1));

        return vertices;
    }

    private generateShape(vertices: P5.Vector[]): P5.Graphics {
        const shape = this.p5.createGraphics(UFO_WIDTH, UFO_HEIHT);

        shape.noFill();

        shape.stroke(Colors.EDGE);
        shape.strokeWeight(1.3);

        shape.beginShape();

        for (let vertex of vertices) {
            shape.vertex(vertex.x, vertex.y);
        }

        shape.endShape(this.p5.CLOSE);

        return shape;
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
}
