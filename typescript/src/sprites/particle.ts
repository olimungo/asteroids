import P5 from 'p5';
import Sprite from './sprite';

export default class Particle extends Sprite {
    private health: number;

    constructor(p5: P5, position: P5.Vector, diameter: number) {
        super(p5, position, diameter);

        this.velocity = P5.Vector.random2D();
        this.health = p5.random(175, 230);
    }

    update(): boolean {
        super.update();

        this.health -= 3;

        if (this.health <= 50) {
            return false;
        }

        return true;
    }

    draw() {
        this.p5.push();

        this.p5.stroke(219, 233, 255);
        this.p5.noFill();

        this.polygon(this.position, this.diameter, 5);

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
