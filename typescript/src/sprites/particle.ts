import P5 from 'p5';
import Sprite from './sprite';

export default class Particle extends Sprite {
    private alpha: number;

    constructor(p5: P5, position: P5.Vector, diameter: number) {
        super(p5, position, diameter);

        this.velocity = P5.Vector.random2D();
        this.alpha = p5.random(175, 230);
    }

    update(): boolean {
        super.update();

        this.alpha -= 3;

        if (this.alpha <= 50) {
            return false;
        }

        return true;
    }

    draw() {
        this.p5.push();

        this.p5.stroke(219, 233, 255, this.alpha);
        this.p5.noFill();

        // this.p5.fill(219, 233, 255, this.alpha);

        this.p5.ellipse(this.position.x, this.position.y, this.diameter);

        this.p5.pop();
    }
}
