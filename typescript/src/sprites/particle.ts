import P5 from 'p5';
import Colors from '../ui/colors';
import Sprite from './sprite';

const POLYGON_SIDES = 5;
const HEALTH_DECREMENT = 5;
const MINIMAL_HEALTH = 50;

export default class Particle extends Sprite {
    private health: number;

    constructor(p5: P5, position: P5.Vector, diameter: number) {
        super(p5, position, diameter, P5.Vector.random2D());

        this.health = p5.random(175, 230);
    }

    update(): boolean {
        super.update();

        this.health -= HEALTH_DECREMENT;

        if (this.health <= MINIMAL_HEALTH) {
            return false;
        }

        return true;
    }

    draw() {
        this.p5.push();

        this.p5.stroke(Colors.EDGE);
        this.p5.noFill();

        this.polygon(this.position, this.diameter, POLYGON_SIDES);

        this.p5.pop();
    }

    private polygon(position: P5.Vector, radius: number, sides: number) {
        let angle = this.p5.TWO_PI / sides;

        this.p5.beginShape();

        for (let a = 0; a < this.p5.TWO_PI; a += angle) {
            let sx = position.x + this.p5.cos(a) * radius;
            let sy = position.y + this.p5.sin(a) * radius;

            this.p5.vertex(sx, sy);
        }

        this.p5.endShape(this.p5.CLOSE);
    }
}
